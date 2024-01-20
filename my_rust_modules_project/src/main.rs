//Allowing unused variables during learning and practice stages
#![allow(unused)]

use std::io;
use rand::Rng; //Had to first add to Cargo.toml under dependencies

fn main() {
    println!("Who goes there?");
    let mut name = String::new();
    io::stdin().read_line(&mut name);
    let enter = String::from("You may now enter.");
    //Added .trim_end() method because we have an automatic line break at end of String
    println!("Hello there, {}. {}", name.trim_end(), enter);

    //Math stuff
    let x = 10;
    let y = 3;
    let x_float = x as f64;
    let y_float = y as f64;
    let mut z = rand::thread_rng().gen_range(1..101);

    println!("{} + {} = {}", x, y, x + y);
    println!("{} - {} = {}", x, y, x - y);
    println!("{} * {} = {}", x, y, x * y);
    println!("{} / {} = {}", x, y, x_float / y_float);
    println!("{} % {} = {}", x, y, x % y);
    //Debugging suggested trying module below which works
    //println!("{} ^ {} = {}", x, y, i32::pow(x, y.try_into().unwrap()));
    //Passing unsigned variable is the more proper way
    //Had to comment the prior println! statement to prevent kernel panic, had signed and unsigned variable for the power of variable
    println!("{} ^ {} = {}", x, y, u32::pow(x, y));
    println!("{}", z);
    z = rand::thread_rng().gen_range(101..201);
    println!("{}", z);
}
