// understanding pattern matching

use std::{array, fmt::Error, vec};

// A VERY IMPORTANT THING IN RUST
pub fn understanding_pattern_matching() {
    // Pattern matching is done via match expressions in Rust.
    // Pattern matching on enum variants.

    enum ScriptSaga {
        ADMIN,
        DEVELOPER,
        USER,
        DESIGNER,
    }

    let script_saga_enum = ScriptSaga::ADMIN;

    // You can consider a match statement something similar to the switch statement from other languages, but
    // match expressions are more versatile and powerful than switch expressions.
    // Match expressions are known as match arms in Rust.
    // Each matching arm should return the same type, which is a must if you want to return something.
    match script_saga_enum {
        ScriptSaga::ADMIN => {
            // Execute admin code.
        }
        ScriptSaga::DESIGNER => {
            // Execute developer code.
        }
        ScriptSaga::USER => {
            // Execute user code.
        }
        _ => {
            // Execute default code.
            // Also note here that no input is bounded here.
        }
    }

    // Error handling using match expressions.
    // Error handling can be done in other ways as well.
    match something_that_returns_result_type(true) {
        Ok(value) => {
            // Execute code if everything is correct.
        }
        Err(_value) => {
            // Execute if something goes wrong.
        }
    };

    // If you don't have the same matching arm output type, the compiler will give an error.
    let value = match something_that_returns_result_type(true) {
        Ok(value) => {
            // Execute code if everything is correct.
            // Don't use the return keyword here because it will try to return from the function scope.
            value
        }
        Err(_value) => {
            // Execute if something goes wrong.
            0
        }
    };

    println!("value is {}", value);

    // You can match multiple expressions as well.
    let statement_one: i32 = 0;
    let statement_two: bool = true;

    let (res1, res2) = match (statement_one, statement_two) {
        (0, true) => (0, 1), // You can omit {} if the code block is of 1 line only.
        (_, _) => (0, 1),
    };

    // Match using if let syntax
    // The if let syntax in match is used when you are concerned about a specific case
    // and don't care about other cases

    let value = Some(5);
    // I am only interested in the OK value, so I can use something like this

    if let Some(val) = value {
        // Execute some code using val
        println!("{}", val);
    }
}

pub fn something_that_returns_result_type(some_condition: bool) -> Result<i32, Error> {
    if !some_condition {
        panic!("fmt error");
        // return Err(0);
    }
    Ok(1)
}

// rust destructuring and pattern matching

pub struct Point(i32, i32, i32);
pub struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

pub fn understanding_rust_destructuring_with_pattern_matching() {
    // you can define multiple variable in none line using pattern matching
    let arr_1: [i32; 2] = [1, 2];
    let (a, mut b, c) = (1, 2, arr_1);
    // you cannot destructure and array or vector with pattern matching but tuple can be
    // let (a,b) = arr_1;  this is wrong

    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin; // remember to use parantheses not curly brackets
                                 // while destructuring tuple struct

    // for key value struct use curly brackets
    let coordinate_one = Coordinate { x: 0, y: 0, z: 0 };
    let Coordinate { x, y, z } = coordinate_one;

    // using with match expressions

    match origin {
        Point(x, _, _) => {
            // excute code related to x coordinate
        }
        Point(x, y, _) => {
            // execute code related to x and y cordinate
        }
        Point(x, y, z) => {
            // execute code related to x,y and z coordinate
        }
        _ => {
            // NOTE: THIS IS UREACHABLE CODE YOU DONT NEED TO USE IT BECAUSE ALL THE ARMS ARE ALREADY SATISFYING
            // ALL THE POSSIBLE CODITION origin CAN TAKE
        }
    }

    // usign match guards

    // match coordinate_one {
    //     Coordinate { x } => {}
    //     Coordinate { x, y, .. } => {}
    //     Coordinate { x, y, z } => {}
    // }
}
