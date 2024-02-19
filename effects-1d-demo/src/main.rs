//! This is the standalone demo program. It does not use `lib.rs`; instead it
//! also uses `demo` as a submodule directly.
//!
//! This allows the same demo to be compiled as WASM library and as standalone executable.

mod demo;

pub fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    demo::run();
}
