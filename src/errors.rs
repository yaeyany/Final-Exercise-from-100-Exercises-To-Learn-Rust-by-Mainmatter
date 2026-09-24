
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TicketTitleError {
    #[error("Title cannot be empty")]
    Empty,

    #[error("Title is too long. Max 50 characters")]
    TooLong,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TicketPriorityError {
    #[error("Please eneter a valid priority")]
    Invalid,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TicketStatusError {
    #[error("Please enter a valid status")]
    Invalid,
}