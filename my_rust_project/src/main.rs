#![allow(unused)]

fn main() {
    // Compound Types: Tuples and Arrays
    //Tuples - () - store up to 12 values
    let student_a = ("Austin", 'A', 3.66);
    // let name_student_a = student_a.0;
    let (name_student_a, grade_student_a, gpa_student_a) = student_a;
    println!("{}", name_student_a);
    println!("My name is {}. My grade is {} with a grade point average of {}.", name_student_a, grade_student_a, gpa_student_a);

    //Arrays - [] - store up to 32 values, same data types
    let students = ["Austin", "Bob", "Mike", "Tim"];
    println!("The first student in our array is {}.", students[0]);
    let last_student = &students[students.len() -1];
    println!("The last student in our array is {}.", last_student);

    //Slices
    let num_array = [1, 2, 3, 4, 5];
    let slice = &num_array[1..3];
    println!("{:?}", slice);

    let mut next_array = [6, 7, 8, 9, 10];
    println!("{:?}", next_array);
    let next_slice = &mut next_array[2..4];
    next_slice[0] = 50;
    next_slice[1] = 100;
    println!("{:?}", next_array);

    //String - several types
    //Likely will only use two types; String and &str
    //str - string slice, &str - borrowed string slice - cannot be modified < < |
    //String - data can be modified
    //&str - essentially a subset of a String - *static

    let name = "Austin".to_string();


    let mut name2 = String::new(); //because you cannot modify &str - ^ see above ^
    name2.push_str("AuStin");
    name2.push_str(" test");
    name2.push_str(" take 3");
    println!("{} {}", name, name2);

    let mut word = String::from("Austin");
    println!("{}", word);
    word.push_str(" is learning");
    word.push_str(" rust language!");
    println!("{}", word);

    //Character Escaping
    println!("Hello again, friend.");

    println!("Hello \
    again, friend.\n");

    println!("He said, \"All is fair in love and war.\"");

    let q = String::from("This is ");
    let r = String::from("the end ");
    let s = String::from("for you my friend.");
    println!("{}{}{}", q, r, s);
    print!("{}", concat!("Fare ", "thee ", "well, ", "my friend."));

}
