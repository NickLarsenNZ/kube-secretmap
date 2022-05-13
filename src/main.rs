use std::thread;
use std::time::Duration;


fn main() {
    println!("Sleeping...");
    loop {
        thread::sleep(Duration::from_secs(1));
    }

}
