// log monitoring items
extern crate chrono;

mod helper;

mod log {
    use std::env;
    use std::process::Command;
    use helper;

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
        let rlog_msg: String = message.to_owned();
        // make sure the data is appended to the rolling log
        // helper::rolling_log::write(rlog_msg);

        let _ = output.stdout;
    }
}

fn main() {
    log::exec();
}