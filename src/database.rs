use sqlx::PgPool;

use crate::tickets::*;

// DB Struct ──────────────────────────────────────────────────
#[derive(Clone)]
pub struct TicketsDB {
    database: PgPool
}

// DB methods ──────────────────────────────────────────────────
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

    // Add a ticket to the database ──────────────────────────────────────────────────
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
    pub async fn get_ticket(
        &self,
        ticket_id: TicketId,
    ) -> Result<Ticket, anyhow::Error> {
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

    // Retrieve tickets ──────────────────────────────────────────────────
    pub async fn get_tickets(
        &self,
        limit: i64,
    ) -> Result<Vec<Ticket>, anyhow::Error> {
        let rows = sqlx::query!(
            "SELECT * FROM tickets ORDER BY id LIMIT $1",
            limit
        )
        .fetch_all(&self.database)
        .await?;

        let tickets = rows
            .into_iter()
            .map(|row| {
                Ok(Ticket::from_parts(
                    row.id.try_into()?,
                    row.title.try_into()?,
                    row.description
                        .map(|d| d.try_into())
                        .transpose()?,
                    row.priority.try_into()?,
                    row.status.try_into()?,
                ))
            })
            .collect::<Result<Vec<Ticket>, anyhow::Error>>()?;

        Ok(tickets)
    }

    // Patch a ticket ──────────────────────────────────────────────────
    pub async fn patch_ticket(
        &self,
        id: TicketId,
        title: TicketTitle,
        description: Option<TicketDescription>,
        priority: TicketPriority,
        status: TicketStatus,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
            UPDATE tickets
            SET
                title = $1,
                description = $2,
                priority = $3,
                status = $4
            WHERE id = $5
            "#,
            title.into_inner(),
            description.map(TicketDescription::into_inner),
            priority.into_inner(),
            status.into_inner(),
            id.into_inner(),
        )
        .execute(&self.database)
        .await?;

        Ok(())
    }
}

// Tests ──────────────────────────────────────────────────
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

    // Test creating, getting and then patching a ticket ──────────────────────────────────────────────────
    #[tokio::test]
    async fn add_get_patch_ticket() {

        // Init
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let db = TicketsDB::new(&database_url).await.unwrap();

        // Adding a ticket
        let title = TicketTitle::try_from("Test ticket").unwrap();
        let description = Some(TicketDescription::try_from("Test description").unwrap());
        let id = db.add_ticket(title, description).await.unwrap();
        
        // Getting it back to check it 
        let ticket = db.get_ticket(id).await.unwrap();
        let (_, title, description, priority, status) = ticket.get_self_parts();

        // Tests
        assert_eq!(title.into_inner(), "Test ticket");
        assert_eq!(description.unwrap().into_inner(), "Test description");
        assert_eq!(priority, TicketPriority::Medium);
        assert_eq!(status, TicketStatus::New);

        // Send a patch
        let title = TicketTitle::try_from("Test adjusted ticket").unwrap();
        let description = Some(TicketDescription::try_from("Test adjusted description").unwrap());
        let priority = TicketPriority::try_from("low").unwrap();
        let status = TicketStatus::try_from("completed").unwrap();
        db.patch_ticket(id, title.clone(), description.clone(), priority.clone(), status.clone()).await.unwrap();

        // Testing the changes
        let ticket = db.get_ticket(id).await.unwrap();
        let (_, title, description, priority, status) = ticket.get_self_parts();
        assert_eq!(title.into_inner(), "Test adjusted ticket");
        assert_eq!(description.unwrap().into_inner(), "Test adjusted description");
        assert_eq!(priority, TicketPriority::Low);
        assert_eq!(status, TicketStatus::Completed);
    }
}