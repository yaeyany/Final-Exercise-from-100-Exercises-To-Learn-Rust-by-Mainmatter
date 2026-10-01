use anyhow::Ok;
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

    // Delete a ticket ──────────────────────────────────────────────────
    pub async fn delete_ticket(
    &self,
    id: TicketId,
    ) -> Result<bool, anyhow::Error> {
        let deleted = sqlx::query_scalar!(
            r#"
            DELETE FROM tickets
            WHERE id = $1
            RETURNING id
            "#,
            id.into_inner(),
        )
        .fetch_optional(&self.database)
        .await?;

        Ok(deleted.is_some())
    }
}