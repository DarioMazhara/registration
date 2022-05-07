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

    let mut dirs: Option<Vec<Directory>> = Directory::load();
    dirs.as_mut().unwrap()[0].display();

    dirs.as_mut().unwrap()[0].add_field("name".to_string(), None);
    dirs.as_mut().unwrap()[0].manual_record(vec![("name".to_string(), "again".to_string())]);
    dirs.as_mut().unwrap()[0].display();
   /* 
   // dirs[0].display();
    let mut dirs: Vec<Directory> = Vec::new();
    dirs.push(Directory::new("account".to_string()));
    dirs[0].manual_record(vec![("name".to_string(), "sabrina".to_string())]);

  //  dirs[0].display();


     dirs[0].manual_record(vec![("name".to_string(), "krishan".to_string())]);
//     dirs[0].manual_record(vec![("name".to_string(), "krishan".to_string())]);
    dirs[0].manual_record(vec![("name".to_string(), "ethan".to_string())]);
    dirs[0].manual_record(vec![("name".to_string(), "rose".to_string())]);
    dirs[0].manual_record(vec![("name".to_string(), "adam".to_string())]);
    dirs[0].manual_record(vec![("name".to_string(), "marat".to_string())]);

    

    dirs[0].get_record(1).unwrap().set_value("name".to_string(), "dario".to_string());
    dirs[0].delete_record_index(1);
    dirs[0].display();
*/
    
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