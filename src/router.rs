use axum::{
    Router,
    routing::{get, patch},
};

use crate::database::TicketsDB;
use crate::handlers::*;

pub fn router(tickets: TicketsDB) -> Router {
    Router::new()
        .nest("/ticket", ticket_router())
        .route("/api/ticket/list", get(handler_ticket_list))
        .fallback(get(redirect_to_home))
        .with_state(tickets)
}

pub fn ticket_router() -> Router<TicketsDB> {
    Router::new()
        .route("/list", get(|| { html_handler("ticket_list.html")}))
        .route(
            "/create",
            get(|| { html_handler("ticket_create.html")})
                .post(handler_ticket_create),
        )
        .route("/{id}", 
            patch(handler_ticket_patch)
            .get(redirect_to_home)
            .delete(handler_ticket_delete))
}