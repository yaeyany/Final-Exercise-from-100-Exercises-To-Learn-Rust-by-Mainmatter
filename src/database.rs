use anyhow::Ok;
use sqlx::PgPool;

use crate::tickets::*;

pub struct TicketsDB {
    database: PgPool
}

impl TicketsDB {
    //Initialize new TicketDB with a given database URL
    pub async fn new(database_url: &str) -> Result<Self, anyhow::Error> {
        let db = TicketsDB { 
            database: sqlx::PgPool::connect(database_url).await?,
        };
        Ok(db)
    }
    //Reference self db
    pub fn database(&self) -> &PgPool {
        &self.database
    }
    //Add a ticket to the database
    pub async fn add_ticket(
        &self,
        title: TicketTitle,
        description: Option<TicketDescription>,
    ) -> Result<(), anyhow::Error> {
        let title = title.into_inner();
        let description = description.map(TicketDescription::into_inner);

        let query = sqlx::query!(
            "INSERT INTO tickets (title, description, priority, status)
            VALUES ($1, $2, 'medium', 'new')",
            title,
            description
        );

        query.execute(&self.database).await?;
        Ok(())
    }
    //Retrieve ticket details
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
}
//Patch a ticket