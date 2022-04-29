use json::object::Object;
use serde_json::{Map, Value};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::collections::VecDeque;
use serde_derive::{Serialize, Deserialize};
use serde_json::Result;
use serde_json::json;
struct Directory {
    records: Vec<Object>,
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

    fn new_records(&mut self, num_fields: u32) -> Record {
       // let mut key_vals: Vec<(String, String)> = Vec::new();
        let mut key_vals: Vec<(String, String)> = Vec::new();
        let mut field: String = String::new();
        let mut value: String = String::new();
        let mut expected = Record::default();
        let mut i = 0;

        while i < num_fields {
            
            println!("Enter a field:");

            let _ = stdout().flush();

            stdin().read_line(&mut field).expect("ERR");

            if let Some('\n')=field.chars().next_back(){field.pop();}
            if let Some('\r')=field.chars().next_back() {field.pop();}
            
            let mut value = String::new();
            
            println!("Enter the associated value");
        
            let _ = stdout().flush();
            
            stdin().read_line(&mut value).expect("ERR");
            
            key_vals.push((field.clone(), value.clone()));
          //  assert_eq!((field, value), *key_vals.back().unwrap());
            expected.key_vals.push((Wrapper(field.clone()), Wrapper(value.into())));
            field = String::new();
            value = String::new();
            i+=1;
        }
      //  println!("{:?}", key_vals);
        
        let mut obj = self.new_json_obj(key_vals);
        let mut string_obj = obj.to_string();

        let res: Record = serde_json::from_str(&string_obj).unwrap();
        return res;
    }
}   

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
struct Wrapper<T>(T);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record{
    #[serde(with = "serde_with::rust::tuple_list_as_map")]
    key_vals: Vec<Label>,
}
impl Record {
    fn default() -> Self {
        Self {
            key_vals: Vec::new(),
        }
    }
}

type Label = (Wrapper<String>, Wrapper<String>);

macro_rules! var_make {
    ($field: expr, $value:expr) => {
        let mut expr = $value;
    };
}

fn main() -> Result<()> {
    let mut dir: Directory = Directory::new();
    
  //  let mut obj: Record = dir.new_records(3);

    let data = r#"{
       "key_vals": {
           "name": "dario mazhara",
           "address": "10 arabian court",
           "age": "20"
            }
       }"#;

    let mut expected = Record::default();
    expected.key_vals.push((Wrapper("name".into()), Wrapper("dario mazhara".into())));
    expected.key_vals.push((Wrapper("address".into()), Wrapper("10 arabian court".into())));
    expected.key_vals.push((Wrapper("age".into()), Wrapper("20".into())));

    let res: Record = serde_json::from_str(&data).unwrap();
    for((exp_k, exp_v), (res_k, res_v)) in expected.key_vals.iter().zip(&res.key_vals) {
        assert_eq!(exp_k.0, res_k.0);
        assert_eq!(exp_v.0, res_v.0);
    }
    assert_eq!(data, serde_json::to_string_pretty(&expected).unwrap());

    println!("{:?}", res.key_vals);
  //  let mut string_obj = obj.to_string();
  //  println!("{:?}", string_obj);

    
    Ok(())
}