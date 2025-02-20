use super::*;
#[derive(Debug, schemars::JsonSchema, serde::Serialize, serde::Deserialize)]
pub struct User {
    id: i32,
    name: String,
    address: Address,
}
