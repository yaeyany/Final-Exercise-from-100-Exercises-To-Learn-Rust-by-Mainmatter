
#[derive(Debug, PartialEq, Eq)]
pub struct Ticket {
    id: TicketId,
    title: TicketTitle,
    description: TicketDescription,
    priority: TicketPriority,
    status: TicketStatus,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TicketId(u32);
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

impl Ticket {
    pub fn new(id: TicketId, title: TicketTitle, description: Option<TicketDescription>) -> Self {
        let description = match description {
            Some(desc) => desc,
            None => TicketDescription("No description".to_string()),
        };

        Self {
            id,
            title,
            description,
            priority: TicketPriority::Medium,
            status: TicketStatus::New,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticket_new() {
        let title = TicketTitle("test".to_string());
        let ticket1 = Ticket::new(TicketId(1), title.clone(), Some(TicketDescription("test".to_string())));
        let ticket2 = Ticket::new(TicketId(2), title, None);
        assert_eq!(ticket1.description.0, "test");
        assert_eq!(ticket2.description.0, "No description");
    }
}



