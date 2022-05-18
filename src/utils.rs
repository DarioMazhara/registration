use crate::Directory;
use crate::Record;
pub trait display {
    fn display(&mut self);
}

impl display for Record {
    fn display(&mut self) {
        println!("=============================");
        println!("            RECORD           ");
        println!("IMMUTABLE FIELDS");
        println!("Name: {}", self.name());
        println!("ID: {}", self.id());
        println!("Is mutable: {}", self.is_mutable());
        println!("Type: {}", self.typename());

        println!("FIELDS");
        for (key, val) in self.key_vals() {
            println!("{}: {}", key, val);
        }


    }
}

impl display for Directory {
    fn display(&mut self) {
        println!("==========================================================");
        println!("                       DIRECTORY                          ");
        println!("Name: {}", self.name.clone());
        println!("Records quantity: {}", self.quantity());
        println!("Hello");
        println!("Default fields: {:#?}", self.default_fields());
        println!("Records: {{");
        for record in self.records() {
            record.display();
        }
        println!("}}");

    }
}