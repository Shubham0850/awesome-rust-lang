
/*
	A struct in Rust is a user-defined data type that allows you to create custom data structures by grouping together mulitple values with different types into a single entity. It provides a way to define your own data structure that encapsulates related data and behaviours.

	

 */

// Define a struct named `Person` to represent a person's information
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

fn main() {
    // Create an instance of the `Person` struct
    let person1 = Person {
        name: String::from("Alice"),
        age: 25,
        is_student: true,
    };

    // Access struct fields
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("Is Student: {}", person1.is_student);

    // Create another instance of the `Person` struct
    let person2 = Person {
        name: String::from("Bob"),
        age: 30,
        is_student: false,
    };

    // Access struct fields
    println!("Name: {}", person2.name);
    println!("Age: {}", person2.age);
    println!("Is Student: {}", person2.is_student);
}
