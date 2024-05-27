/*
create docker image and run it from below two commands

sudo docker build . -t rustbuild:1
sudo docker run rustbuild:1


run command to see the images
sudo docker ps -a

run command on terminal to create a tar image
docker save rustbuild:1 > rustbuild.tar

*/


/*
how to upload a image container to docker hub so that anyone can pull it and use it

1. first create a docker hub account
2. create a repo say poc_actix_docker
3. login - on windows 10 wsl don't forget to use sudo to login otherwise it will not be able to save creds

sudo docker login

4. push by tagging the image buildname:versionname and then username/repo
docker tag rustbuild:1 akashsoni8285/actix_poc_docker_repo
5. push the image again on wsl use sudo
docker push akashsoni8285/actix_poc_docker_repo


6. to pull
docker pull akashsoni8285/actix_poc_docker_repo:latest


7. to remove it from local first list all images docker image
docker rmi akashsoni8285/actix_poc_docker_repo

9. to run it
docker run
*/
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
    println!("Your server is starting .... at 127.0.0.1:8080");
    println!("try running command on terminal \n curl ")

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



