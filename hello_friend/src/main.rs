#![allow(unused)]

const BIRTHDAY:i32 = 1;

fn main() {
/*     // println!("Hello, world!");
    // println!("hello, friend")
    let mut hello:&str = "Hello, friend.";
    println!("{}", hello);
    hello = "Goodbye, friend.";
    println!("{}", hello);

    let x: i32 = 6;
    let mut y: i32 = 6;

    // Constants are immuntable and global
    const NUMBER: i32 = 6;
    println!("{} + {} + {} = {}", x, y, NUMBER, x + y + NUMBER);

    y = 12;
    println!("{} + {} + {} = {}", x, y, NUMBER, x + y + NUMBER);

    let z = 1;
    let z:&str = "Hello again, friend.";
    println!("{}", z);

    let q = 42_u32;
    let r = 1_000_000_u32;
    println!("{}", q);
    println!("{}", r); */

    let my_name:&str = "Austin";
    let my_birthday:&str = "October 4th";
    let mut age = 37;
    let new_age = age + BIRTHDAY;

    println!("My name is {} and I am {} years old. I will turn {} on {}.", my_name, age, new_age, my_birthday);
}
