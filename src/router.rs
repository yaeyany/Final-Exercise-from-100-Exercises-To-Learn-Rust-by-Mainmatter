use axum::{
    Router, routing::{get, patch, post},
};

use crate::handlers::*;

use crate::database::TicketsDB;

pub fn router(tickets: TicketsDB) -> Router {
    Router::new()
        // 1. The HTML page route
        .route("/ticket/list", get(|| html_handler("ticket_list.html")))
        
        // 2. The JSON API routes (renamed slightly or kept separate)
        .route("/ticket/create", 
            get(|| html_handler("ticket_create.html"))
            .post(create_ticket)
        )
        .route("/api/ticket/list", get(list_tickets)) // <-- JSON endpoint
        .route("/ticket/{id}", patch(patch_ticket))
        
        .with_state(tickets)
}