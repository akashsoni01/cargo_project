use actix_web:: {
    middleware,
    web::{self},
    App,
    HttpServer,
    HttpResponse, 
    Responder
};

use std::collections::HashMap;
use juniper::{
    graphql_object, graphql_value, http::{playground::playground_source, GraphQLRequest}, EmptyMutation, EmptySubscription, FieldError, FieldResult, GraphQLObject, RootNode

};

use juniper_actix:: {
    graphiql_handler, 
    graphql_handler, 
    playground_handler
};


#[derive(Clone, GraphQLObject)]
pub struct User {
    id: i32,
    name: String,
}

#[derive(Clone, Default)]
pub struct Database {
    users: HashMap<i32, User>,
}

impl Database {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(1, User { id: 1, name: "John".to_string() });
        users.insert(2, User { id: 2, name: "Jane".to_string() });
        Database { users }
    }

    pub fn get_user(&self, id: i32) -> FieldResult<&User> {
        self.users.get(&id).ok_or(FieldError::new("User not found", graphql_value!({ "not_found": id })))
    }
}

struct Query;

#[graphql_object(context = Database)]
impl Query {
    fn user(context: &Database, id: i32) -> FieldResult<&User> {
        context.get_user(id)
    }
}


async fn handle_incoming_request() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/").route(web::get().to(handle_incoming_request))
            )
    });

    let url = "127.0.0.1:8080";
    println!("Starting server at: {}", url);
    server.bind(url).unwrap().run().await
}


/*
async fn handle_incoming_request_with_params(info: web::Path<(String, u32)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}! Your age is {}", info.0, info.1))
}

async fn handle_incoming_request_with_query_params(info: web::Query<(String, u32)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}! Your age is {}", info.0, info.1))
}

async fn handle_incoming_request_with_json(info: web::Json<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(info.0)
}

async fn handle_incoming_request_with_form(info: web::Form<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(info.0)
}

async fn handle_incoming_request_with_headers(info: web::Header<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(info.0)
}

async fn handle_incoming_request_with_body(info: web::Bytes) -> impl Responder {
    HttpResponse::Ok().body(info)
}

async fn handle_incoming_request_with_streaming_body(info: web::Payload) -> impl Responder {
    HttpResponse::Ok().streaming(info)
}

async fn handle_incoming_request_with_multipart(info: actix_multipart::Multipart) -> impl Responder {
    HttpResponse::Ok().body("Multipart")
}

async fn handle_incoming_request_with_cookies(info: web::Cookie) -> impl Responder {
    HttpResponse::Ok().body("Cookies")
}

async fn handle_incoming_request_with_cookies_and_state(info: web::Cookie, state: web::Data<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(state.0)
}

async fn handle_incoming_request_with_cookies_and_state_and_params(info: web::Cookie, state: web::Data<serde_json::Value>, info2: web::Path<(String, u32)>) -> impl Responder {
    HttpResponse::Ok().json(state.0)
}
*/