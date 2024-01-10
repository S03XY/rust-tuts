// Understanding Rust Lifetimes

/*
   Rust lifetimes are a very simple concept. Just think of it this way: the lifetime of a variable is the scope within which
   the variable is valid.

   Since Rust variables can be referenced, the Rust compiler needs to know whether those references are valid or not,
   because nobody likes dangling references. The Rust compiler helps a developer prevent dangling references using
   a lifetime, as every reference in Rust requires a lifetime to be specified. The Rust compiler tries to assign
   a lifetime to the required references by following lifetime elision rules.

   The first rule is that the compiler assigns a lifetime parameter to each parameter that's a reference. In other words,
   a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);`, and a function with two parameters gets
   two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.

   The second rule is that if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime
   parameters: `fn foo<'a>(x: &'a i32) -> &'a i32;`.

   The third rule is that if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this
   is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to
   read and write because fewer symbols are necessary.

   If, after following the elision rules, the Rust compiler doesn't able to figure out the lifetime of all the variables, then you need to explicitly define all the lifetimes.

   As a general rule, you have to specify the lifetime wherever a reference is stored or transferred in or out from a function.
*/

pub fn understanding_lifetime() {
    // Lifetimes can be used everywhere when references are not present in the same scope.
    // Simple example:
    let a: i32 = 5;
    let b = &a; // Here, you don't need to specify the lifetime because the Rust compiler already knows the scope of the above variables.
}

pub fn function_without_lifetime_reference(arg1: &i32, arg2: &bool) {
    // In this function, the Rust compiler automatically assigns lifetimes to input variables,
    // similar to function_without_lifetime_reference(arg1: &'a i32, arg2: &'b bool), following
    // the lifetime elision rules.
}

pub fn function_with_lifetime_reference<'solana>(
    arg1: &'solana i32,
    arg2: &'solana bool,
    arg3: &'solana mut String,
) {
    // You can give any name to your lifetime but follow the alphabetic convention
}

pub fn function_with_one_param(arg1: &str) -> &str {
    // Follows the second rule, so its lifetime can be inferred by the compiler.
    arg1
}

// pub fn function_with_incorrect_return_ref(arg1: &str, arg2: &bool, arg3: &str) -> &str {
//     let a = "script_saga";
//     // a
//     &arg1
//     // The above code is wrong because the reference is destroyed after the function execution,
//     // so the Rust compiler gets confused. Here you need to specify the lifetime manually.
//     // Always remember the returning lifetime will be the smallest of all lifetimes if lifetimes are not specified,
//     // and hence the compiler gets confused.
// }

pub fn function_with_correct_return_ref<'a>(arg1: &'a str) -> &str {
    arg1
}

// Sample example

pub fn script_saga() {
    let arg1 = "script";
    {
        let arg2 = "saga";
        execute_life_time(&arg1, &arg2); // This is correct
    }

    // execute_life_time(&arg1, &arg2); // This is incorrect because both args don't have the same lifetime
}

pub fn execute_life_time<'a>(arg1: &'a str, arg2: &'a str) -> &'a str {
    arg1
}