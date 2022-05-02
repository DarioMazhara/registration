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



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub user_id: i64,
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
struct Directory {
    records: Vec<Record>,
}

impl Directory {
    fn new() -> Self {
        std::fs::create_dir("directory");
        Self {
            records: Vec::new(),
        }
    }
    fn new_json_obj(&mut self, key_vals: Vec<(String, String)>) -> Value {
        let mut map = Map::new();
        for (key, val) in key_vals.into_iter() {
         //   println!("{}: {}", key, val);
            map.insert(key, Value::String(val));
        }

        let obj = Value::Object(map);
        std::fs::write(
            "directory/record.json",
            serde_json::to_string_pretty(&obj).unwrap(),
        ).unwrap();
        return obj;
    }
    fn delete_record_index(&mut self, index: u32) {
        
    }

    fn new_records(&mut self, num_fields: u32) -> Record {
       // let mut key_vals: Vec<(String, String)> = Vec::new();
        let mut key_vals: Vec<(String, String)> = Vec::new();
        let mut field: String = String::new();
        let mut value: String = String::new();
        let mut expected = Record::default();
       // expected.name = String::from("record");
        let mut i = 0;
        while (i < num_fields) {
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
        println!("{:#?}", expected);
        std::fs::write(
            expected.name.clone() + ".json",
            serde_json::to_string_pretty(&expected).unwrap(),
        ).unwrap();
        return expected;
    }
}   

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
struct Wrapper<T>(T);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record{
    name: String,
    #[serde(with = "serde_with::rust::tuple_list_as_map")]
    key_vals: Vec<(String, String)>,
}

static mut RECORDS: u32 = 0;
impl Record {
    fn default() -> Self {
        unsafe {
            let mut this = Self {
                name: String::from("record_".to_owned() + &(RECORDS).to_string()),
                key_vals: Vec::new(),
            };
            RECORDS += 1;
            this
        }
    }
    fn record(name: String) -> Self {
        unsafe {
            RECORDS += 1;
            Self {
                name,
                key_vals: Vec::new(),
            }
        }
    }

    fn update(&mut self) {
       std::fs::write(
           self.name.clone() + ".json",
           serde_json::to_string_pretty(&self).unwrap(),
       ).unwrap();
    }

    fn set_field_name(&mut self, current: String, new: String) {
        for i in self.key_vals.iter_mut() {
            if (i.0 == current) {
                i.0 = new;
                break;
            }
        }
        self.update();

    }

    fn set_value(&mut self, field: String, new_val: String) {
        for i in self.key_vals.iter_mut() {
            if i.0 == field {
                i.1 = new_val;
                break;
            }
        }
        self.update();
    }
}

type Label = (Wrapper<String>, Wrapper<String>);

macro_rules! var_make {
    ($field: expr, $value:expr) => {
        let mut expr = $value;
    };
}

fn main() -> Result<()> {
 //  let mut dir: Directory = Directory::new();

  // let mut obj: Record = dir.new_records(3);



    let path = "record_0.json";
    let data = fs::read_to_string(path).expect("ERROR!");


    let mut record: Record = serde_json::from_str::<Record>(&data).expect("ERROR");
   // let mut res: Value = serde_json::from_str(&data).expect("ERROR!");
  
    record.set_value(String::from("name"), String::from("test"));

    

   // res.set_value(String::from("bank"), String::from("$2,000,000"));
  //  res.set_field_name(String::from("name"), String::from("tag"));
 //   println!("{}", data);
   // println!("{:#?}", res);
   /*   
    let transaction = r#"{
        "name": "transactions",
        "key_vals": {
            "from": "ethan gary",
            "to": "krishan lall",
            "amount": "$450,920.00",
            "status":"posted"
        }
    }"#;
    let mut required = Record::default(String::from("transactions"));
    required.name = String::from("transactions");
    required.key_vals.push((String::from("from"),String::from("nicholas gary")));
    required.key_vals.push((String::from("to"), String::from("krishan lall")));
    required.key_vals.push((String::from("amount"), String::from("$450,920.00")));
    required.key_vals.push((String::from("status"), String::from("posted")));
    std::fs::write(
        "transaction.json",
        serde_json::to_string_pretty(&required).unwrap(),
    ).unwrap();

    required.key_vals[0].0 = String::from("penis");
    println!("{:?}", required.key_vals);

    



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