// Understanding Structure in Rust

/*
    A struct is used to store multiple different types of data in one place.
    In Rust, a struct can be used with a custom type or as a class object if you want to
    implement object-oriented programming.
*/

// Normal struct implementation

pub struct ScriptSagaFollower {
    first_name: String,
    last_name: String,
    mobile_number: u64,
    country_code: u16,
}

// Pointer struct
// Pointer structs can be used as custom types that require the same arguments, but you need to differentiate them because they
// have different traits or implementations or when you want to be very specific.

pub struct CartesianCoordinate(i32, i32, i32); // Implement some different types of functions or traits

pub struct EulerCoordinate(i32, i32, i32); // Implement some different types of functions or traits

pub fn implementing_rust_struct() {
    // Tuple struct
    let robot_cartesian_coordinate = CartesianCoordinate(0, 0, 0);
    let robot_euler_coordinate = EulerCoordinate(10, 20, 30);

    let user: ScriptSagaFollower = ScriptSagaFollower {
        first_name: String::from(""),
        last_name: String::from(""),
        mobile_number: 123456789,
        country_code: 91,
    };

    println!("User first name: {}", user.first_name);
    println!("User last name: {}", user.last_name);
    println!("User mobile number: {}", user.mobile_number);
    println!("User country code: {}", user.country_code);
}
