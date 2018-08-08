use std::any::Any;
use std::fmt::Debug;

pub trait TerminalMessageWriter {
    fn write_generic(&self, message: String);
    fn write_now(&self);

    /// Prints formatted output
    fn write<T: Any + Debug>(&self, value: &T) {
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

pub trait TerminalDefaultWriter {
    fn write_generic(&self);
}

pub struct TerminalLine {
    pub message: String
}

pub struct TerminalBanner;

pub struct TerminalCustomBanner {
    pub message: String
}

impl TerminalMessageWriter for TerminalLine {
    fn write_generic(&self, message: String) {
        self.write(message);
    }

    fn write_now(&self) {
        let dt = Local::now();
        let fmt_message: String = format!("\n{}/{}/{} - {}", dt.month(), dt.day(), dt.year(), self.message);

        self.write_generic(fmt_message);
    }
}

impl TerminalDefaultWriter for TerminalBanner {
    fn write_generic(&self) {
        TerminalCustomBanner { message: String::new() }.write_now();
    }
}

impl TerminalMessageWriter for TerminalCustomBanner {
    fn write_generic(&self, message: String) {
        let default_banner: String = format!("=====================\n{}\n=====================", message);

        self.write(default_banner);
    }

    fn write_now(&self) {
        let date = Local::now();
        let default_banner: String = format!("=====================\n{}\n=====================", date);

        self.write(default_banner);
    }
}
