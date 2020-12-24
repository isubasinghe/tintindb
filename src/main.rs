#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod tokenizer;
mod dtos;
mod index;

fn main() {
    println!("Hello, world!");
}
