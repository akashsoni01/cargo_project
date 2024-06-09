use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware, HttpRequest};
use actix_web::cookie::time::macros::date;
use actix_web::web::service;
use rustis::{
    client::Client,
};
use rustis::commands::StringCommands;
use rustis::resp::cmd;

/*
request
http://localhost:8080/test_lang2?lang=hi-in&query={}

response
language = hi-in!
*/





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


#[derive(Clone)]
pub struct RedisClient {
    client: Client
}
// connectino, power, change
impl RedisClient {
    async fn build() -> Self {
        let client = Client::connect("127.0.0.1:6379").await.unwrap();
        return Self {
            client
        }
    }
}

#[derive(Clone)]
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

async fn test_redis_set_1(
    data: web::Data<AppState>,
) -> impl Responder {
    &data.redis_client.client.send(cmd("MSET")
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
        .await
        .unwrap()
        .to::<()>()
        .unwrap();
    return HttpResponse::Ok().body("saved!")

}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[get("/test_redis_set_2")]
async fn test_redis_set_2(
    data: web::Data<AppState>,
) -> impl Responder {
    let client = &data.redis_client.client;
    let values: Vec<String> = data.redis_client.client
        .send(
            cmd("MGET").arg("key1").arg("key2").arg("key3").arg("key4"),
            None,
        )
        .await
        .unwrap()
        .to()
        .unwrap();


    return HttpResponse::Ok().body(values.join("-"))
}

#[actix_web::main]
#[cfg_attr(feature = "tokio-runtime", tokio::main)]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let state = AppState::build().await.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(test_redis_set_2)
            .route("/hey2", web::get().to(test_redis_set_1))
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}



