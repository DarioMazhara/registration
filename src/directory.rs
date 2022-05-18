use crate::Record;
use crate::utils::display;
use core::num;
use std::{io::{stdin, stdout, Write, self}, path::{self, Path}, fs::{self, write, File}};
use json::object::Object;
use serde_json::{Map, Value};
use glob::glob;
use serde_derive::{Serialize, Deserialize};
use std::fs::read_link;
use std::str::Split; 
use pwhash::bcrypt;


#[allow(dead_code)]
#[allow(warnings, unused)]
#[derive(Serialize)]
pub struct Directory {
    pub name:String,
    pub default_fields: Option<Vec<String>>,
    pub records: Vec<Record>,
    num_records: u32,
}



impl Directory {
    pub fn add_field(&mut self, mut field: String, is_password: Option<bool>) {
        if let None = self.default_fields {
            self.default_fields = Some(Vec::new());
        }
        self.default_fields.as_mut().unwrap().push(field);
    }
    pub fn check_fields(&mut self, key_vals: Vec<(String, String)>) -> bool {
        for (key, val) in key_vals {
            if let None = self.default_fields {
                return false;
            }
            if !self.default_fields.as_mut().unwrap().contains(&key) {
                return false;
            }
        }
        return true;
    }
    pub fn default_fields(&mut self) -> Vec<String> {
        let mut to_return: Vec<String> = Vec::new();
        if None == self.default_fields {
            return vec![];
        }
        for field in self.default_fields.as_ref().unwrap() {
            to_return.push(field.clone());
        }
        return to_return;
    }
    pub fn delete_all(&mut self) {
        for mut record in &mut self.records {
            fs::remove_file("directories/".to_owned() + &self.name.clone() + "_directory/" + &record.name().clone() + ".json");
            self.num_records -= 1;
        }
        self.records = Vec::new();
    }

    pub fn delete_dir(&mut self) {
        println!("{}","directories/".to_owned() + &self.name.clone() + "_directory/");
        fs::remove_dir_all("../registration/directories/".to_owned() + &self.name.clone() + "_directory/").expect("failed to delete");
    }

    pub fn delete_record_index(&mut self, index: usize) {
        if self.num_records == 0 {
            return;
        }
        fs::remove_file("directories/".to_owned() + &self.name.clone() + "_directory/" + &self.records[index].name() + &".json".to_owned());
        self.records.remove(index);
        if self.num_records >= 0 
        {
            self.num_records -= 1;
        }
        self.update();
    }

    pub fn display_info(&mut self) {
        println!("==========================================================");
        println!("                       DIRECTORY                          ");
        println!("Name: {}", self.name.clone());
        println!("Records quantity: {}", self.quantity());
        println!("Default fields: {:#?}", self.default_fields());
    }

    pub fn get_record(&mut self, id: u32) -> Option<&mut Record> {
        println!("Get record");
        for i in 0..self.records.len() {
            println!("{}", &self.records[i].name()[7..]);
            if &self.records[i].name()[7..] == id.to_string() {
                return Some(&mut self.records[i]);
            }
        }
        return None;
    }
    fn iter_records(&mut self, current: String, new: String, value: bool) {
        self.records.iter_mut().for_each(|record| {
            if value {
                record.set_value(current.clone(), new.clone());
            }
            else {
                record.set_field_name(current.clone(), new.clone());
            }
        })
    }
    pub fn load() -> Option<Vec<Directory>>  {  
         
        Directory::load_print();
        let mut dirs: Vec<String> = Vec::new();
        for e in glob("../registration/directories/*").expect("err") {
            match e {
                Ok(path) => {
                    (&mut dirs).push(path.into_os_string().into_string().unwrap());
                },
                Err(e) => println!("{:?}", e),
            }
        }
 
        let mut dir_structs: Vec<Directory> = Vec::new();
        for i in 0..dirs.len() {
            let mut a = dirs[i][28..].to_string();
            a = a.replace("_directory", "");
            dir_structs.push(
                Directory { name:  a.to_string(), default_fields: None, num_records: 0, records: Vec::new() }
            )
        }
        for mut dir in &mut dir_structs {

            println!("../registration/directories/{}/_{}", dir.name.clone(), "_directory/");
            for file in fs::read_dir("../registration/directories/".to_owned() + &dir.name.clone() + "_directory/").unwrap() {
                
                let a = file.unwrap().path();
                let data = fs::read_to_string(a.clone()).expect("err");
                println!("{:#?}\n{}\n--------------------\n", a.clone(), data);
                let filename = a.file_name()?.to_str();  
                if filename == Some("info") {
                    dir.num_records = data.to_string().parse::<u32>().unwrap();
                }
                else {
                    let record: Record = serde_json::from_str(&data).unwrap();
                    dir.records.push(record);
                }
                
            }
        }
        return Some(dir_structs);
    }
    pub fn load_print() {
        println!("Loading following paths and their records:");
        for e in glob("../registration/directories/*").expect("err") {
            match e {
                Ok(path) => {
                    let path_string = path.into_os_string().into_string().unwrap();
                    println!("[FULL PATH] {}", path_string.to_string());
                    println!("[PATH] {}", path_string[15..].to_string());
                    println!("[PATH] {}", path_string[..].to_string());
                   // println!(" PATH {:#?}", path);
                    if path_string[28..] != ".DS_Store".to_string() {
                        for file in fs::read_dir(path_string).unwrap() {
                            let file_string: String = file.unwrap().path().into_os_string().into_string().unwrap();
                            println!("\t[FILE] {}", file_string[27..].to_string());
                            println!();
                            //println!("\t{}", file.unwrap().path().display());
                        }
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }

    pub fn manual_record(&mut self, vals_string: String)  {
        let split = vals_string.split(",");
        let mut vals: Vec<String> = Vec::new();
        let mut key_vals: Vec<(String, String)> = Vec::new();
        for val in split {
            println!("{}, ", val.clone());
            vals.push(val.to_string());
        }
        if None != self.default_fields {
            if self.default_fields.as_ref().unwrap().len() != vals.len() {
            println!("Value quantity does not match fields");
            return;
        }
        for i in 0..self.default_fields.as_ref().unwrap().len() {
            key_vals.push((self.default_fields.as_ref().unwrap()[i].clone(), vals[i].clone()));
        }
        if !self.check_fields(key_vals.clone()) {
            println!("Input does not match any field");
            return;
        }
    }
       
   
        let mut expected: Record = Record::default(self.name.clone(), self.num_records, false);
        for val in key_vals {
            if val.0.to_lowercase() == "password".to_string() {
                let h = bcrypt::hash(val.1).unwrap();
                expected.key_vals().push(("password hash".to_string(), h));
            }
            else {
                expected.key_vals().push((val.0, val.1));
            }
        }
        self.num_records += 1;
        expected.update();
        self.update();
        self.records.push(expected);
    }
    pub fn new(name: String) -> Self {


        fs::create_dir("directories/".to_owned() + &name.clone() + "_directory");
        let x = Self {
            name: name.clone(),
            default_fields: None,
            num_records: 0,
            records: Vec::new(),
        };
        write(
            r#"directories/"#.to_owned() + &name.clone() + "_directory/info",
            x.num_records.to_string(),
        );
        return x;
    }
    pub fn new_record(&mut self, num_fields: u32, is_custom: Option<bool>)  {
       
       // let mut key_vals: Vec<(String, String)> = Vec::new();
        let field: String = String::new();
        let value: String = String::new();
        let mut expected = Record::default(self.name.clone(), self.num_records, false);
       // expected.name() = String::from("record");
        let mut i = 0;
        while i < num_fields {

            let mut field = String::new();

            println!("Enter field: ");
        //    if !self.check_fields(vec![(field.clone(), String::new())]) {
         //       return;
        //    };
            let _ = stdout().flush();
            stdin().read_line(&mut field).expect("ERR");
            if let Some('\n')=field.chars().next_back(){field.pop();}
            if let Some('\r')=field.chars().next_back() {field.pop();}

            let mut value = String::new();
            
            print!("Enter the associated value: ");
        
            let _ = stdout().flush();
            
            stdin().read_line(&mut value).expect("ERR");
            
          //  key_vals.push((field.clone(), value.clone()));
          //  assert_eq!((field, value), *key_vals.back().unwrap());

            expected.key_vals().push((field, value));
            field = String::new();
            value = String::new();
            i+=1;
        }
      //  println!("{:?}", key_vals);
        
   //     let mut obj = self.new_json_obj(key_vals);
   //     let mut string_obj = obj.to_string();

   //     let res: Record = serde_json::from_str(&string_obj).unwrap();)
      /*   if is_custom != None {
            if is_custom.unwrap() == false && !self.check_fields(expected.key_vals().clone())  {
                return;
            }
        }*/
       
        self.num_records += 1;
        println!("{:#?}", expected);
        expected.update();
        self.update();
        self.records.push(expected);
    }
    pub fn quantity(&mut self) -> u32 {
        self.num_records
    }
    pub fn records(&mut self) -> &mut Vec<Record> {
        &mut self.records
    }
    pub fn set_all_field(&mut self, current: String, new: String) {
        self.iter_records(current, new, true);
    }
    pub fn set_all_value(&mut self, current: String, new: String) {
        self.iter_records(current, new, false);
    }
    pub fn update(&mut self) {
       let mut path: String = String::new();
        if let path = r#"directories/"#.to_owned() + &self.name.clone() + "_directory/info" {
        }
        write(
            r#"directories/"#.to_owned() + &self.name + "_directory/info",
            self.num_records.to_string(),
        );
        
    }
}