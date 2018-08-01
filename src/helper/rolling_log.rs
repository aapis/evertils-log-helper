use chrono::prelude::*;
use std::string::String;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;

pub fn update(message: String) {
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
