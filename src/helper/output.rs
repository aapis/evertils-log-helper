use std::any::Any;
use std::fmt::Debug;

/// Prints formatted output
pub fn print<T: Any + Debug>(value: &T) {
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