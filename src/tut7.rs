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

/*
    Before understanding advanced pattern matching, you need to understand REFUTABILITY AND IRREFUTABILITY.

    While pattern matching, you need to take care of the type of values an expression can possibly have.
    Patterns in which the pattern can take all possible values of an expression are known as irrefutable patterns,
    whereas patterns that don't care about all the possible values of an expression are known as refutable patterns.

    An example of an irrefutable pattern is let a = 5;
    Remember that function parameters, for loops, and let statements take only irrefutable patterns.

    An example of a refutable pattern is when you use if let or while let syntax:

    if let Some(value) = option_args {
        // Execute some code.
    }

    while let Some(value) = vec_args.pop() {
        // Execute some code; this code is going to execute until the value is not None.
    }
*/

// Rust destructuring and pattern matching

pub struct Point(i32, i32, i32);
pub struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

pub fn understanding_rust_destructuring_with_pattern_matching() {
    let arr_1: [i32; 2] = [1, 2];
    // You can define multiple variables in one line using pattern matching.
    let (a, mut b, c) = (1, 2, arr_1);

    // You cannot destructure an array as shown above.
    // let (a, b) = arr_1;  // This is incorrect.
    // Instead, you can use the range operator.
    let [a, ..] = arr_1; // This is correct.

    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin; // Remember to use parentheses, not curly brackets,
                                 // while destructuring tuple structs.

    // For key-value structs, use curly brackets.
    let coordinate_one = Coordinate { x: 0, y: 0, z: 0 };
    let Coordinate { x, y, z } = coordinate_one;

    // You need to specify all the fields while destructuring or use _ or .. syntax.
    // Use .. syntax for struct named identifiers {Key Value Struct} and use _ for unnamed identifiers (Point Struct),
    // else the compiler will complain.
    // let Coordinate {x} = coordinate_one; This is wrong.

    let a = vec![1, 2, 3, 4];

    let Point(x, _, _) = origin; // You can use Point(x, ..) syntax, but don't use it for uniformity in learning.
                                 // In general, you can use .. for any range-based type like struct, array, vec, etc.,
                                 // which don't have any named identifiers in them.

    let Coordinate { x, .. } = coordinate_one; // But here you cannot use Coordinate { x, _, _ } because it uses named identifiers.

    // Destructuring and pattern matching
    match origin {
        Point(x, _, _) => {
            // This type of syntax means that you are taking care of the patterns, but you only need
            // the x value out of it, and the rest of the arms will become unreachable or dead code.
        }

        Point(x, y, _) => {
            // This is unreachable code.
        }
    }

    // The same thing is true for the .. (range operator), i.e.,
    match origin {
        Point(x, ..) => {
            // Same as Point(x, _, _) as shown above.
        }
        Point(x, y, ..) => {
            // This is unreachable code.
        }
    }
}
