// declaring varibles in rust

pub fn variable_in_rust() {
    // A variable can be used to store data or a pointer to a specific memory location in Rust.
    // Variables can be defined and initialized using the 'let' or 'const' keyword.
    // Variables are immutable by default and can be made mutable by adding the 'mut' keyword.
    // (There is a smart pointer type that does not require explicit initialization with the 'mut' keyword, which will be discussed later.)

    /* Important:
        Rust behaves differently for data stored in stack memory and data stored in heap memory,
        as specified by ownership and borrow rules, which we will explore later.
    */

    /*  PRO TIP:
        Whenever you write code in any language, keep memory in mind. Imagine memory as an address used to store something,
        like your home address. While most languages have garbage collectors, Rust uses an ownership model for memory management,
        providing more control to developers than garbage collectors. Memory is easy; just don't overreact!
        It will make things easier almost everywhere. 😅
    */

    let a: &str = "script_saga"; // &str is a string slice stored directly in the program binary
    let a = "script_saga"; // You don't need to specify a type explicitly; the Rust compiler is smart enough. However, there are cases where you need to specify the type explicitly.

    let a = b"script_saga"; // Redefining a variable with the same name is called shadowing, and it shadows the previous declaration.

    let arg_bool = true;
    let arg_int = 32; // Read the documentation for all integer types and choose according to the required memory.

    let arg_float = 69.9; // Read the documentation for all float types and choose according to the required memory.

    let pointer_arg = &arg_bool; // A pointer to some memory.

    const MY_CONSTANT: &'static str = "script_saga"; // Constants require the type to be explicitly defined, and the convention is to use capitalized letters.

    let mut arg_mutable = false;
    arg_mutable = true;

    // Will discuss complex types in some other snapshot.
}

// Understanding Rust ownership in a simpler way.

/*
    When a program is executed, it has two types of memory: stack and heap.
    Working with stack memory is faster compared to heap memory because variables
    are stored in memory at compile time in an ordered manner, whereas data in heap memory
    is stored in an unordered manner. Therefore, the allocator needs to do some work to find a space
    to store a particular data and keep a book tracking it. That's why pushing data into
    stack memory is faster than allocating data in the heap.

    Ownership rules:
    1. Each value in Rust has an owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.

    The data whose size can't be known at compile time is called dynamic data and is stored
    in heap memory, while the data whose size can be known at compile time is stored in stack memory.
    So when you reassign a variable to another variable, Rust first checks if the data is sized or unsized.
    If it is sized, Rust simply copies and pushes it onto the stack. If it's unsized data,
    Rust transfers the ownership to the new variable, and the other variable becomes invalid.

    Ownership can be transferred to other variables, functions, as return values from functions, threads, closures,
    and if you don't want to transfer ownership, you borrow. We'll look into that in the next segment.
*/

pub fn understand_ownership() {
    let a: u32 = 4;
    let b = a; // Its size is known at compile time, and hence it is copied and stored in the stack

    let unsized_data = String::from("ScriptSaga");
    let get_ownership = unsized_data; // This is unsized data, and then the ownership is transferred to a new variable, and unsized data is not valid anymore
    
}
