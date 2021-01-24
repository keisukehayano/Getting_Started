use actix_web::{ web, App, HttpServer };
use std::sync::Mutex;


// web :: scope（）メソッドを使用すると、リソースグループのプレフィックスを設定できます。
// このスコープは、リソース構成によって追加されたすべてのリソースパターンの前に付加されるリソースプレフィックスを表します。
// これは、同じリソース名を維持しながら、元の作成者が意図したものとは異なる場所に一連のルートをマウントするのに役立ちます。

// For example:

#[actix_web::main]
async fn main() {
    let scope = web::scope("/users").service(show_users);
    App::new().service(scope);
}


// 上記の例では、アプリケーションのスコープ引数がパターンの前に付加されるため、
// show_usersルートの有効なルートパターンは/ showではなく/ users / showになります。
// この場合、ルートはURLパスが/ users / showの場合にのみ一致し、
// HttpRequest.url_for（）関数がルート名show_usersで呼び出されると、同じパスのURLが生成されます。