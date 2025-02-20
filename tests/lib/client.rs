use super::*;
// use reqwest::blocking;
// use schemars::generate;
// pub struct Client {
//     client: blocking::Client,
//     addr: String,
// }
// impl Client {
//     pub fn openapi() -> openapiv3::OpenAPI {
//         let mut openapi = openapiv3::OpenAPI {
//             openapi: "3.0.3".to_string(),
//             info: openapiv3::Info {
//                 title: "My API".to_string(),
//                 version: "0.0.1".to_string(),
//                 ..Default::default()
//             },
//             servers: vec![openapiv3::Server {
//                 url: "/api/v1".to_string(),
//                 ..Default::default()
//             }],
//             ..Default::default()
//         };
//         let mut settings = generate::SchemaSettings::openapi3();
//         settings.inline_subschemas = true;
//         let generator = &mut schemars::SchemaGenerator::new(settings);
//         openapi.paths.paths.insert(
//             "/users/{id}/{name}".into(),
//             openapiv3::ReferenceOr::Item(openapiv3::PathItem {
//                 post: Some(openapiv3::Operation {
//                     parameters: parse_parameters("/users/{id}/{name}"),
//                     request_body: Some(openapiv3::ReferenceOr::Item(openapiv3::RequestBody {
//                         content: indexmap::indexmap! {
//                             "application/json".to_string() => openapiv3::MediaType {
//                                 schema: Some(openapiv3::ReferenceOr::Item(serde_json::from_value(generator.root_schema_for::<User>().clone().to_value()).unwrap())),
//                                 ..Default::default()
//                             }
//                         },
//                         ..Default::default()
//                     })),
//                     responses: openapiv3::Responses {
//                         default: Some(openapiv3::ReferenceOr::Item(openapiv3::Response {
//                             description: "Ok".to_string(),
//                             content: indexmap::indexmap! {
//                                 "application/json".to_string() => openapiv3::MediaType {
//                                     schema: Some(openapiv3::ReferenceOr::Item(serde_json::from_value(generator.root_schema_for::<User>().clone().to_value()).unwrap())),
//                                     ..Default::default()
//                                 }
//                             },
//                             ..Default::default()
//                         })),
//                         ..Default::default()
//                     },
//                     ..Default::default()
//                 }),
//                 ..Default::default()
//             }),
//         );
//         openapi
//     }
//     pub fn new(client: blocking::Client, addr: String) -> Self {
//         Self { client, addr }
//     }
// pub fn post_user(&self, body: &User) -> reqwest::Result<blocking::Response> {
//     let res = self
//         .client
//         .post(format!("{}/api/v1/users", self.addr))
//         .json(body)
//         .send()?;
//     if let Err(err) = res.error_for_status_ref() {
//         eprintln!("{}", res.text().unwrap());
//         return Err(err);
//     }
//     Ok(res)
// }
//     pub fn get_user_by_id(&self, id: String) -> reqwest::Result<User> {
//         let res = self
//             .client
//             .get(format!("{}/api/v1/users/{id}", self.addr))
//             .send()?;
//         if let Err(err) = res.error_for_status_ref() {
//             eprintln!("{}", res.text().unwrap());
//             return Err(err);
//         }
//         res.json()
//     }
// }
// use reqwest::blocking;
// pub struct Client {
//     client: blocking::Client,
//     addr: String,
// }
// impl Client {
//     pub fn new(client: blocking::Client, addr: String) -> Self {
//         Self { client, addr }
//     }
//     pub fn get_users_id(&self, id: &str) -> reqwest::Result<User> {
//         let res = self
//             .client
//             .get(::alloc::__export::must_use({
//                 let res = ::alloc::fmt::format(format_args!("{0}/users/{1}", self.addr, id));
//                 res
//             }))
//             .send()?;
//         if let Err(err) = res.error_for_status_ref() {
//             {
//                 ::std::io::_eprint(format_args!("{0}\n", res.text().unwrap()));
//             };
//             return Err(err);
//         }
//         res.json()
//     }
// }
