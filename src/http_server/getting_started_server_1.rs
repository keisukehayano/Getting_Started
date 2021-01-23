use actix_web::{ web };

// HttpServerは、アプリケーションインスタンスではなく、アプリケーションファクトリを受け入れます。 
// HttpServerは、スレッドごとにアプリケーションインスタンスを構築します。
// したがって、アプリケーションデータは複数回作成する必要があります。
// 異なるスレッド間でデータを共有する場合は、共有可能なオブジェクトを使用する必要があります。送信+同期。

// 内部的には、web :: DataはArcを使用します。
// したがって、2つのアークの作成を回避するには、App :: app_data（）を使用してデータを登録する前にデータを作成する必要があります。

// 次の例では、変更可能な共有状態のアプリケーションを作成します。
// まず、状態を定義し、ハンドラーを作成します。

use std::sync::Mutex;

pub struct AppStateWithCounter {
     pub counter: Mutex<i32>,      // スレッド間で安全にミューテーションするにはミューテックスが必要です
}

pub async fn index(data: web::Data<AppStateWithCounter>) -> String {
     let mut counter = data.counter.lock().unwrap();        // <- counterのMutexGuardを取得します
     *counter += 1;                                         // <- MutexGuard内のアクセスカウンター

     format!("Request number: {}", counter)                 // <- response with count
}