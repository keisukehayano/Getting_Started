use actix_web::{ web, App, HttpServer };
use std::sync::Mutex;

mod http_server;
use crate::http_server::getting_started_server_1::index;
use crate::http_server::getting_started_server_1::AppStateWithCounter;

// アプリにデータを登録します。

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // カウンターをクロージャーに移動します
        App::new()
        // 注：データの代わりにapp_dataを使用する
        .app_data(counter.clone())      // <- register the created data
        .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}