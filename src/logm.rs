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
        let writer: Line = Line;
        writer.write_now(message.to_string());

        let vec_out: TerminalLine<Vec<u8>> = TerminalLine;
        vec_out.write_generic(output.stdout);
    }
}

fn main() {
    log::exec();
}