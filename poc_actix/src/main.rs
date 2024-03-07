use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::{HttpRequest};

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
#[get("/test_lang")]
async fn get_language_from_url(req: HttpRequest) -> impl Responder {
    // Extract the query parameter 'lang' from the URL
    let query_string = req.query_string().to_string();
    if let Some((key, value)) = url::form_urlencoded::parse(query_string.as_bytes()).find(|(key, _)| key == "lang") {
        println!("Found lang parameter: {} = {}", key, value);
        return HttpResponse::Ok().body(value.to_string());
    }


    HttpResponse::BadRequest().body("Lang parameter not found")
}

// #[get("/test_lang2")]
// async fn get_language_from_url2(req: HttpRequest) -> impl Responder {
//     req.
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_language_from_url)
            .service(test_lang2)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

