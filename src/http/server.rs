use actix_web::{web, App, HttpResponse, HttpServer};
use tracing::{instrument};
use crossbeam_utils::sync::ShardedLock;
use serde_json;

use crate::index::persistance::DocumentStore;

#[instrument]
#[actix_web::main]
pub async fn web_main(dstore: DocumentStore) -> std::io::Result<()> {
    let index = web::Data::new(ShardedLock::new(dstore));
    HttpServer::new(move || {
        App::new()
            .app_data(index.clone())
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}