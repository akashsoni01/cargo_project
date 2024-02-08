use actix_web:: {
    Error, middleware, web::{self, Data, Payload}, App, HttpRequest, HttpResponse, HttpServer, Responder, Result
};

use std::collections::HashMap;
use juniper::{
    graphql_object, graphql_value, http::{playground::playground_source, GraphQLRequest}, EmptyMutation, EmptySubscription, FieldError, FieldResult, GraphQLObject, RootNode

};

use juniper_actix:: {
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
impl juniper::Context for Database {}

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
    fn apiVersion() -> &'static str {
        "1.0"
    }

    fn user(context: &Database, id: i32) -> FieldResult<&User> {
        context.get_user(id)
    }
    fn users(context: &Database) -> FieldResult<Vec<&User>> {
        context.users
    }
    // using sqlx fetch user by id 
    // fn user_by_id(context: &Database, id: i32) -> FieldResult<User> {
    //     let user = sqlx::query_as!(User, "SELECT id, name FROM users WHERE id = $1", id)
    //         .fetch_one(&context.pool)
    //         .await?;
    //     Ok(user)
    // }

    //  // using sqlx fetch users
    // fn users(context: &Database) -> FieldResult<Vec<User>> {
    //     let users = sqlx::query_as!(User, "SELECT id, name FROM users")
    //         .fetch_all(&context.pool)
    //         .await?;
    //     Ok(users)
    // }
    
}


type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}

async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
    // other examples 
    // playground_handler("/graphql", Some("/subscriptions")).await;
}

async fn graphiql_route(
    req: HttpRequest,
    payload: Payload,
    schema: Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Database::new();
    graphql_handler(&schema, &context, req, payload).await
}

// async fn handle_graphql_request(data: web::Data<Database>, req: web::Json<GraphQLRequest>) -> impl Responder {
//     let res = req.execute(&schema(), data);
//     HttpResponse::Ok().json(res)
// }

async fn handle_incoming_request() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let server = HttpServer::new(|| {
        App::new()
        .app_data(Data::new(schema())) // <- create app with shared state
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/").route(web::get().to(handle_incoming_request))
            )
            .service(
                web::resource("/graphql")
                .route(web::get().to(playground_route))
                .route(web::post().to(graphiql_route)),
            )
    });

    let url = "127.0.0.1:8080";
    println!("Starting server at: {}", url);
    server.bind(url).unwrap().run().await
}