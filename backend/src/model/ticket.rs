use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum TicketStatus {
    New,
    Pending,
    Complete,
    Rejected,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ticket {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
    pub created_by: String,
    pub assigned_to: String,
    pub team_id: String,
}
