use actix_web::{ get, web };


// アプリケーションの状態は、同じスコープ内のすべてのルートとリソースで共有されます。
// 状態には、web :: Data <T>エクストラクタを使用してアクセスできます。
// ここで、Tは状態のタイプです。ミドルウェアでも状態にアクセスできます。

// 簡単なアプリケーションを作成し、アプリケーション名を次の状態で保存しましょう。

// This struct represents state
pub struct AppState {
     pub app_name: String,
}

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> String {
     let app_name = &data.app_name;     // <- get app_name

     format!("Hello {}!", app_name)     // <- response with app_name
}