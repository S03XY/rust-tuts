// Understanding Rust Hash Maps
use std::collections::HashMap;

// A hash map is used as key-value storage.
pub fn understanding_hash_map() {
    // You need to specify the type of key-value pairs that
    // will be stored in the hash map.
    let mut map: HashMap<i32, String> = HashMap::new();

    // Inserting values into the map.
    map.insert(0, "script_saga_follower_one".to_string());
    map.insert(1, "script_saga_follower_two".to_string());
    map.insert(2, "script_saga_follower_three".to_string());

    // Getting values from the map.
    let value_one = map.get(&0); // Returns an Option because a value may or may not exist.

    // Getting the length of the map.
    let length = map.len();

    // You can also create a map using capacity and get the capacity.
    let map_with_capacity: HashMap<i32, String> = HashMap::with_capacity(10);
    let capacity = map_with_capacity.capacity(); // With capacity, you tell the compiler to reserve memory for that amount of elements
                                                 // without reallocating the memory.

    // Iterating over a map.
    for (key, value) in map.iter() {
        // Execute code for key and value.
    }

    // Iterating over keys of a map.
    for key in map.keys() {
        // Execute code with keys.
    }

    // Iterating over values of a map.
    for v in map.values() {
        // Execute code for values.
    }

    // Other useful functions.

    // Convert into a Vec.
    let a: Vec<&i32> = map.keys().collect(); // Collect is an iterator function that collects all the elements and converts
                                             // them into an array of the specified type.

    // All the iterator functions are valid. For example, if you want to filter out.
    let filter: Vec<(&i32, &String)> = map
        .iter()
        .filter(|&(_, value)| value == &"script_saga_follower_one".to_string())
        .collect(); // Will discuss more in detail about the iterator trait later on.

    for (key, value) in filter.iter() {
        println!("{}", value);
    }

    // Removing a value.
    map.remove(&0);

    // Check whether the key exists in the map.
    let contains = map.contains_key(&0);

    // Check if the map is empty.
    let is_empty = map.is_empty();

    // Cleans the map but keeps the allocated memory for reuse.
    map.clear();
}
