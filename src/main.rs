mod tut2;
mod tut3;

// how do we define a function in rust ?
// fn <function_name>(arg1:type,arg2:type,arg3:type...) -> type {
//  return <object_of_return_type>
// }
// use pub keyword when this function is used outside a module of from the current current file

pub fn script_saga_function_one(_arg1: u32, _arg2: &String, _arg3: bool) -> (u32, bool) {
    // _args => remove compiler warning but binds the variables
    // _ => use this if you want to completely ignore the varibles and its useful for defining function signatures or in pattern matching

    // code here...

    // return  (23,true); OR if you dont want to use "return" term then use
    (23, true)
}

fn main() {
    // tut2::function_return_a_result();
    // tut3::variable_in_rust()
    tut3::understand_borrow_rules();

    // let arg2 = String::from("Shashank");
    // script_saga_function_one(1, &arg2, true);
}



