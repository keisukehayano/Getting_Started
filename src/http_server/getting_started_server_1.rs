use actix_web::{ web, HttpResponse };

// この関数は別のモジュールに配置できます
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
     cfg.service(
         web::resource("/test")
         .route(web::get().to(|| HttpResponse::Ok().body("test")))
         .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
     );
 }
 
 
 
 // この関数は別のモジュールに配置できます
 pub fn config(cfg: &mut web::ServiceConfig) {
     cfg.service(
         web::resource("/app")
         .route(web::get().to(|| HttpResponse::Ok().body("app")))
         .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
     );
 }

 // 上記の例の結果は次のようになります。

//  /         -> "/"
//  /app      -> "app"
//  /api/test -> "test"