use chrono::prelude::*;
use std::string::String;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;

fn write_generic(message: String) {
    match env::home_dir() {
        Some(path) => {
            let mut file = OpenOptions::new()
                .append(true)
                .open(format!("{}/.evertils/rolling.log", path.display()))
                .unwrap();

            if let Err(e) = write!(file, "{}", message) {
                println!("Error writing to rolling log: {}", e);
            }
        },
        None => println!("Impossible to get your home dir!"),
    }
}

fn write(message: String) {
    match env::home_dir() {
        Some(path) => {
            let mut file = OpenOptions::new()
                .append(true)
                .open(format!("{}/.evertils/rolling.log", path.display()))
                .unwrap();

            let dt = Local::now();
            let fmt_message: String = format!("\n{}/{}/{} - {}", dt.month(), dt.day(), dt.year(), message);

            if let Err(e) = write!(file, "{}", fmt_message) {
                println!("Error writing to rolling log: {}", e);
            }
        },
        None => println!("Impossible to get your home dir!"),
    }
}

pub fn update(message: String) {
    self::write(message);
}

pub fn new_day() {
    let date = Local::now();
    let message: String = format!("=====================\n{}\n=====================", date);

    self::write_generic(message);
}
