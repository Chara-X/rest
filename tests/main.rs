#![allow(special_module_name)]
mod lib;
pub use lib::*;
use std::vec;
rest::client!({
    info {
        title: "My API",
        version: "1.0",
        description: "My API",
    };
    server { url: "/api/v1" };
    op("POST", "/users", User, ());
    op("GET", "/users/{id}", (), User);
    op("POST", "/users/swap", User, User);
    op("DELETE", "/users/empty", (), ());
});
#[test]
fn main() {
    let api = Client::openapi();
    let text = serde_json::to_string(&api).unwrap();
    println!("{}", text);
}
