use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;

pub trait MessageWriter {
    fn write_generic(&self, message: String);
    fn write_now(&self);

    fn write(&self, message: &mut String) {
        self.prepare(message);

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

    fn prepare(&self, message: &mut String) -> Option<String> {
        if message.to_owned().len() > 0 {
            Some(message.to_owned())
        } else {
            None
        }
    }
}

pub trait DefaultWriter {
    fn write_generic(&self);
}

pub struct Line {
    pub message: Option<String>
}

pub struct Banner;

pub struct CustomBanner {
    pub message: Option<String>
}

impl MessageWriter for Line {
    fn write_generic(&self, message: String) {
        self.write(&mut message);
    }

    fn write_now(&self) {
        let dt = Local::now();

        match self.message {
            Some(ref msg) => {
                let fmt_message: String = format!("\n{}/{}/{} - {}", dt.month(), dt.day(), dt.year(), msg);

                self.write_generic(fmt_message);
            }
            None => println!("Error formatting log message"),
        }
    }
}

impl DefaultWriter for Banner {
    fn write_generic(&self) {
        CustomBanner { message: Option::None }.write_now();
    }
}

impl MessageWriter for CustomBanner {
    fn write_generic(&self, message: String) {
        let default_banner: String = format!("=====================\n{}\n=====================", message);

        self.write(&mut default_banner);
    }

    fn write_now(&self) {
        let date = Local::now();
        let default_banner: String = format!("=====================\n{}\n=====================", date);

        self.write(&mut default_banner);
    }
}
