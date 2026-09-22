use axum::{Router, extract::ConnectInfo, response::{Html, Redirect}, routing::{get, post}};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let main = Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(hello_world).put(page_not_found).delete(list_users));

    let app = Router::new()
        .nest("/api/users", main)
        .route("/", get(index))
        .route("/page-not-found", get(page_not_found))
        .route("/hello-world", get(hello_world))
        .route("/users", get(list_users).post(create_user))
        .fallback(anything_else); // <-- add that fallback;

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

// create an easy handler
async fn anything_else() -> Redirect {
    Redirect::to("/page-not-found")
}

async fn page_not_found() -> &'static str {
    "I think you made a typo"
}

// change return type to `Html<String>` to let browser know we are sending html
async fn index(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Html<String> {
    let html = format!(
        "<h1>Your ip address is: \"{addr}\"</h1>\n\
         <h2>You are in immediate danger of getting identified by bad people.</h2>\n\
         <h2>Thankfully we have a VPN service to hide your ip.</h2>\n\
         <h2>Visit <a href=\"http://localhost:3000/average_joe_absolutely_needs_vpn\">THIS</a> link to download it.</h2>"
    );

    // create `Html` type like this
    Html(html)
}
 
async fn list_users() -> &'static str { "List users" }
async fn create_user() -> &'static str { "Create user" }
 