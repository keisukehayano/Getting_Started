use actix_web::{ web, App, HttpResponse, HttpServer };

mod http_server;

use crate::http_server::getting_started_server_1::config;
use crate::http_server::getting_started_server_1::scoped_config;

// 単純さと再利用性のために、Appとweb :: Scopeの両方がconfigureメソッドを提供します。
// この関数は、構成の一部を別のモジュールまたはライブラリに移動する場合に役立ちます。
// たとえば、リソースの構成の一部を別のモジュールに移動できます。



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
        .configure(config)
        .service(web::scope("/api").configure(scoped_config))
        .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}