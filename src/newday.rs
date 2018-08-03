// start a new day
extern crate chrono;

mod helper;

mod newday {
    use std::env;
    use std::process::Command;
    use std::string::String;
    use std::any::Any;
    use std::fmt::Debug;
    use helper;

    /// Prints formatted output
    fn print_output<T: Any + Debug>(value: &T) {
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

    /// Append the required text to the log file
    fn exec() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("evertils generate morning")
            .output()
            .expect("failed to execute");

        helper::rolling_log::new_day();

        print_output(&output.stdout);
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