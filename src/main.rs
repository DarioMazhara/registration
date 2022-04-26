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

    fn name_search(&mut self, mut name: &str) -> &User {
        let name = name.to_string();
        let mut results: Vec<&User> = Vec::new();
        let mut index = 0;
        for (pos, e) in self.records.iter().enumerate() {
            if self.records[pos].name.clone() == name.clone() {
                results.push(&self.records[pos]);
                index = pos;
            }
        }
        if results.len() == 0 {
            println!("No existing record");
            return &self.none;
        }
        else if results.len() == 1 {
            println!("Record found.");
            print_deserialized(Some(results[index]), None);
        }
        else {
            print_deserialized(None, Some(results));
        }

        return &self.none;
    }

    fn address_search(&self, mut address: &str) -> &User {
        let address = address.to_string();
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
            return &self.none;
        }
        else if results.len() == 1 {
            println!("Record found.");
            println!("{:?}", self.records[index]);
            return &self.records[index];
        }
        print_deserialized(None, Some(results));
        return &self.none;
        
    }
    fn record(&mut self, name: &str, address: String, age: i32) {
        let mut record_naming = "user_".to_owned() + name;
        let mut record: User = User::new(name.to_string(), address, age);
        let mut record_copy = User::new(record.name.clone(), record.address.clone(), age);
        self.records.push(record);
        std::fs::write(
            "users_directory/".to_owned() + &record_naming + &".json".to_owned(),
            serde_json::to_string_pretty(&record_copy).unwrap(),
        ).unwrap();
        let serialized = serde_json::to_string(&record_copy).unwrap();
        //println!("{:?}", serde_json::from_str::<User>(&serialized).unwrap());
        println!("Record added!");
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub address: String,
    pub age: i32,
}

impl User {
    fn new(name: String, address: String, age: i32) -> Self {
        Self {
            name,
            address,
            age
        }
    }
    fn none() -> Self {
        Self {
            name: String::from("Record not found"),
            address: String::from("Record not found"),
            age: -1,
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
fn main() -> Result<(), std::io::Error> {

    let mut dir: UserDirectory = UserDirectory::new();
    
    dir.record("Rose Bafaiz", String::from("1920 Mantova Street"), 21);
    dir.record("Dario Mazhara", String::from("1920 Mantova Street"), 20);

    dir.address_search("1920 Mantova Street");

    Ok(())
}