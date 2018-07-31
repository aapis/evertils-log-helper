mod logt {
    use std::env;
    use std::process::Command;
    use std::io;
    use std::string::String;

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


            self::print_output<std::process::Output>(output.stdout);
        }
    }

    fn error(error: io::Error) {
        println!("error: {}", error);
    }

    fn print_output<T>(output: T) {
        let resp = String::from_utf8(output.stdout).unwrap();

        println!("{}", resp);
    }

    pub fn exec() {
        let args: Vec<String> = env::args().collect();
        let message = &args[1];

        println!("Task number?");

        // reference to the job number
        let mut job_number = String::new();

        // gets input and passes it to evertils
        match io::stdin().read_line(&mut job_number) {
            Ok(n) => self::success(n, job_number, message.to_string()),
            Err(error) => self::error(error),
        }
    }
}

fn main() {
    logt::exec();
}