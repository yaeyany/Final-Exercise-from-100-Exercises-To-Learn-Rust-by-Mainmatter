use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::database::TicketsDB;

mod tickets;
mod database;
mod errors;
mod helpers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = TicketsDB::new(&database_url).await?;    
    
    let app = Router::new();
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    let server = tokio::spawn(async move {
        axum::serve(
            listener, 
            app.into_make_service()
        )
        .await.unwrap();
    });

    server.await?;
    Ok(())
}