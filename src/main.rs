use actix_web::{ App, HttpServer };


mod http_server;
use crate::http_server::getting_started_server_1::index;
use crate::http_server::getting_started_server_1::AppState;

// アプリを初期化するときに状態を渡し、アプリケーションを起動します。

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("actix-web"),
            })
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// アプリケーション内には、任意の数の状態タイプを登録できます。