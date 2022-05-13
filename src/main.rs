use std::io;
use std::sync::Arc;
use std::thread;
use std::time;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

fn main() {
    let mut number = String::new();
    io::stdin().read_line(&mut number).ok().expect("Invalid input");
    let mut x: i32;
    match number.trim().parse::<i32>() {
        Ok(num) => println!("Hello, {}!", num + 10),
        Err(..) => println!("You input {} was not an integer", number.trim())
    };
    let bar = ProgressBar::new(100);
    bar.set_style(ProgressStyle::default_bar().template("[{bar:40.green/blue}]").progress_chars("##-"));
    bar.set_position(10);
    thread::sleep(time::Duration::from_secs(1));
    bar.set_position(20);
    thread::sleep(time::Duration::from_secs(1));
    bar.finish();
}
