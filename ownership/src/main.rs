fn main() {
    // Types in Stack Memory
    let x = 5;
    // It can be copied to the new variable
    let y = x; // Copy
    println!("y: {}", y);

    // Copy types:
    // Ints, Bool, Floats, Char and tuples with this types

    // Types in Heap Memory
    let s1 = String::from("text");
    // It will be moved to the new variable
    let mut s2 = s1; // Move
    // This meaning that s1 won't be available
    // println!("s1: {}", s1); // error
    // s2 will have the s1 value
    println!("s2: {}", s2);

    // Strings can be clonned to the new variable
    let s3 = s2.clone(); // Clone
    println!("s3: {}", s3);
    s2.push_str(" 1"); // This will only change the s2 value

    // s2 will be moved to the function's param
    get_string(s2);
    // This meaning that s2 won't more be available
    // println!("s2: {}", s2); error

    // s3 will be moved to the function's param and then it will be back to s4
    let s4 = return_string(s3);
    println!("s4: {}", s4);

    // s4 value will be moved to the function's param and then it will be back to s5 besides another values
    let (s5, size) = str_length(s4);
    println!("The size of {} is {}", s5, size);
} // Variables x, y and s5 will be removed from memory here

fn get_string(param: String) {
    println!("string: {}", param)
} // String Param will be removed from memory here

fn return_string(param: String) -> String { param }

// Functions can return more than one value
fn str_length(param: String) -> (String, usize) {
    let size = param.len();
    (param, size)
}
