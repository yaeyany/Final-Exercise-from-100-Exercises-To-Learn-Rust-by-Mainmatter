use anyhow::Ok;
use sqlx::{PgPool, QueryBuilder};

use crate::tickets::*;

pub struct TicketsDB {
    database: PgPool
}

impl TicketsDB {
    // Initialize new TicketDB with a given database URL ──────────────────────────────────────────────────
    pub async fn new(database_url: &str) -> Result<Self, anyhow::Error> {
        let db = TicketsDB { 
            database: sqlx::PgPool::connect(database_url).await?,
        };
        Ok(db)
    }
    // Reference self db ──────────────────────────────────────────────────
    pub fn database(&self) -> &PgPool {
        &self.database
    }
    // dd a ticket to the database ──────────────────────────────────────────────────
    pub async fn add_ticket(
        &self,
        title: TicketTitle,
        description: Option<TicketDescription>,
    ) -> Result<TicketId, anyhow::Error> {
        let title = title.into_inner();
        let description = description.map(TicketDescription::into_inner);

        let query = sqlx::query!(
            "INSERT INTO tickets (title, description, priority, status)
            VALUES ($1, $2, 'medium', 'new')
            RETURNING id",
            title,
            description
        )
        .fetch_one(&self.database)
        .await?;

        Ok(TicketId::try_from(query.id)?)
    }
    // Retrieve ticket details and convert into a ticket ──────────────────────────────────────────────────
    pub async fn get_ticket(&self, ticket_id: TicketId) -> Result<Ticket, anyhow::Error> {
        let id = ticket_id.into_inner();
        let row = sqlx::query!(
            "SELECT * FROM tickets WHERE id = $1",
            id
        )
        .fetch_one(&self.database)
        .await?;

        let ticket = Ticket::from_parts(
            row.id.try_into()?,
            row.title.try_into()?,
            row.description
                .map(|d| d.try_into())
                .transpose()?,
            row.priority.try_into()?,
            row.status.try_into()?,
        );
        Ok(ticket)
    }
    //Patch a ticket
    pub async fn patch_ticket(&self, ticket_id: TicketId, title: Option<TicketTitle>, description: Option<TicketDescription>, priority: Option<TicketPriority>, status: Option<TicketStatus>) -> Result<(), anyhow::Error> {
        let mut query = QueryBuilder::new("UPDATE tickets SET ");
        let mut first = true;

        if let Some(title) = title {
            if !first {
                query.push(", ");
            }
            first = false;

            query.push("title = ");
            query.push_bind(title.into_inner());
        }

        if let Some(description) = description {
            if !first {
                query.push(", ");
            }
            first = false;

            query.push("description = ");
            query.push_bind(description.into_inner());
        }

        if let Some(priority) = priority {
            if !first {
                query.push(", ");
            }
            first = false;

            query.push("priority = ");
            query.push_bind(priority.into_inner());
        }

        if let Some(status) = status {
            if !first {
                query.push(", ");
            }

            query.push("status = ");
            query.push_bind(status.into_inner());
        }

        query.push(" WHERE id = ");
        query.push_bind(ticket_id.into_inner());

        query.build()
            .execute(&self.database)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{database::TicketsDB, tickets::*};
    // Test connection to the psql using sqlx ──────────────────────────────────────────────────
    #[tokio::test]
    async fn sql_connection() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let pool = TicketsDB::new(&database_url).await.unwrap();    
        let row = sqlx::query!("SELECT 1 as number")
            .fetch_one(pool.database())
            .await
            .unwrap();
        assert_eq!(row.number, Some(1));
    }
    #[tokio::test]
    async fn add_and_get_ticket() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let db = TicketsDB::new(&database_url).await.unwrap();

        let title = TicketTitle::try_from("Test ticket").unwrap();
        let description = Some(TicketDescription::try_from("Test description").unwrap());

        let id = db.add_ticket(title, description).await.unwrap();

        let ticket = db.get_ticket(id).await.unwrap();

        let (_, title, description, priority, status) = ticket.get_self_parts();

        assert_eq!(title.into_inner(), "Test ticket");
        assert_eq!(description.unwrap().into_inner(), "Test description");
        assert_eq!(priority, TicketPriority::Medium);
        assert_eq!(status, TicketStatus::New);


        let title = TicketTitle::try_from("Test adjusted ticket").unwrap();
        let description = Some(TicketDescription::try_from("Test adjusted description").unwrap());
        let priority = TicketPriority::try_from("low").unwrap();
        let status = TicketStatus::try_from("completed").unwrap();

        db.patch_ticket(id, Some(title.clone()), description.clone(), Some(priority.clone()), Some(status.clone())).await.unwrap();

        let ticket = db.get_ticket(id).await.unwrap();

        let (_, title, description, priority, status) = ticket.get_self_parts();
        
        assert_eq!(title.into_inner(), "Test adjusted ticket");
        assert_eq!(description.unwrap().into_inner(), "Test adjusted description");
        assert_eq!(priority, TicketPriority::Low);
        assert_eq!(status, TicketStatus::Completed);
    }
}