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
use crate::utils::display;
use crate::record::Record;
use crate::directory::Directory;
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

fn main() -> Result<()> {

    let mut dirs: Vec<Directory> = Directory::load().unwrap();

    dirs[0].add_field("name".to_string(), None);
    dirs[0].add_field("address".to_string(), None);
    dirs[0].add_field("password".to_string(), Some(true));

    dirs[0].get_record(1).unwrap().set_mutable(true);
    dirs[0].get_record(1).unwrap().set_value("id".to_string(), "10".to_string());
    dirs[0].display();
/*                                                                                                                                                                                                        
    let data = r#"{
        "name": "users",
       "key_vals": {
           "name": "dario mazhara",
           "address": "10 arabian court",
           "age": "20"
            }
       }"#;
       */
    Ok(())
}