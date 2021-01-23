
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


// リクエストハンドラーは、0個以上のパラメーターを受け入れる非同期関数を使用します。
// これらのパラメーターは、リクエストから抽出でき（FromRequestトレイトを参照）、
// HttpResponseに変換できるタイプを返します（レスポンダートレイトを参照）。



     // ルートパスにgetで来たら`Hello World`を返す
   #[get("/")]
   pub async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello World!")
   }

   // ルートパスにpostで来たら`送信した文字列をそのまま返す`
   #[post("/echo")]
   pub async fn echo(req_body: String) -> impl Responder {
       HttpResponse::Ok().body(req_body)
   }

   // マニュアルで記述する場合＊メイン関数でルーティングを記述する
   pub async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there")
   }
