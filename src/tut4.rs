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
    // lifetime can be used everywhere where the references are not present in the same scope
    // simple example
    let a: i32 = 5;
    let b = &a; // here you dont need to specify the lifetime because the rust compiler already know
                // what will be the scope of above variables
}

pub fn function_without_lifetime_reference(arg1: &i32, arg2: &bool) {
    // in this function rust compiler automatically assign lifetime to input variables aka input
    // lifetime which is similar to function_without_lifetime_reference(arg1:&'a i32, arg2:&'b bool) by following
    // the lifetime elision rules
}
pub fn function_with_lifetime_reference<'a>(arg1: &'a i32, arg2: &'a bool, arg3: &'a mut String) {
    //  you can give any name to your lifetime
}

pub fn function_with_one_param(arg1: &str) -> &str {
   // follows second rule so its lifetime can be inferred by the compiler
    arg1
}

// pub fn function_with_incorrect_return_ref (arg1:&str,arg2:&bool,arg3:&str) -> &str{
//    let a = "script_saga";
//    // a
//    &arg1
//    // the above code it wrong because the reference in destroyed after the function execution so rust compiler get confused
//    // and here you need to specify the lifetime manually
//    // always remember the returning life time with be the smallest of all the life if lifetime are not specified
//    // and hence the compiler get confused
// }

pub fn function_with_correct_return_ref<'a>(arg1: &'a str) -> &str {
    arg1
}

// sample example 

pub fn script_saga() {
   let arg1 = "script";
   {
      let arg2 = "saga";
      execute_life_time(&arg1, &arg2); // this is correct
   } 
   
   // execute_life_time(&arg1, &arg2); // this is incorrect cause both args done have same life time

}


pub fn execute_life_time<'a>(arg1:&'a str,arg2:&'a str)->&'a str{
   arg1
}

