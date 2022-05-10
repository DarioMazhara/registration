use dialoguer::console::Key;
use serde_derive::{Serialize, Deserialize};
use crate::utils::display;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Record {
    name: String,
    id: u32,
    #[serde(rename = "type")]
    typename: String,
    is_mutable: bool,
    #[serde(with = "serde_with::rust::tuple_list_as_map")]
    key_vals: Vec<(String, String)>,
}




impl Record {
    pub fn default(typename: String, id: u32, is_mutable: bool) -> Self {
        unsafe {
            Self {                                                                                                                                              
                name: String::from("record_".to_owned() + &id.to_string()),
                typename,
                is_mutable,
                id,
                key_vals: Vec::new(),
            }
        }
    }
    pub fn name(&self) -> String {
        return self.name.clone();
    }
    pub fn typename(&self) -> String {
        return self.typename.clone();
    }
    pub fn is_mutable(&self) -> bool {
        return self.is_mutable;
    }
    pub fn set_mutable(&mut self, mutability: bool) {
        if mutability {
            self.is_mutable = true;
        }
        else {
            self.is_mutable = false;
        }
    }
    pub fn key_vals(&mut self) -> &mut Vec<(String, String)> {
        &mut self.key_vals
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }
    pub fn record(name: String, typename: String, id: u32) -> Self {
        unsafe {
            Self {
                name,
                typename,
                is_mutable: true,
                id,
                key_vals: Vec::new(),
            }
        }
    }
    pub fn update(&mut self) {
        std::fs::write(
            "directories/".to_owned() + &self.typename.clone() + "_directory/" + &self.name.clone() + ".json",
            serde_json::to_string_pretty(&self).unwrap(),
        ).unwrap();
     }
     pub fn add_custom_value(&mut self, field: String, value: String) {
         self.key_vals().push((field.clone(), value.clone()));
         self.update();
     }
 
     pub fn set_field_name(&mut self, current: String, new: String) {
         for i in self.key_vals.iter_mut() {
             if i.0 == current {
                 i.0 = new;
                 break;
             }
         }
         self.update();
     } 
     pub fn set_value(&mut self, field: String, new_val: String) {
         if !self.is_mutable {
             println!("Error setting value: this record is immutable.");
             return;
         }
        for i in self.key_vals.iter_mut() {
            if i.0 == field {
                i.1 = new_val;
                self.update();
                return;
            }
        }
        println!("This field does not exist or is immutable");
    }

}