use axum::response::{Html, Redirect};
use serde::Deserialize;
use axum::Json;

use crate::{database::TicketsDB, errors::TicketError, tickets::{TicketDescription, TicketId, TicketTitle}};

// Ticket create struct ──────────────────────────────────────────────────
#[derive(Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: Option<String>,
}

// Ticket creation validation Json -> TicketTitle and TicketDescription ──────────────────────────────────────────────────
fn validate_request(
    request: CreateTicketRequest,
) -> Result<(TicketTitle, Option<TicketDescription>), TicketError> {
    let title = request.title.try_into()?;

    let description = request
        .description
        .map(TicketDescription::try_from)
        .transpose()?;

    Ok((title, description))
}

// Creating a ticket ──────────────────────────────────────────────────
async fn create_ticket(
    Json(request): Json<CreateTicketRequest>,
    tickets: &TicketsDB,
) -> Result<TicketId, anyhow::Error> {
    let (title, description) = validate_request(request)?;

    let id = tickets.add_ticket(title, description).await?;

    Ok(id)
}

// Checking for an html file ──────────────────────────────────────────────────
async fn index_handler() -> Html<String> {
    match tokio::fs::read_to_string("index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>500 Internal Server Error</h1><p>index.html not found.</p>".to_string()),
    }
}

// Redirect to home ──────────────────────────────────────────────────
async fn redirect_to_home() -> Redirect {
    Redirect::temporary("/")
}


// Tests ──────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use crate::router::*;

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

    // Testing ticket creation ──────────────────────────────────────────────────
    #[tokio::test]
    async fn create_ticket_test() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let tickets = TicketsDB::new(&database_url).await.unwrap();

        // Init request
        let request = CreateTicketRequest {
            title: "Test ticket".to_string(),
            description: Some("This is a test ticket".to_string()),
        };

        // Create ticket
        let result = create_ticket(
            Json(request),
            &tickets,
        ).await;

        // Check that the ticket exists and is correct
        let ticket_id = result.unwrap();
        println!("Created ticket: {:?}", ticket_id);

        let row = sqlx::query!(
            "SELECT title, description FROM tickets WHERE id = $1",
            ticket_id.into_inner()
        )
        .fetch_one(tickets.database())
        .await
        .unwrap();

        assert_eq!(row.title, "Test ticket");
        assert_eq!(
            row.description.as_deref(),
            Some("This is a test ticket")
        );
    }
}