#[macro_use]
mod base;

use base::Error;


/*

cargo run -A unused_import
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

RUSTFLAGS="$RUSTFLAGS -Awarnings" cargo run

*/

fn main() {

    // panic_never_call_this!();

    println!("{} {}", "abc 123", s!("error"));
}
