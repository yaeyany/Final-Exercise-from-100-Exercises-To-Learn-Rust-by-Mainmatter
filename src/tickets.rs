use crate::{errors::{TicketPriorityError, TicketStatusError, TicketTitleError}, helpers::sanitize_string};


#[derive(Debug, PartialEq, Eq)]
pub struct Ticket {
    id: TicketId,
    title: TicketTitle,
    description: Option<TicketDescription>,
    priority: TicketPriority,
    status: TicketStatus,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TicketId(i64);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TicketTitle(String);
#[derive(Debug, PartialEq, Eq)]
pub struct TicketDescription(String);
#[derive(Debug, PartialEq, Eq)]
pub enum TicketPriority {
    Low,
    Medium,
    High,
}
#[derive(Debug, PartialEq, Eq)]
pub enum TicketStatus {
    New,
    InProgress,
    Completed,
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(TicketTitleError::Empty)
        } else if value.len() > 50 {
            Err(TicketTitleError::TooLong)
        } else {
            Ok(TicketTitle(value.to_string()))
        }
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for TicketPriority {
    type Error = TicketPriorityError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = sanitize_string(value);
        match value.as_str() {
            "low" => Ok(TicketPriority::Low),
            "medium" => Ok(TicketPriority::Medium),
            "high" => Ok(TicketPriority::High),
            _ => Err(TicketPriorityError::Invalid),
        }
    }
}

impl TryFrom<String> for TicketPriority {
    type Error = TicketPriorityError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for TicketStatus {
    type Error = TicketStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = sanitize_string(value);
        match value.as_str() {
            "new" => Ok(TicketStatus::New),
            "in progress" => Ok(TicketStatus::InProgress),
            "completed" => Ok(TicketStatus::Completed),
            _ => Err(TicketStatusError::Invalid),
        }
    }
}

impl TryFrom<String> for TicketStatus {
    type Error = TicketStatusError;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }

}

//Ticket methods
impl Ticket {
    //New ticket constructor
    pub fn new(id: TicketId, title: TicketTitle, description: Option<TicketDescription>) -> Self {
        Self {
            id,
            title,
            description,
            priority: TicketPriority::Medium,
            status: TicketStatus::New,
        }
    }
    //Change ticket state
    pub fn toggle_state(&mut self, status: TicketStatus) {
        self.status = status;
    }
    //Change ticket priority
    pub fn toggle_priority(&mut self, priority: TicketPriority) {
        self.priority = priority;
    }
    //Change ticket title
    pub fn edit_title(&mut self, new_title: TicketTitle) {
        self.title = new_title;
    }
    //Change ticket description
    pub fn edit_description(&mut self, new_description: TicketDescription) {
        self.description = Some(new_description);
    }
    //Get ticket id
    pub fn get_id(&self) -> TicketId {
        self.id
    }
}

#[cfg(test)]
mod tests {

    use crate::{tickets::*, errors::*};

    // ── TicketTitle tests ───────────────────────────────────────────────
    #[test]
    fn test_ticket_title_valid() {
        let title = TicketTitle::try_from("Hello").unwrap();
        assert_eq!(title.0, "Hello");
    }

    #[test]
    fn test_ticket_title_empty() {
        let err = TicketTitle::try_from("").unwrap_err();
        assert_eq!(err, TicketTitleError::Empty);
    }

    #[test]
    fn test_ticket_title_too_long() {
        let long = "a".repeat(51);
        let err = TicketTitle::try_from(long.as_str()).unwrap_err();
        assert_eq!(err, TicketTitleError::TooLong);
    }

    #[test]
    fn test_ticket_title_from_string_valid() {
        let title = TicketTitle::try_from("World".to_string()).unwrap();
        assert_eq!(title.0, "World");
    }

    // ── TicketPriority tests ───────────────────────────────────────────────
    #[test]
    fn test_ticket_priority_valid() {
        let p = TicketPriority::try_from("low").unwrap();
        assert_eq!(p, TicketPriority::Low);

        let p = TicketPriority::try_from("medium").unwrap();
        assert_eq!(p, TicketPriority::Medium);

        let p = TicketPriority::try_from("high").unwrap();
        assert_eq!(p, TicketPriority::High);
    }

    #[test]
    fn test_ticket_priority_invalid() {
        let err = TicketPriority::try_from("invalid").unwrap_err();
        assert_eq!(err, TicketPriorityError::Invalid);
    }

    #[test]
    fn test_ticket_priority_from_string() {
        let p = TicketPriority::try_from("high".to_string()).unwrap();
        assert_eq!(p, TicketPriority::High);
    }

    // ── TicketStatus tests ──────────────────────────────────────────────────
    #[test]
    fn test_ticket_status_valid() {
        let s = TicketStatus::try_from("new").unwrap();
        assert_eq!(s, TicketStatus::New);

        let s = TicketStatus::try_from("in progress").unwrap();
        assert_eq!(s, TicketStatus::InProgress);

        let s = TicketStatus::try_from("completed").unwrap();
        assert_eq!(s, TicketStatus::Completed);
    }

    #[test]
    fn test_ticket_status_invalid() {
        let err = TicketStatus::try_from("invalid").unwrap_err();
        assert_eq!(err, TicketStatusError::Invalid);
    }

    #[test]
    fn test_ticket_status_from_string() {
        let s = TicketStatus::try_from("completed".to_string()).unwrap();
        assert_eq!(s, TicketStatus::Completed);
    }
}




