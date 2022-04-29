use serde_json::{Number, Value};
use serde_derive::{Serialize, Deserialize};

/* 
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Directory {
    name: String,
    records: Vec<User>,
}
impl Directory {
    fn new(name: String) -> Self {
        std::fs::create_dir(name.clone());
        Self {
            name,
            records: Vec::new(),
        }
    }
}*/
#[derive(Serialize, Deserialize, Debug)]
struct UserDirectory {
    records: Vec<User>,
    none: User,
}
impl UserDirectory {
    fn new() -> Self {
        std::fs::create_dir("users_directory");
        Self {
            records: Vec::new(),
            none: User::none(),
        }
    }

    fn name_search(&mut self, mut name: &str) -> &mut User {
        let name = name.to_string();
        let mut results: Vec<&User> = Vec::new();
        let mut index = 0;
        for (pos, e) in self.records.iter().enumerate() {
            if self.records[pos].name.clone() == name {
                results.push(&self.records[pos]);
                index = pos;
            }
        }
        if results.len() == 0 {
            println!("No existing record");
            return &mut self.none;
        }
        else if results.len() == 1 {
            println!("Record found.");
           // print_deserialized(Some(results[index]), None);
           // return &mut results[index];
           return &mut self.records[index];
        }
      
        return &mut self.none;
    }

    fn address_search(&mut self, mut address: &str) -> &mut User {
        let address_string = address.to_string();
        let mut results: Vec<&User> = Vec::new();
        let mut index = 0;
        for (pos,e) in self.records.iter().enumerate() {
            if self.records[pos].address == address {
                results.push(&self.records[pos]);
                //print_deserialized(&self.records[pos]);
                index = pos;
            }
        }
        if results.len() == 0  {
            println!("No existing record");
            return &mut self.none;
        }
        else if results.len() == 1 {
            println!("Record found.");
            println!("{:?}", self.records[index]);
            return &mut self.records[index];
        }
        print_deserialized(None, Some(results));
        return &mut self.none;
        
    }
    fn record(&mut self, name: &str, address: String, age: i32) {

        let mut record: User = User::new(name.to_string(), address, age);
        let mut record_copy = User::new(record.name.clone(), record.address.clone(), age);
        self.records.push(record);
        std::fs::write(
            "users_directory/".to_owned() + &record_copy.record_name.clone() + &".json".to_owned(),
            serde_json::to_string_pretty(&record_copy).unwrap(),
        ).unwrap();
        let serialized = serde_json::to_string(&record_copy).unwrap();
        //println!("{:?}", serde_json::from_str::<User>(&serialized).unwrap());
        println!("Record added!");
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub record_name: String,
    pub name: String,
    pub address: String,
    pub age: i32,
}

static mut RECORDS: u32 = 0;
impl User {

    fn new(name: String, address: String, age: i32) -> Self {
        unsafe
        {RECORDS += 1;}
        unsafe
        {Self {
            record_name: String::from("user_".to_owned() + &RECORDS.to_string()),
            name,
            address,
            age
        }}
    }
    fn none() -> Self {
        Self {
            record_name: String::from("Record not found"),
            name: String::from("Record not found"),
            address: String::from("Record not found"),
            age: -1,
        }
    }
    fn new_name(&mut self, data: &str) {
        self.modify_internal("name", Some(data.to_string()), None);
     //   let record_data = std::fs::read_to_string()
    }
    fn new_address(&mut self, data: &str) {
        self.modify_internal("address", Some(data.to_string()), None);
        std::fs::write(
            "users_directory/user_".to_owned() + &self.name.clone() + &".json".to_owned(),
            serde_json::to_string_pretty(&self).unwrap(), 
        ).unwrap();
    }
    fn new_age(&mut self, data: i32) {
        self.modify_internal("age", None, Some(data));
        std::fs::write(
            "users_directory/user_".to_owned() + &self.name.clone() + &".json".to_owned(),
            serde_json::to_string_pretty(&self).unwrap(), 
        ).unwrap();
    }

    fn modify_internal(&mut self, field: &str, text: Option<String>, numeric: Option<i32>) {
        match field {
            "name" => { self.name = text.unwrap() },
            "address" => { self.address = text.unwrap() },
            "age" => { self.age = numeric.unwrap() },
            _ => {  },
        }
    }
}
pub fn print_deserialized(user: Option<&User>, users: Option<Vec<&User>>) {
    if !user.is_none() && users.is_none() {
        let serialized = serde_json::to_string(&user).unwrap();
        println!("{:?}", serde_json::from_str::<User>(&serialized).unwrap());
    }
    
    else if user.is_none() && !users.is_none(){
        println!("Multiple records");
        let mut users = users.unwrap();
        for i in 0..users.len() {
            println!("{:?}", users[i]);

        }
    }
    
}

// serde_json::to_string().unwrap();
// serde_json::from_str().unwrap();
/*
std::fs::write(
    "path",
    serde_json::to_string_pretty(&record).unwrap(),
).unwrap();
*/
/*
let mut record = {
let record = std::fs::read_to_string("path")?;
serde_json::from_str::<User>(&record).unwrap()
}
*/
/* 
fn main() -> Result<(), std::io::Error> {

    let mut dir: UserDirectory = UserDirectory::new();
    
    dir.record("Rose Bafaiz", String::from("1920 Mantova Street"), 21);
    dir.record("Dario Mazhara", String::from("1920 Mantova Street"), 20);
    dir.record("Dario Mazhara", String::from("10 Arabian Court"), 20);

    dir.address_search("1920 Mantova Street");
   // dir.name_search("Dario Mazhara").new_age(3);
    println!("{}", dir.name_search("Dario Mazhara").age);

    dir.address_search("10 Arabian Court").new_name("Pickle");

    println!("{}", dir.address_search("10 Arabian Court").name);

    println!("{}", dir.address_search("10 Arabian Court").record_name);
    Ok(())
}*/