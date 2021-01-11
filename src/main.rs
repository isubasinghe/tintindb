#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use tracing::{Level, event};
use tracing_subscriber;
use crossbeam_utils::sync::ShardedLock;
use std::rc::Rc;

mod tokenizer;
mod dtos;
mod index;
mod http;

fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    event!(Level::INFO, "Starting application");
    let config = index::persistance::StoreConfig { path: "./db".to_string(), cache: index::persistance::Size::GB(1), wal: index::persistance::Size::GB(1)};
    let document_store = index::persistance::DocumentStore::new(&config).unwrap();

    http::server::web_main(document_store).unwrap();
    event!(Level::INFO, "Exiting application");
}
