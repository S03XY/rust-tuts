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