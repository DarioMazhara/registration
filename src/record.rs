use serde_derive::{Serialize, Deserialize};
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    id: u32,
    #[serde(rename = "type")]
    pub typename: String,
    #[serde(with = "serde_with::rust::tuple_list_as_map")]
    pub key_vals: Vec<(String, String)>,
}

impl Record {
    pub fn default(typename: String, id: u32) -> Self {
        unsafe {
            Self {
                name: String::from("record_".to_owned() + &id.to_string()),
                typename,
                id,
                key_vals: Vec::new(),
            }
        }
    }
    pub fn id(self) -> u32 {
        return self.id;
    }
    pub fn record(name: String, typename: String, id: u32) -> Self {
        unsafe {
            Self {
                name,
                typename,
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
        for i in self.key_vals.iter_mut() {
            if i.0 == field {
                i.1 = new_val;
                break;
            }
        }
        self.update();
    }
}