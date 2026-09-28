use axum::response::{Html, Redirect};



async fn index_handler() -> Html<String> {
    match tokio::fs::read_to_string("index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>500 Internal Server Error</h1><p>index.html not found.</p>".to_string()),
    }
}

async fn redirect_to_home() -> Redirect {
    Redirect::temporary("/")
}
