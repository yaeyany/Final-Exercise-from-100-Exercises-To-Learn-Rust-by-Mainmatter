use axum::{
    Router, routing::{get, patch, post},
};

use crate::handlers::{
    create_ticket,
    list_tickets,
    patch_ticket,
};

use crate::database::TicketsDB;

pub fn router(tickets: TicketsDB) -> Router {
    Router::new()
        .route("/ticket/create", post(create_ticket))
        .route("/ticket/list", get(list_tickets).patch(patch_ticket))
        .with_state(tickets)
}