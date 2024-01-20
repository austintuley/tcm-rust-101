#![allow(unused)]

use std::io;

// FIRST trait, struct, and impl run together with top portion under main function
trait Damage {
    fn damage(self: &mut Self);
}

#[derive(Debug)]
struct HP {
    hp_remaining: i32,
}

impl Damage for HP {
    fn damage(self: &mut Self) {
        self.hp_remaining -= 1;
    }
}

// SECOND run using traits
trait  Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with a radius of {}.", self.radius);
    }
}

// Using a generic to implement trait
fn draw_shape<T: Drawable>(shape: &T) {
    shape.draw();
}

fn main() {
    // Traits
    // A group of methods that are defined for a particular type.

    let mut hp = HP {hp_remaining: 100};
    hp.damage();
    hp.damage();
    hp.damage();
    hp.damage();
    hp.damage();
    hp.damage();
    hp.damage();
    hp.damage();
    hp.damage();
    hp.damage();
    println!("You took damage! HP remaining: {:?}", hp);

    let circle = Circle { radius: 10.0 };
    circle.draw();
    let second_circle = Circle { radius: 20.0 };
    draw_shape(&second_circle);
}
