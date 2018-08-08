// log monitoring items
extern crate chrono;

mod output;
mod writer;

mod log {
    use std::env;
    use std::process::Command;
    use writer::{Line, MessageWriter};
    use output::{TerminalLine, TerminalMessageWriter};

    pub fn exec() {
        let args: Vec<String> = env::args().collect();
        let message = &args[1];
        let output = Command::new("sh")
                .arg("-c")
                .arg(format!("evertils log message \"monitoring - {}\"", message))
                .output()
                .expect("failed to execute");

        // TODO: this prints on 2 lines, should only print on one
        // let rlog_msg: String = format!("{} - {}", job_number, message);
        // make sure the data is appended to the rolling log
        let writer: Line = Line { message: message.to_string() };
        writer.write_now();

        let output: TerminalLine = TerminalLine { message: &output.stdout };
        output.write_generic(&output.stdout);
    }
}

fn main() {
    log::exec();
}