use json::object::Object;
use serde_json::{Map, Value};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::collections::VecDeque;
use serde_derive::{Serialize, Deserialize};
use serde_json::Result;
use serde_json::json;
use std::collections::HashMap;
use std::io::Read;
mod record;
mod directory;
mod utils;
mod io;
use crate::utils::display;
use crate::record::Record;
use crate::directory::Directory;
use crate::io::*;
extern crate glob;
use glob::glob;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
fn s(text: &str) -> String {
    return text.to_string();
}
static mut dirs: Vec<Directory> = Vec::new();
fn main() {
    unsafe 
    {
        dirs = Directory::load().unwrap();
        
        let mut ok = String::new();
        println!("THIS IS A BRANCH TEST\nPRESS ANY KEY TO CONTINUE...");
        stdin().read_line(&mut ok).expect("err");
        main_menu();

    }  /*                                                                                                                                                                                                        
    let data = r#"{
        "name": "users",
       "key_vals": {
           "name": "dario mazhara",
           "address": "10 arabian court",
           "age": "20"
            }
       }"#;
       */
}