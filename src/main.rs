use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


mod http_server;
use crate::http_server::getting_started_server_1::hello;
use crate::http_server::getting_started_server_1::echo;
use crate::http_server::getting_started_server_1::manual_hello;


// 次に、アプリインスタンスを作成し、リクエストハンドラーを登録します。
// ルーティングマクロを使用するハンドラーにはApp :: serviceを使用し、
// 手動でルーティングされるハンドラーにはApp :: routeを使用して、パスとメソッドを宣言します。
// 最後に、アプリはHttpServer内で起動され、アプリを「アプリケーションファクトリ」として使用して着信リクエストを処理します。


   #[actix_web::main]
   async fn main() -> std::io::Result<()> {
       HttpServer::new(|| {
           App::new()
           .service(hello)
           .service(echo)
           .route("/hey", web::get().to(manual_hello))
       })
       .bind("127.0.0.1:8080")?
       .run()
       .await
   }


// それでおしまい！カーゴランでプログラムをコンパイルして実行します。 
// ＃[actix_web :: main]マクロは、actixランタイム内で非同期main関数を実行します。
// これで、http：//127.0.0.1：8080 /または定義した他のルートにアクセスして、結果を確認できます。

