use serde::{Serialize, Deserialize};
use mongodb::bson;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub name: String,
    pub description: String,
    pub origin: String,
    pub name_origin: String,
    pub evolution: String,
    pub category: String,
    pub height: String,
    pub weight: String,
    pub pokemon_types: Vec<String>,
    pub generation: i32
}
