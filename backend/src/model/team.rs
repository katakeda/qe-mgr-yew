use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
}
