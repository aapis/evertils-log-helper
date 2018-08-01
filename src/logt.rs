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

    fn err(error: io::Error) {
        println!("error: {}", error.to_string());
    }

    fn print_output<T: Any + Debug>(value: &T) {
        let value_any = value as &Any;

        match value_any.downcast_ref::<Vec<u8>>() {
            Some(ref string) => {
                let resp = String::from_utf8(string.to_vec()).unwrap();
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

    fn exec(args: Vec<String>) {
        let message: String = args[1].to_string();

        println!("Task number?");

        // reference to the job number
        let mut job_number = String::new();

        // gets input and passes it to evertils
        match io::stdin().read_line(&mut job_number) {
            Ok(n) => success(n, job_number, message),
            Err(error) => err(error),
        }
    }

    pub fn new() {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            exec(args);
        } else {
            let err_message: String = "Not enough args, 1 required".to_string();
            print_output(&err_message);
        }
    }
}

fn main() {
    logt::new();
}