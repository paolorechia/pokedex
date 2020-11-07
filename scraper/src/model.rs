use mongodb::bson;
use serde::{Deserialize, Serialize};

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
    pub generation: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    pub ok: Vec<String>,
    pub redirected: Vec<String>,
    pub missing: Vec<String>,
}
