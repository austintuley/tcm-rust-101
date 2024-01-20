/*Build calculator that takes two user inputs
  then calculates addition, subtraction,
  multiplication, and division of both inputs.*/  

#![allow(unused)]
use core::num;
use std::io;

fn main() {
    println!("Provide your 1st number: ");
    let mut num_1 = String::new();
    io::stdin().read_line(&mut num_1);

    println!("Provide your 2nd number: ");
    let mut num_2 = String::new();
    io::stdin().read_line(&mut num_2);

    let num_1: i32 = num_1.trim().parse().expect("Entry was not an integer.");
    let num_2: i32 = num_2.trim().parse().expect("Entry was not an integer.");
    
    let num_1_float = num_1 as f64;
    let num_2_float = num_2 as f64;

    println!("{} + {} = {}", num_1, num_2, num_1 + num_2);
    println!("{} - {} = {}", num_1, num_2, num_1 - num_2);
    println!("{} * {} = {}", num_1, num_2, num_1 * num_2);
    println!("{} / {} = {}", num_1, num_2, num_1_float / num_2_float);
}
