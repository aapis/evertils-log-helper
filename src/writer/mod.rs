use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;

pub trait MessageWriter {
    fn write_generic(&self, message: String);
    fn write_now(&self, message: String);
    fn write_empty(&self);

    fn write(&self, message: String) {
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
}

pub trait DefaultWriter {
    fn write_generic(&self, message: String);
    fn write_empty(&self);
}

pub struct Line;
pub struct Banner;
pub struct CustomBanner;

impl MessageWriter for Line {
    fn write_generic(&self, message: String) {
        self.write(message);
    }

    fn write_now(&self, message: String) {
        let dt = Local::now();
        let fmt_message: String = format!("\n{}/{}/{} - {}", dt.month(), dt.day(), dt.year(), message);

        self.write(fmt_message);
    }

    fn write_empty(&self) {
        self.write(String::new());
    }
}

impl DefaultWriter for Banner {
    fn write_generic(&self, message: String) {
        CustomBanner {}.write_generic(message);
    }

    fn write_empty(&self) {
        CustomBanner {}.write_empty();
    }
}

impl MessageWriter for CustomBanner {
    fn write_generic(&self, message: String) {
        let default_banner: String = format!("=====================\n{}\n=====================", message);

        self.write(default_banner);
    }

    fn write_now(&self, message: String) {
        let date = Local::now();
        let default_banner: String = format!("=====================\n{}\n{}\n=====================", date, message);

        self.write(default_banner);
    }

    fn write_empty(&self) {
        let date = Local::now();
        let default_banner: String = format!("=====================\n{}\n=====================", date);

        self.write(default_banner);
    }
}
