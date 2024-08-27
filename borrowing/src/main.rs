// Improving the below function from Ownership examples
// fn str_length(param: String) -> (String, usize) {
//     let size = param.len();
//     (param, size)
// }

fn main() {
    let s1 = String::from("Text");
    // Using a reference to s1 such a param (Borrowing)
    let size = str_length(&s1);
    println!("The size of '{}' is {}", s1, size);

    // To change the value of a reference in a function,
    // it need to be a mutable variable and mutable refernce
    let mut s2 = String::from("Text"); // Mutable variable
    println!("s2 is '{}'", s2);
    change_str(&mut s2); // Mutable reference
    println!("now, s2 is '{}'", s2);

    // We cannot borrow a mutable variable more than once at a time in a scope
    let mut s3 = String::from("Text");
    let ref1 = &mut s3;
    // let ref2 = &mut s3; // Error
    println!("ref1: {}", ref1);
    // println!("ref2: {}", ref2);
    {
        // Since this is a new scope, the borrow is permitted
        let ref3 = &mut s3;
        println!("ref3: {}", ref3);
    }
    // If we need to use the ref1 after the scope above, the ref3 won't works
    // println!("ref1: {}", ref1);
    // println!("ref3: {}", ref3); Error: Here ref3 already not exists

    // SLICES
    let mut s4 = String::from("long text");
    // Getting index of the first word
    let word = fwi(&s4);
    // s4 will be a empty string
    s4.clear();
    // s4 is now a empty string, but word continue with value 4
    println!("s4 is: '{}', first word end at index: {}", s4, word);

    // take back the text to s4
    s4.push_str("long text");
    let first_word_index = fwi(&s4);
    // Using range to get the partial string in s4, from 0 to index of the end of first word
    let first_word = &s4[..first_word_index]; // 0 is optional [0..2]
    println!("s4 is: '{}', the first word is: '{}'", s4, first_word);
    let second_word = &s4[first_word_index+1..]; // the last element is optional [2..-1]
    // s4.clear(); Error: We cannot change the value of s4 after using slices
    println!("s4 is: '{}', the second word is: '{}'", s4, second_word);
    
    s4.push_str("long text");
    let new_first_word = fw(&s4);
    // s4.clear(); Error: We cannot change the value of s4 after using slices
    println!("s4 is: '{}', the first word is: '{}'", s4, new_first_word);
}

fn fwi(param: &String) -> usize {
    // Creating an bytes array from the string in param
    let bytes = param.as_bytes();
    // iter() make the bytes array iterable and...
    // enumerate() get the index and value for each iterate
    for (i, &item) in bytes.iter().enumerate() {
        // if value is equal a empty space, return his index
        if item == b' ' { // b' ' == literal byte syntax
            return i;
        }
    }
    param.len()
}

fn fw(param: &str) -> &str {
    let bytes = param.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &param[0..i]; // Return the slice from initial to first empty space
        }
    }
    &param[..] // Return all slice
}

// Receiving in params a reference to an object instead get the object
fn str_length(param: &String) -> usize {
    // The param cannot be changed because it's only reference
    // param.push_str(" 2");
    param.len()
}

// Receiving in params a mutable reference to an object
fn change_str(param: &mut String) {
    param.push_str(" 2");
}
