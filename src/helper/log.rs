use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;

struct Writer {
    message: &'static str,
    banner: bool
}

impl Writer {
    fn is_banner(&self) -> bool {
        self.banner
    }

    pub fn write_generic(&self, message: String) {
        match env::home_dir() {
            Some(path) => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(format!("{}/.evertils/rolling.log", path.display()))
                    .unwrap();

                if let Err(e) = write!(file, "\n{}", message) {
                    println!("Error writing to rolling log: {}", e);
                }
            },
            None => println!("Impossible to get your home dir!"),
        }
    }

    pub fn write_now(&self, message: String) {
        let dt = Local::now();
        let fmt_message: String = format!("\n{}/{}/{} - {}", dt.month(), dt.day(), dt.year(), message);

        self.write_generic(fmt_message);
    }

    pub fn banner(&self, banner_fmt: String, string_fmt: String) {
        let date = Local::now();
        let message: String = format!("=====================\n{}\n=====================", date);

        self.write_generic(message);
    }

    pub fn default_banner(&self, string_fmt: String) {
        let date = Local::now();
        let message: String = format!("=====================\n{}\n=====================", date);

        self.write_generic(message);
    }
}
