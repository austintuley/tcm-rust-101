#![allow(unused)]

use std::io;
use std::ops::Add;

fn main() {
    // Vector - similar to array
    // slower than arrays but more flexible
    let mut vec1 = Vec::new();
    let mut vec2 = vec![1, 2, 3];

    vec1.push(1);
    vec2.push(4);
    vec2.push(5);
    vec2.push(6);

    let second_element = vec2[1];
    println!("The second element in vec2 is {}.", second_element);
    println!("The length of vec2 is {}.", vec2.len());

    for element in vec2.iter() {
        println!("Element: {}", element);
    }

    // Structures
    struct Car{
        make: String,
        model: String,
        year: u32,
        price: f64,
    }

    let mut huracan = Car{
        make: String::from("Lamborghini"),
        model: String::from("Huracan"),
        year: 2020,
        price: 320000.00,
    };

    println!("The cost of a {} {} {} is ${}.", huracan.year, huracan.make, huracan.model, huracan.price);

    struct Rectangle{
        width: u32,
        height: u32,
    }

    // implement feature helps build a function 
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle {width: 30, height: 50};
    let area = rect.area();
    println!("The area of our triangle is {}.", area);

    // Enumeration or enum
    // Define a set of named values

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    #[derive(Debug)]
    enum Shape {
        Cirlce(f32),
        Rectangle(f32, f32),
    }

    let circle = Shape::Cirlce(10.0);
    let rectangle = Shape::Rectangle(30.0, 40.0);
    println!("{:?}", circle);

    // Generics
    // added std::ops::Add above main function

    fn sum<T:Add<Output = T>> (a: T, b: T,) -> T {
        a + b
    }

    let x = sum(1, 2);
    let y = sum(2.3, 3.5);
    println!("The value of x is {}.", x);
    println!("The value of y is {}.", y);
    println!("The sum of 3 + 2 = {}.", sum(3,2));

    struct Items<T> {
        x: T,
        y: T,
    }

    let i = Items {x: 1.0, y: 2.0};
    println!("{}, {}", i.x, i.y);

}

