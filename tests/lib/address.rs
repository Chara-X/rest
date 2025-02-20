#[derive(Debug, schemars::JsonSchema, serde::Serialize, serde::Deserialize)]
pub struct Address {
    city: String,
    street: String,
}
