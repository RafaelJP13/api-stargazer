use serde::{Serialize, Deserialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Champion {
    pub id: Uuid,
    pub name: String,
    pub title: String,
    pub description: String,
    pub role: String,
    pub difficulty_level: String,
}
