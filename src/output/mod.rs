use std::any::Any;
use std::fmt::Display;
use std::fmt::Debug;
use chrono::prelude::*;

pub trait TerminalMessageWriter<T> {
    fn write_generic(&self, message: T);
    fn write_now(&self, message: T);

    /// Prints formatted output
    fn write<G: Any + Debug>(&self, value: &G) {
        let value_any = value as &Any;

        match value_any.downcast_ref::<Vec<u8>>() {
            Some(ref string) => {
                // print the output without the final newline character
                let mut resp = String::from_utf8(string.to_vec()).unwrap();
                let len: usize = resp.len();
                resp.truncate(len - 2);

                println!("{}", resp);
            }
            None => {
                // pass
            }
        }

        match value_any.downcast_ref::<String>() {
            Some(string) => {
                println!("{}", string);
            }
            None => {
                // pass
            }
        }
    }
}

pub trait TerminalDefaultWriter<T> {
    fn write_generic(&self, message: &T);
}

pub struct TerminalLine<T>;
pub struct TerminalBanner<T>;
pub struct TerminalCustomBanner<T>;


impl <T> TerminalMessageWriter<T> for TerminalLine<T> {
    fn write_generic(&self, message: T) {
        self.write(&message);
    }

    fn write_now(&self, message: T) {
        let dt = Local::now();
        let fmt_message: T = format!("\n{}/{}/{} - {}", dt.month(), dt.day(), dt.year(), message);

        self.write_generic(fmt_message);
    }
}

impl <T> TerminalDefaultWriter<T> for TerminalBanner<T> {
    fn write_generic(&self, message: T) {
        TerminalCustomBanner::<T>.write_generic(message);
    }
}

impl <T> TerminalMessageWriter<T> for TerminalCustomBanner<T> {
    fn write_generic(&self, message: T) {
        let default_banner: T = format!("=====================\n{}\n=====================", message);

        self.write(&default_banner);
    }

    fn write_now(&self, message: T) {
        let date = Local::now();
        let default_banner: T = format!("=====================\n{}\n{}\n=====================", date, message);

        self.write(&default_banner);
    }
}
