#![allow(unused)]

use std::io;

// In Rust, memory is managed through a system of ownership and borrowing.
// Each value in Rust has an owner,
// which is responsible for managing the memory used by that value.
// When a value goes out of scope, its memory is automatically freed.
// This eliminates the need for manual memory management or garbage collection,
// which can lead to bugs, performance issues, and security vulnerabilities. 

// Stack vs Heap
// Stack is fast. Values are stored in order and all are fix-sized. Uses LIFO.
// Heap is slow. Values are unordered and of a variable size.
// The heap uses a return address for requested space called a pointer. 

// Ownership - has 3 Rules
// Each value has an owner (owned by a variable)
// There can only be one owner at a time
// When the owner goes out of scope, the memory becomes free

fn main() {
    let name = String::from("Austin");
    // let new_name = name; without clone like below name is no longer available for first println! statement
    let new_name = name.clone();
    println!("Hello, my name is {}.", name);
    println!("Hello again, my name is {}.", new_name);

    // Reference - &
    // Borrowing has to match the mutability.

    let a = String::from("Austin");
    let b = &a;

    // println!("{}", a);
    // println!("{}", b);
    println!("My name is {}.", *b); // * deref

    let s1 = String::from("Hello"); // owner is s1
    let len = calculate_length(&s1); // pass a reference to s1 to the function
    println!("The length of {} is {}.", s1, len);

    let x = 10;
    let ptr = &x;
    println!("{}", *ptr); // just a pointer (points to x which has a pointer that points to the heap) just a reference

    let mut y = 11;
    let z = &mut y;
    println!("{}", *z);
    *z += 1;
    println!("{}", y);

}

fn calculate_length(s: &String) -> usize { // s is a reference to a String value
    s.len() // return the length of the string pointed to by s
} // s goes out of scope
