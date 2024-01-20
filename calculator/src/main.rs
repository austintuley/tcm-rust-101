#![allow(unused)]

use std::io;

fn main() {

    println!("Enter the first number: ");
    let mut first_num = String::new();
    io::stdin().read_line(&mut first_num);

    println!("Enter the second number: ");
    let mut second_num = String::new();
    io::stdin().read_line(&mut second_num);

    println!("Enter the sign for your calculation: ");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator);

    let mut first: i32 = first_num.trim().parse().expect("\nEntry for first number was not a valid integer.\n");
    let mut second: i32 = second_num.trim().parse().expect("\nEntry for second numnber was not a valid integer.\n");
    let mut first_float: f64 = first as f64;
    let mut second_float: f64 = second as f64;
    let mut operator_slice = operator.trim();

    if (operator_slice == "+"){
        let result = addition(first, second);
        println!("{} {} {} = {}", first, operator_slice, second, result)
    }
    else if (operator_slice == "-"){
        let result = subtraction(first, second);
        println!("{} {} {} = {}", first, operator_slice, second, result)
    }
    else if (operator_slice == "*"){
        let result = multiplication(first, second);
        println!("{} {} {} = {}", first, operator_slice, second, result)
    }
    else if (operator_slice == "/"){
        let result = division(first_float, second_float);
        println!("{} {} {} = {}", first, operator_slice, second, result)
    }
    else{
        println!("You must enter a valid sign for calculation.")
    }

}

fn addition(x: i32, y: i32) -> i32 {
    x + y
}
fn subtraction(x: i32, y: i32) -> i32 {
    x - y
}
fn multiplication(x: i32, y: i32) -> i32 {
    x * y
}
fn division(x: f64, y: f64) -> f64 {
    x / y
}