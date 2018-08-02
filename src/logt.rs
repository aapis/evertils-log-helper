// log task-oriented items
extern crate chrono;

mod helper;

mod logt {
    use std::env;
    use std::process::Command;
    use std::io;
    use std::string::String;
    use std::any::Any;
    use std::fmt::Debug;
    use helper;

    /// Create a subshell and execute the command
    fn success(size: usize, job_number: String, message: String) {
        if size < 6 {
            let command_string: String = format!(
                "evertils log message \"{} - {}\"",
                job_number,
                message
            );

            let output = Command::new("sh")
                .arg("-c")
                .arg(command_string)
                .output()
                .expect("failed to execute");

            // TODO: this prints on 2 lines, should only print on one
            // let rlog_msg: String = format!("{} - {}", job_number, message);
            let rlog_msg: String = message;
            // make sure the data is appended to the rolling log
            helper::rolling_log::update(rlog_msg);

            print_output(&output.stdout);
        }
    }

    /// An error occurred, print a message
    fn err(error: io::Error) {
        println!("error: {}", error.to_string());
    }

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

    /// Get user input and funnel it to an output method
    fn exec(args: Vec<String>) {
        let message: String = args[1].to_owned();

        println!("Task number?");

        // reference to the job number
        let mut job_number = String::new();

        // gets input and passes it to evertils
        match io::stdin().read_line(&mut job_number) {
            Ok(n) => success(n, job_number, message),
            Err(error) => err(error),
        }
    }

    /// Create a new instance of logt
    pub fn new() {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            exec(args);
        } else {
            let err_message: String = "Not enough args, 1 required".to_owned();
            print_output(&err_message);
        }
    }
}

fn main() {
    logt::new();
}