// Rust Inheritance and Method Resolution

/*
    There is no concept of inheritance in Rust, but to implement OOP in Rust,
    we have structs, implementations, and traits.
*/

use std::{collections::btree_map::Values, vec};

pub trait Role {
    // For traits, you don't need to define 'pub' in front of the function because
    // in traits, all methods take access modifiers of the trait itself.

    fn get_role_with_no_return_or_block(&self);
    fn get_role_with_no_block(&self) -> ();
    fn get_role_return_and_block(&self) -> () {
        // This type of implementation means that this function is optional.
        // This code block will execute by default if this is not overridden.
    }
}

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Human {
    pub fn fly(&self) {
        println!("Human flying");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot flying");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard flying");
    }
}

// Human has the same 'fly' method coming from three different implementations.

pub fn understanding_rust_inheritance() {
    let human = Human;
    human.fly(); // Run the direct implementation
    Pilot::fly(&human);
    Wizard::fly(&human);

    // Or you can use fully qualified syntax
    <Human as Wizard>::fly(&human);
}

// Rust Vectors

// Rust vectors are a special data type, essentially smart pointers, but you don't need to understand smart pointers right now.
// Vectors are a special type of array that doesn't require a static size because elements are stored in heap memory, and their
// reference is stored in the stack.

// We use vectors when the size of the array is unknown at compile time.

pub fn understanding_rust_vec() {
    // Vectors can be defined in two ways
    let mut vec_one: Vec<i32> = Vec::new(); // Using the module
    let vec_two: Vec<bool> = vec![]; // Using the macro
                                     // The type of a vector can be inferred directly from the first element in the vector, otherwise, it needs to be specified.

    // Pushing values into the vector
    vec_one.push(0);
    vec_one.push(1);
    vec_one.push(2);

    // Checking if a vector is empty
    let is_empty = vec_one.is_empty();
    println!("Is empty: {}", is_empty);

    // Getting an element from the array using indexing
    println!("{}", vec_one[0]);
    let value_at_zero_index = vec_one.get(0); // Returns an Option type

    // Getting the length of the vector
    println!("Length: {}", vec_one.len());

    // Iterating over the array elements by converting the vector into a slice
    for element in vec_one.as_slice() {
        // Iterating over elements using a slice
    }

    // Iterating over the array using indexing
    for index in 0..vec_one.len() {
        // Iterating over an array using indexing
    }

    // Iterating over the vector using the iterator
    for element in vec_one.iter() {
        // 'element' is a reference and will be automatically dereferenced into i32 because of deref coercion
    }

    // Get the iterator for a vector
    let mut vec_two_iter = vec_two.into_iter();
    vec_two_iter.next();

    // Additional Vec useful functions

    vec_one.capacity(); // The number of elements an array can store without reallocating
    vec_one.reverse();
    vec_one.sort();
    vec_one.insert(0, 1);
    let does_contain = 1;
    vec_one.contains(&does_contain);
    let array_sliced = &vec_one[..];

    let new_slice = [1, 2];
    vec_one.copy_from_slice(&new_slice); // It will copy elements from another slice
    vec_one.clone_from_slice(&new_slice); // Clone elements from another slice

    // There are other functions that you can explore

    // Popping the last value
    vec_one.pop(); // Gives you an Option type

    // You can use it with a while loop to iterate over the array until the Option value is None
    while let Some(value) = vec_one.pop() {}

    // You can also use it in an if statement
    if let Some(value) = vec_one.pop() {}
}
