#![allow(unused)]

use std::io::{self, stdin};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
/* 
    //if, else if, else statements
    println!("How much money do you have?");
    let mut input_money = String::new();
    io::stdin().read_line(&mut input_money);

    let money: i32 = input_money.trim().parse().expect("Entry must be an integer.");

    println!("How old are you?");
    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age);

    let age: i32 = input_age.trim().parse().expect("Entry must be an integer.");

    if (age >= 21) && (money >= 5) {
        println!("You may buy a drink.")
    }
    else if (age >= 21) && (money < 5) {
        println!("You need at least $5 to purchase a drink.")
    }
    else if (age < 21) && (money >= 5) {
        println!("You are not old enough to purchase a drink.")
    }
    else {
        println!("You are not old enough nor do you have enough money to purchase a drink.")
    }

    //match
    let candidacy_age = 37;

    match candidacy_age {
        1..=24 => println!("Cannot hold office."),
        25..=29 => println!("Can run for the House."),
        30..=34 => println!("Can run for the Senate."),
        35..=i32::MAX => println!("Can run for President."),
        _ => println!("Are you an infant?")
    }

    let my_age = 20;
    let drinking_age = 21;

    match my_age.cmp(&drinking_age){
        Ordering::Less => println!("Too young to drink."),
        Ordering::Equal => println!("Woo, you can drink!"),
        Ordering::Greater => println!("You may have a drink.")
    };
*/

    //Loops - for, while, infinite

    //for
    let mut fruits = ["strawberry", "banana", "apple", "orange", "kiwi", "pineapple"];
    for fruit in fruits.iter(){
        println!("{}", fruit)
    }

    //while
    let mut x = 1;
    while x < 11{
        println!("{}", x);
        x += 1;
    } 
    
    //infinite
    let mut y = 0;
    println!("Counting.....");
    loop{
        y += 1;
        println!("{}", y);
        if (y == 10){
            println!("We've reached 10!");
            continue;
        }
        if (y == 20){
            println!("We've reached 20. Exiting program.");
            break;
        }
    }



}
