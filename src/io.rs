use std::{io::{stdout, stdin, Write}};
use crate::{Directory, Record};
use crate::dirs;
use crate::utils::display;

pub fn main_menu() {
    loop 
    {println!("====================================================");
    println!("                      MAIN MENU          ");
    println!();
    println!("                    SELECT OPTION        ");
    println!();
    println!("[1] VIEW DIRECTORIES [2] NEW DIRECTORY [3] NEW RECORD\n
[4] DELETE DIRECTORY [5] DELETE RECORD [6] EXIT");
    println!();
    println!("===================================================");
    if !take_input("$>".to_string()) {
        break;
        }
    }
} 

pub fn take_input(msg: String) -> bool {
    let mut input = String::new();
    print!("{}", msg);

    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Error reading input");

    if let Some('\n') = input.chars().next_back(){input.pop();}

    match input.as_str() {
        "1" => {list_dirs(); true},
        "2" => {create_dir(); true},
        "3" => {new_record(); true},
        "4" => {del_dir(); true},
        "5" => {del_rec(); true},
        "6" => {println!("Exiting.. Goodbye"); false},
        _ => {println!("Unknown command, please select from the given options"); true}
    }
}

pub fn del_rec() {
    let mut rec_name = String::new();
    let mut dir_name = String::new();

    println!("Directory of record to delete: ");
    stdin().read_line(&mut dir_name).expect("Error, invalid entry");

    unsafe {
        for mut dir in &mut dirs {
            println!("{}, {}", dir.name, dir_name.trim().clone());
        if dir.name == dir_name.trim() {
            println!("Directory found");
            println!("Enter ID of directory to delete");
            stdin().read_line(&mut rec_name).expect("Error, invalid entry");
            for mut rec in &mut dir.records {
                println!("{}", rec.name().clone());
                if rec.name() == rec_name.trim() {
                    let mut id = rec.id().clone();
                    println!("{}", rec.name().clone());
                    dir.delete_record_index(id.try_into().unwrap());
                    return;
                }
            }
        }
    }}
}


pub fn del_dir() {
    let mut dir_name = String::new();
    println!("Enter name of directory to delete: ");
    stdin().read_line(&mut dir_name).expect("Error, invalid entry");
    unsafe {
        for mut dir in &mut dirs {
            if dir.name == dir_name.trim() {
                println!("Deleting directory: {}", dir.name.clone());
                dir.delete_dir();
            }
        }
    }
}
pub fn list_dirs() {
    unsafe {
        for mut dir in &mut dirs { 
            dir.display_info();
        }
    }
}
pub fn create_dir() {
    let mut dir_name = String::new();
    println!("Enter new directory name: ");
    stdin().read_line(&mut dir_name).expect("Error, invalid entry");
    unsafe
   { dirs.push(Directory::new(dir_name.clone()));}

   println!("Created new directory: {}", dir_name.clone());
}

pub fn new_record() {
    let mut dir_name = String::new();
    let mut rec_name = String::new();
    println!("Directory of new record: ");
    stdin().read_line(&mut dir_name).expect("Error, invalid entry");

    unsafe {
        for mut dir in &mut dirs {
            println!("{}, {}", dir.name, dir_name.trim().clone());
        if dir.name == dir_name.trim() {
            println!("Directory found");
            &mut dir.new_record(3, None);
            //&mut dir.new_record(dir.default_fields.as_ref().unwrap().len().try_into().unwrap(), None);
        }
    }}
}