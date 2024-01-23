// Understanding advanced pattern matching

pub fn understanding_advanced_pattern_matching() {

    let number = 0;

    // Remember, match expressions are exhaustive and require the sll the conditions that the value can take to be specified.
    // Range/or based conditioning match arms
    // In range-based arms, the end is inclusive in <start>..<end>

    match number {
        0..=10 => {
            println!("Value is in between the range 1 and 10");
        }

        11..=20 => {
            println!("Value is in between the range 10 and 20");
        }

        21 | 22 | 23 => {
            println!("Value is either 21, 22, or 23");
        }

        _ => {
            // Execute default code for other values in 'number'
        }
    }


    let number = 0;
    // Match expression with match guards
    match number {
        n if n >= 1 && n <= 10 && n % 2 == 0 => {
            // Execute code if the number is between 1 and 10 and is even
        }
        n if n >= 11 && n <= 20 && n % 2 != 0 => {
            // Execute code if the number is between 11 and 20 and is odd
        }
        _ => {}
    }

    struct Coordinate {
        x: i32,
        y: i32,
        z: i32,
    }

    let point1 = Coordinate { x: 1, y: 1, z: 1 };

    // Destructuring and pattern matching with match guards
    match point1 {
        Coordinate { x, y, z } if x > 5 && y > 5 && z > 5 => {
            // Execute code for x, y, and z values greater than 5
        }
        Coordinate { x, y, z } if x < 5 && y < 5 && z > 5 => {
            // Execute code for x and y less than 5 and z greater than 5
        }
        _ => {
            // Execute default code in which none of the above arms matches
        }
    }

    // Using @ operator in pattern matching
    // @ operator is used when we want to store a value inside a variable and match some pattern against that variable
    // @ is used for nested pattern matching
    // key: <custom_name> @ possible matching pattern for custom_name
    // Pattern matching with nested struct and using @ operator

    enum ScriptSagaFollower {
        FOLLOWER { follower_count: i32 },
    }

    let script_saga_follower = ScriptSagaFollower::FOLLOWER { follower_count: 5 };

    match script_saga_follower {
        ScriptSagaFollower::FOLLOWER {
            follower_count: count @ 1..=10,
        } => {
            // If you want to give an alias name to follower_count and have a pattern matched againt it
            println!("Follower count is {} and is less between 1 to 10", count)
        }
        ScriptSagaFollower::FOLLOWER {
            follower_count: 11..=20,
        } => {
            // If you want to do pattern matching inside the nested struct but don't care about the variable
            // Remember if you do pattern matching like this, the follower_count variable will not be available
            // outside like in the third arm
            println!("Follower count is less between 11 to 20");
        }
        ScriptSagaFollower::FOLLOWER { follower_count } if follower_count > 20 => {
            println!("Follower count is more than 20")
        }
        _ => {
            // Execute default code in which none of the above arms matches
        }
    }
}
