use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::{HttpRequest};
use rustis::{
    client::Client,
    commands::{FlushingMode, ServerCommands, StringCommands},
    Result,
};
use rustis::resp::cmd;

// use url::form_urlencoded;
// use serde::Deserialize;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    lang: String,
}

/*
request
http://localhost:8080/test_lang2?lang=hi-in&query={}

response
language = hi-in!
*/

#[get("/test_lang2")]
async fn test_lang2(info: web::Query<Info>) -> String {
    format!("language = {}!", info.lang)
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


/*
Request
http://127.0.0.1:8080/test_lang?lang=hi-in


Response
hi-in
*/
// #[get("/test_lang")]
// async fn get_language_from_url(req: HttpRequest) -> impl Responder {
//     // Extract the query parameter 'lang' from the URL
//     let query_string = req.query_string().to_string();
//     if let Some((key, value)) = url::form_urlencoded::parse(query_string.as_bytes()).find(|(key, _)| key == "lang") {
//         println!("Found lang parameter: {} = {}", key, value);
//         return HttpResponse::Ok().body(value.to_string());
//     }
//     HttpResponse::BadRequest().body("Lang parameter not found")
// }

// #[get("/test_lang2")]
// async fn get_language_from_url2(req: HttpRequest) -> impl Responder {
//     req.
// }

pub struct RedisClient {
    client: Client
}
// connectino, power, change
impl RedisClient {
    async fn build() -> Self {
        let client = Client::connect("127.0.0.1:6379").await?;
        return Self {
            client
        }
    }
}
pub struct AppState {
    redis_client: RedisClient
}

impl AppState {
    async fn build() -> Self {
        return Self {
            redis_client: RedisClient::build().await
        }
    }
}

#[get("/test_redis_set_1")]
async fn test_redis_set_1(
    state: web::Data<AppState>,
) -> impl Responder {
    state.redis_client.client.send( cmd("MSET")
                                        .arg("key1")
                                        .arg("value1")
                                        .arg("key2")
                                        .arg("value2")
                                        .arg("key3")
                                        .arg("value3")
                                        .arg("key4")
                                        .arg("value4"),
                                    None,
    )
}


#[cfg_attr(feature = "tokio-runtime", tokio::main)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(AppState::build())
            .service(hello)
            .service(echo)
            // .service(get_language_from_url)
            .service(test_lang2)
            .service(test_redis_set_1)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}



