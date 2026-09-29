use axum::{Json, extract::{Path, State}, response::{Html, Redirect}};
use serde::Deserialize;
use crate::{database::TicketsDB, errors::AppError, tickets::{Ticket, TicketDescription, TicketId, TicketPriority, TicketStatus, TicketTitle}};

// Ticket create struct ──────────────────────────────────────────────────
#[derive(Deserialize)]
pub struct RequestTicketCreate {
    title: String,
    description: Option<String>,
}

// Ticket patch struct ──────────────────────────────────────────────────
#[derive(Deserialize)]
pub struct RequestTicketPatch {
    title: String,
    description: Option<String>,
    priority: String,
    status: String,
}

// Ticket creation validation Json -> TicketTitle and TicketDescription ──────────────────────────────────────────────────
fn validate_ticket_request(
    request: RequestTicketCreate,
) -> Result<(TicketTitle, Option<TicketDescription>), anyhow::Error> {
    let title = request.title.try_into()?;

    let description = request
        .description
        .map(TicketDescription::try_from)
        .transpose()?;

    Ok((title, description))
}

// Ticket patch validation Json -> Ticket struct fields ──────────────────────────────────────────────────
fn validate_patch_request(
    request: RequestTicketPatch,
) -> Result<(TicketTitle, Option<TicketDescription>, TicketPriority, TicketStatus), anyhow::Error> {
    let title = request.title.try_into()?;

    let description = request
        .description
        .map(TicketDescription::try_from)
        .transpose()?;

    let priority = request.priority.try_into()?;
    let status = request.status.try_into()?;

    Ok((title, description, priority, status))
}

// Creating a ticket ──────────────────────────────────────────────────
pub async fn create_ticket(
    State(tickets): State<TicketsDB>,
    Json(request): Json<RequestTicketCreate>,
) -> Result<Json<TicketId>, AppError> {
    let (title, description) = validate_ticket_request(request)?;

    let id = tickets.add_ticket(title, description).await?;

    Ok(Json(id))
}

// List all tickets ──────────────────────────────────────────────────
pub async fn list_tickets(
    State(tickets): State<TicketsDB>,
) -> Result<Json<Vec<Ticket>>, AppError> {
    let tickets = tickets.get_tickets(100).await?;

    Ok(Json(tickets))
}

// Patch a ticket ──────────────────────────────────────────────────
pub async fn patch_ticket(
    State(tickets): State<TicketsDB>,
    Path(id): Path<i64>, // Extract id from URL
    Json(request): Json<RequestTicketPatch>,
) -> Result<(), AppError> {
    let ticket_id = id.try_into()?;
    let title = request.title.try_into()?;
    let description = request
        .description
        .map(TicketDescription::try_from)
        .transpose()?;
    let priority = request.priority.try_into()?;
    let status = request.status.try_into()?;

    tickets.patch_ticket(ticket_id, title, description, priority, status).await?;
    Ok(())
}

// Checking for an html file ──────────────────────────────────────────────────
pub async fn html_handler(path: &str) -> Html<String> {
    match tokio::fs::read_to_string(path).await {
        Ok(content) => Html(content),
        Err(_) => Html(format!(
            "<h1>500 Internal Server Error</h1><p>Critical error: HTML file '<strong>{}</strong>' not found on disk.</p>",
            path
        )),
    }
}
// Redirect to home ──────────────────────────────────────────────────
pub async fn redirect_to_home() -> Redirect {
    Redirect::temporary("/ticket/create")
}


// Tests ──────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use crate::handlers::*;

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
        let request = RequestTicketCreate {
            title: "Test ticket".to_string(),
            description: Some("This is a test ticket".to_string()),
        };

        // Create ticket
        let result = create_ticket(
            State(tickets.clone()),
            Json(request),
        ).await;

        // Check that the ticket exists and is correct
        let ticket_id = result.unwrap();
        println!("Created ticket: {:?}", ticket_id);

        let row = sqlx::query!(
            "SELECT title, description FROM tickets WHERE id = $1",
            ticket_id.into_inner()
        )
        .fetch_one(*State(tickets.database()))
        .await
        .unwrap();

        assert_eq!(row.title, "Test ticket");
        assert_eq!(
            row.description.as_deref(),
            Some("This is a test ticket")
        );
    }
}