// esult and Option are two special type of Enum that comes with the default std library
// Result capture the error and correct value whereas Option is used to define some type and None

/*
    pub enum Result<T, E> {
    Ok(T),
    Err(E),
    }

    pub enum Option<T> {
    None,
    Some(T),
    }
    T and E are generic types dont worry about them right now will discuss it later
*/

// here is how you can use it with functions

use std::{io::ErrorKind, fmt::Error};
pub fn function_return_a_result() -> Result<i32, String> {
    // code goes here...

    let condition = true;

    if condition {
        Ok(1)
    } else {
        Err(String::from("value"))
    }
}

// here is how to use Option as return in function
pub fn function_return_an_option() -> Option<i32> {
    // code goes here...

    let condition = true;

    if condition {
        Some(1)
    } else {
        None
    }
}
