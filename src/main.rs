use actix_web::{ web, HttpResponse, HttpServer, guard, App };



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .service(
                web::scope("/")
                .guard(guard::Header("Host", "www.rust-lang.org"))
                .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
            .service(
                web::scope("/")
                .guard(guard::Header("Host", "www.rust-lang.org"))
                .route("", web::to(|| HttpResponse::Ok().body("user")))
            )
            .route("/", web::to(|| HttpResponse::Ok()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}