use std::collections::HashMap;
use dialoguer::Input;

struct RecordDirectory<T> {
   // fields_and_types: HashMap<String, String>,
    records: Vec<T>,
    // settings
    max_records: Option<i32>,
    removable: bool,
    mutable: bool,
}
impl<T> RecordDirectory<T> {
    fn new(/*settings*/ max_records: i32, removable: bool, mutable: bool) -> Self {
        Self {
            max_records: Some(max_records),
            records: Vec::new(),
            removable,
            mutable,
        }
    }
}

struct Record {
    pub record_type: String,
    pub fields_and_values: HashMap<String, String>,
}   
impl Record {
    fn empty_record(record_type: String) -> Self {
        Self {
            record_type,
            fields_and_values: HashMap::new(),
        }
    }

    fn fill_record(&mut self, field: String, data: String) {
        self.fields_and_values.insert(
            field.clone(),
            data,
        );
        println!("{}", self.record_type);
        println!("{}: {:?}", field.clone(), self.fields_and_values.get(&field).unwrap());
    }
}

pub fn get_data(prompt: &str) -> String {
    return Input::<String>::new().with_prompt(prompt).interact_text().unwrap();
}
/* 
fn main() {
    let mut record: Record = Record::empty_record(String::from("User"));

    record.fill_record(String::from("Name"), String::from("Dario Mazhara"));
}*/