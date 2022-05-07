use crate::Record;
use crate::utils::display;
use core::num;
use std::{io::{stdin, stdout, Write}, path::{self, Path}};
use json::object::Object;
use serde_json::{Map, Value};
use glob::glob;
use serde_derive::{Serialize, Deserialize};
use std::fs::read_link; 
#[allow(dead_code)]

#[derive(Serialize)]
pub struct Directory {
    pub name:String,
    pub default_fields: Option<Vec<String>>,
    records: Vec<Record>,
    num_records: u32,
}



impl Directory {
    pub fn new(name: String) -> Self {

        println!("{}", name);
        std::fs::create_dir("directories/".to_owned() + &name.clone() + "_directory");
        let x = Self {
            name: name.clone(),
            default_fields: None,
            num_records: 0,
            records: Vec::new(),
        };
        std::fs::write(
            r#"directories/"#.to_owned() + &name.clone() + "_directory/info",
            x.num_records.to_string(),
        );
        return x;
    }
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

    pub fn load_print() {
        println!("Loading following paths and their records:");
        for e in glob("../registration/directories/*").expect("err") {
            match e {
                Ok(path) => {
                    let mut path_string = path.into_os_string().into_string().unwrap();
                    println!("[PATH] {}", path_string[15..].to_string());
                    println!();
                   // println!(" PATH {:#?}", path);
                    for file in std::fs::read_dir(path_string).unwrap() {
                        let mut file_string = file.unwrap().path().into_os_string().into_string().unwrap();
                        println!("\t[FILE] {}", file_string[27..].to_string());
                        println!();
                        //println!("\t{}", file.unwrap().path().display());
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
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
  //          let num_data = std::fs::read_to_string("../registration/directories/".to_owned() + &dir.name.clone() + "_directory/info.json");

//            dir.update();
            for file in std::fs::read_dir("../registration/directories/".to_owned() + &dir.name.clone() + "_directory/").unwrap() {
                let mut a = file.unwrap().path();
                let data = std::fs::read_to_string(a.clone()).expect("err");
                println!("{:#?}\n{}\n--------------------\n", a.clone(), data);
              //  println!("{:#?}", data_name)
                let filename = a.file_name()?.to_str();  
                if filename == Some("info") {
                    dir.num_records = data.to_string().parse::<u32>().unwrap();
                }
                else 
                {let mut record: Record = serde_json::from_str(&data).unwrap();
                dir.records.push(record);}
                
            }
        }
        return Some(dir_structs);
    }

    pub fn get_record(&mut self, id: u32) -> Option<&mut Record> {
        println!("Get record");
        for i in 0..self.records.len() {
            println!("{}", &self.records[i].name[7..]);
            if &self.records[i].name[7..] == id.to_string() {
                return Some(&mut self.records[i]);
            }
        }
        return None;
    }
    pub fn delete_record_index(&mut self, index: usize) {

        std::fs::remove_file("directories/".to_owned() + &self.name.clone() + "_directory/" + &self.records[index].name.clone() + &".json".to_owned());
        self.records.remove(index);
        self.num_records -= 1;
        self.update();
    }
    pub fn delete_all(&mut self) {
        for mut record in &mut self.records {
            std::fs::remove_file("directories/".to_owned() + &self.name.clone() + "_directory/" + &record.name.clone() + ".json");

        }
        self.records = Vec::new();
    }
    pub fn delete_dir(&mut self) {
        println!("{}","directories/".to_owned() + &self.name.clone() + "_directory/");
        std::fs::remove_dir_all("../registration/directories/".to_owned() + &self.name.clone() + "_directory/").expect("failed to delete");
    }
    pub fn manual_record(&mut self, key_vals: Vec<(String, String)>)  {
        if !self.check_fields(key_vals.clone()) {
            println!("Input does not match any field");
            return;
        }
        self.num_records += 1;
        let mut expected: Record = Record::default(self.name.clone(), self.num_records);
        for val in key_vals {
            expected.key_vals.push((val.0, val.1));
        }
        expected.update();
        self.update();
        self.records.push(expected);
    }
    pub fn update(&mut self) {
        std::fs::write(
            r#"directories/"#.to_owned() + &self.name.clone() + "_directory/info",
            &self.num_records.to_string(),
        );
    }
    pub fn new_record(&mut self, num_fields: u32, is_custom: Option<bool>)  {
       
       // let mut key_vals: Vec<(String, String)> = Vec::new();

        let mut field: String = String::new();
        let mut value: String = String::new();
        let mut expected = Record::default(self.name.clone(), self.num_records);
       // expected.name = String::from("record");
        let mut i = 0;
        while (i < num_fields) {

            let mut field = String::new();

            print!("Enter field: ");
            if !self.check_fields(vec![(field.clone(), String::new())]) {
                return;
            }
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
            expected.key_vals.push((field, value));
            field = String::new();
            value = String::new();
            i+=1;
        }
      //  println!("{:?}", key_vals);
        
   //     let mut obj = self.new_json_obj(key_vals);
   //     let mut string_obj = obj.to_string();

   //     let res: Record = serde_json::from_str(&string_obj).unwrap();)
        if is_custom.unwrap() == false && !self.check_fields(expected.key_vals.clone())  {
            return;
        }

        println!("{:#?}", expected);
        std::fs::write(
            "directories/".to_owned() + &self.name.clone() + "_directory/" + &expected.name.clone() + ".json",
            serde_json::to_string_pretty(&expected).unwrap(),
        );
        self.records.push(expected);
    }
    pub fn records(&mut self) -> &mut Vec<Record> {
        &mut self.records
    }
    pub fn quantity(&mut self) -> u32 {
        self.num_records
    }

    pub fn set_all_field(&mut self, current: String, new: String) {
        for mut record in &mut self.records {
            record.set_field_name(current.clone(), new.clone());
        }
    }
    pub fn set_all_value(&mut self, current: String, new: String) {
        for mut record in &mut self.records {
            record.set_value(current.clone(), new.clone());
        }
    }
}