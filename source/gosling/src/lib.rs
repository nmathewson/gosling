#[macro_use]
extern crate lazy_static;
extern crate crypto;
extern crate data_encoding;
extern crate anyhow;

mod ffi;
mod tor_crypto;
mod object_registry;

#[no_mangle]
pub extern "C" fn rust_hello_world() -> i32 {
    println!("Hallo van Rust!");
    return 0;
}
