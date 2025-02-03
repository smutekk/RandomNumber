extern crate core;

use std::io;
use rand::{Rng};

fn main() {
    println!("Maximum number to generate?");
    let mut max_num = String::new();

    io::stdin()
        .read_line(&mut max_num)
        .expect("Failed to read line");

    let max_int: i32 = max_num.trim().parse().expect("Not a valid number");

    let mut random_num = rand::thread_rng().gen_range(0..max_int);

    println!("Maximum number to generate is: {}", random_num);

}
