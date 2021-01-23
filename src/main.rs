use actix_web::{ web, App,  HttpServer };


mod http_server;
use crate::http_server::getting_started_server_1::index;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // それに接続されているすべてのリソースとルートのプレフィックス...
            web::scope("/app")
                // ...したがって、これは `GET / app / index.html`のリクエストを処理します
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}