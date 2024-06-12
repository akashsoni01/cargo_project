use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware, HttpRequest};
use actix_web::cookie::time::macros::date;
use actix_web::web::service;
use rustis::{
    client::Client,
};
use rustis::commands::StringCommands;
use rustis::resp::cmd;
use uuid::{NoContext, Timestamp, Uuid};

/*
pub fn current_thread_runtime() -> tokio::runtime::Runtime {
    let mut builder = tokio::runtime::Builder::new_current_thread();
    builder.enable_io();
    builder.enable_time();
    builder.build().unwrap()
}
*/
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


/*
// no need to spawn task if the number of task is one hahahah

async fn test_redis_set_1(
    data: web::Data<AppState>,
) -> impl Responder {
    let tasks = tokio::spawn(async move {
        let client = data.redis_client.client.clone();
        let key_uuid_stirng = Uuid::new_v4().to_string();
        let value_uuid_stirng = Uuid::new_v4().to_string();
        client.set(key_uuid_stirng, value_uuid_stirng).await.unwrap();
    });
    futures_util::future::join_all(tasks).await;
    return HttpResponse::Ok().body("saved!")

}

*/

async fn test_redis_set_1(
    data: web::Data<AppState>,
) -> impl Responder {
    let key_uuid_stirng = Uuid::new_v4().to_string();
    let value_uuid_stirng = Uuid::new_v4().to_string();
    data.redis_client.client.set(key_uuid_stirng, value_uuid_stirng).await.unwrap();
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
    let state = AppState::build().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(test_redis_set_2)
            .route("/hey2", web::get().to(test_redis_set_1))
            .route("/hey", web::get().to(manual_hello))
    }).workers(4)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}



/*
Running 5s test @ http://127.0.0.1:8080/hey2
  8 threads and 800 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.83ms  709.28us  16.14ms   79.06%
    Req/Sec     5.46k     1.60k    8.75k    59.50%
  217181 requests in 5.01s, 16.78MB read
  Socket errors: connect 555, read 115, write 0, timeout 0
Requests/sec:  43381.23
Transfer/sec:      3.35MB
(base) akashsoni@Akashs-MacBook-Air poc_redis_actix % wrk -t8 -c400 -d5s -s test.lua http://127.0.0.1:8080/hey
test.lua: cannot open test.lua: No such file or directory
test.lua: cannot open test.lua: No such file or directory
test.lua: cannot open test.lua: No such file or directory
test.lua: cannot open test.lua: No such file or directory
test.lua: cannot open test.lua: No such file or directory
test.lua: cannot open test.lua: No such file or directory
test.lua: cannot open test.lua: No such file or directory
test.lua: cannot open test.lua: No such file or directory
test.lua: cannot open test.lua: No such file or directory
Running 5s test @ http://127.0.0.1:8080/hey
  8 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    20.77ms   51.09ms 491.42ms   88.59%
    Req/Sec    34.41k    31.25k  171.37k    78.68%
  1251736 requests in 5.05s, 102.66MB read
  Socket errors: connect 155, read 115, write 0, timeout 0
Requests/sec: 248079.55
Transfer/sec:     20.35MB
*/


