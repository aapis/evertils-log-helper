// start a new day
extern crate chrono;

mod writer;
mod output;

mod newday {
    use std::env;
    use std::process::Command;
    use writer::{Banner, DefaultWriter};

    /// Append the required text to the log file
    fn exec() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("evertils generate morning")
            .output()
            .expect("failed to execute");

        let writer: Banner = Banner;
        writer.write_generic();
    }

    pub fn new() {
        let args: Vec<String> = env::args().collect();

        if args.len() > 0 {
            exec();
        } else {
            println!("This command takes no arguments");
        }
    }
}

fn main() {
    newday::new();
}