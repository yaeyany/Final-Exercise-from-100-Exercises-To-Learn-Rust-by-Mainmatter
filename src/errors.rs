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