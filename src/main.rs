#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod tokenizer;
mod dtos;
mod index;

fn main() {
    let config = index::persistance::StoreConfig { path: "./db".to_string(), cache: index::persistance::Size::GB(1), wal: index::persistance::Size::GB(1)};
    let document_store = index::persistance::DocumentStore::new(&config).unwrap();
    println!("Hello, world!");
}
