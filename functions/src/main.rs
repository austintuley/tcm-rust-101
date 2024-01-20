#![allow(unused)]

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //Functions - organized blocks of code
    who_am_i();
    add_one_hundred(100);
    println!("{}", multiplication_with_returned_value(33, 2));
    let (added, multiplied) = add_and_multiply(10, 10);
    println!("Added: {}", added);
    println!("Multiplied: {}", multiplied);
}

fn who_am_i(){
    let name = "Austin";
    let age = "37";
    println!("My name is {} and I am {} years old.", name, age);
}

fn add_one_hundred(num: i32){
    println!("{}", num + 100);
}

fn multiplication_with_returned_value(x: i32, y: i32) -> i32 {
    x * y
}

fn add_and_multiply(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}