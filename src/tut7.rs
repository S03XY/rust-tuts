// understanding pattern matching

use std::fmt::Error;

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
}

pub fn something_that_returns_result_type(some_condition: bool) -> Result<i32, Error> {
    if !some_condition {
        panic!("fmt error");
        // return Err(0);
    }
    Ok(1)
}
