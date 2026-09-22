use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod tickets;
mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    let app = Router::new();
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(
        listener, 
        app.into_make_service()
    )
    .await?;

    Ok(())
}