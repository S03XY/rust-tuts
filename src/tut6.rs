// Understanding control flow in Rust

pub fn understanding_if_else() {
    // if-else statements are used when you want to execute a certain functionality that depends
    // on some condition

    let some_condition: bool = true;
    let some_other_condition: bool = true;

    if some_condition {
        // execute code that needs to be executed if the condition is true
    } else {
        // execute code that needs to be executed if the condition is false
    }

    // Multiple conditions
    if some_condition && some_other_condition {
        // execute code when both sides of the && condition are true
    }
    if some_condition || some_other_condition {
        // execute code when one side of the || condition is true
    }

    // if-else if-else

    if some_condition {
        // execute code that needs to be executed if the condition is true
    } else if some_other_condition {
        // execute code that needs to be executed if the condition is false
        // if there are other else-if conditions, then the code will check for all the else-if conditions
        // until the condition is met, and if it is not met, then the code in the else block will be executed
    } else {
        // execute code that needs to be executed if none of the conditions match
    }

    // Use of if-else for initialization
    let args: i32 = if some_condition { 0 } else { 1 };

    /*
       Note: Unlike other programming languages, like JavaScript or Python, where if-else can be used with null,
       undefined, or empty values, here in Rust, the if condition can only work with boolean condition inputs.

       This is wrong => if None { some condition }
    */
}

// understanding loops in rust

pub fn understanding_loops() {
    // This type of for loop is used as a range-based for loop where the last index of <start_index>..<last_index>
    // is exclusive.
    for i in 0..10 {
        println!("working on index {}", i);
    }

    // Conventional while loop which runs a block until a condition becomes false.
    let mut x = 0;
    while x < 100 {
        print!("value of x {}", x);
        x += 1;
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for number in a {
        println!("getting the value, not reference {}", number);
    }

    // Types that implement an Iterator trait need to define the next function to get the next value.
    // Rust developers love iterators.
    // Iterator is a way to iterate over any type of collection.
    // There are a few benefits of iterators which we discuss in some other snap.
    for number in a.iter() {
        println!("current value reference is {}", number); // Remember in Rust, dereferences happen automatically for types that
                                                           // implement a Deref trait.
    }
}
