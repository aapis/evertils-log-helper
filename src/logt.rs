// log task-oriented items
extern crate chrono;

mod writer;
mod output;

mod logt {
    use std::env;
    use std::process::Command;
    use std::io;
    use writer::{Line, MessageWriter};
    use output;

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
            // make sure the data is appended to the rolling log
            let writer: Line = Line { message: message };
            writer.write_now();

            output::print(&output.stdout);
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
            Ok(jn_size) => success(jn_size, job_number, message),
            Err(error) => println!("error: {:?}", error),
        }
    }

    /// Create a new instance of logt
    pub fn new() {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            exec(args);
        } else {
            let err_message: String = "Not enough args, 1 required".to_owned();
            output::print(&err_message);
        }
    }
}

fn main() {
    logt::new();
}