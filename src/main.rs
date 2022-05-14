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
    {//  let mut dirs: Vec<Directory> = Directory::load().unwrap();
        dirs.push(Directory::new("accounts".to_string()));
        dirs[0].add_field(s("name"), Some(false));
        dirs[0].add_field(s("address"), Some(false));
        dirs[0].add_field(s("age"), Some(false));

        dirs[0].manual_record(s("dario, 10 arabian court, 21"));
        ///////////////////////////////////////////////////////

        dirs.push(Directory::new("employees".to_string()));
        dirs[1].add_field(s("name"), Some(false));
        dirs[1].add_field(s("position"), Some(false));

        dirs[1].manual_record(s("ethan gary, manager"));
//      ////////////////////////////////////////////////////

        dirs.push(Directory::new("transactions".to_string()));
        dirs[2].add_field(s("type"), Some(false));
        dirs[2].add_field(s("amount"), Some(false));

        dirs[2].manual_record(s("transfer, $10000"));
        ////////////////////////////////////////////////////

       // dirs[0].new_record(3, Some(false));
      //  dirs[0].manual_record("bob, wlr, 50".to_string());

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