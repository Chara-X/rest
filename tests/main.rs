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
pub fn openapi() -> openapiv3::OpenAPI {
    let title = "API".to_string();
    let version = "0.0.1".to_string();
    let url = "/api/v1".to_string();
    let ops: Vec<Op> = vec::Vec::new();
    let mut api = openapiv3::OpenAPI {
        openapi: "3.0.3".to_string(),
        info: openapiv3::Info {
            title: title,
            version: version,
            ..Default::default()
        },
        servers: vec![openapiv3::Server {
            url: url,
            ..Default::default()
        }],
        ..Default::default()
    };
    let mut settings = generate::SchemaSettings::openapi3();
    settings.inline_subschemas = true;
    let generator = &mut schemars::SchemaGenerator::new(settings);
    for op in ops {
        if let openapiv3::ReferenceOr::Item(path) = api
            .paths
            .paths
            .entry(op.path)
            .or_insert(openapiv3::ReferenceOr::Item(openapiv3::PathItem::default()))
        {
            let operation = Some(openapiv3::Operation {
                request_body: Some(openapiv3::ReferenceOr::Item(openapiv3::RequestBody {
                    content: indexmap::indexmap! {
                        "application/json".to_string() => openapiv3::MediaType {
                            schema: Some(openapiv3::ReferenceOr::Item(serde_json::from_value(generator.root_schema_for::<User>().clone().to_value()).unwrap())),
                            ..Default::default()
                        }
                    },
                    ..Default::default()
                })),
                responses: openapiv3::Responses {
                    default: Some(openapiv3::ReferenceOr::Item(openapiv3::Response {
                        description: "Ok".to_string(),
                        content: indexmap::indexmap! {
                            "application/json".to_string() => openapiv3::MediaType {
                                schema: Some(openapiv3::ReferenceOr::Item(serde_json::from_value(generator.root_schema_for::<User>().clone().to_value()).unwrap())),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    })),
                    ..Default::default()
                },
                ..Default::default()
            });
            match op.method.as_str() {
                "POST" => path.post = operation,
                "GET" => path.get = operation,
                "DELETE" => path.delete = operation,
                _ => panic!(),
            };
        }
    }
    api
}
pub struct Op {
    pub method: String,
    pub path: String,
    pub input: Option<String>,
    pub output: Option<String>,
}
