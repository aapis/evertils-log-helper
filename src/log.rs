mod log {
    use std::env;
    use std::process::Command;

    pub fn exec() {
        let args: Vec<String> = env::args().collect();
        let message = &args[1];

        let output = Command::new("sh")
                .arg("-c")
                .arg(format!("evertils log message {}", message))
                .output()
                .expect("failed to execute");

        let _ = output.stdout;
    }
}

fn main() {
    log::exec();
}