// String, &str
// CString, CStr
// OsString, OsStr
// FFI, Foreign function interface

// rand
// cargo.toml ->
// [dependencies]
// rand = "0.8.1"

#![no_implicit_prelude]

extern crate std;
use std::convert::From;
use std::{println, string::String, vec};
fn main() {
    let my_vec = vec![8, 9, 10];
    let my_string = String::from("This won't work");
    println!("{my_vec:?}, {my_string:?}");
}

