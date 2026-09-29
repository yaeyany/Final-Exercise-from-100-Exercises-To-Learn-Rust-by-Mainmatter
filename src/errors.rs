// Custom Ticket errors ──────────────────────────────────────────────────
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TicketError {
    #[error("Ticket ID invalid. Can only be more than 0")]
    IdInvalid,

    #[error("Title cannot be empty")]
    TitleEmpty,

    #[error("Title is too long. Max 50 characters")]
    TitleTooLong,

    #[error("Description is too long. Max 100 characters")]
    DescriptionTooLong,

    #[error("Please enter a valid priority")]
    PriorityInvalid,

    #[error("Please enter a valid status")]
    StatusInvalid,
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
// 1. Create a wrapper type
pub struct AppError(anyhow::Error);

// 2. Tell Axum how to convert it into an HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the error internally for debugging
        print!("AppError: {:?}", self.0);
        // Return a safe generic error message to the client
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong on our end.",
        )
            .into_response()
    }
}

// 3. Enable using the `?` operator on any error that can convert into anyhow::Error
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
