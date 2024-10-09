use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    // UNRECOVERABLE ERRORS
    // Ends the program with an error
    // panic!("Hello, world!");

    // Doesn't compile
    // let vet = [1, 2, 3];
    // vet[99];

    // RECOVERABLE ERRORS
    let f = File::open("hello.txt");
    let _ = match f {
        Ok(file) => file,
        // REF is used to get the reference by a value, different of & that get the value by a reference
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // the IF in a Match condition is a Match Guard, to refine the validations
            println!("File not found, creating it...");
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Failed to create the file: {:?}", e)
                }
            }
        }
        Err(error) => {
            panic!("Failed to open the file: {:?}", error)
        },
    };

    // there is a non verbose way that is possible in this case
    // With unwrap()
    // let _ = File::open("world.txt").unwrap();
    // Or with expect()
    // let _ = File::open("world.txt").expect("Failed to open world.txt");

    // Propagate errors
    let result = read_username_from_file().expect("Failed to read the name");
    println!("Name is: {:?}", result)
}

// Propagate errors
fn read_username_from_file() -> Result<String, io::Error> {
    // Using Match to propagate the errors
    // let f = File::open("names.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e) // Return the error to the code that called this function
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s), // Return the correct value to the code that called this function
    //     Err(e) => Err(e) // Return the error to the code that called this function
    // }

    // Using ? instead Match to reduce the code lines
    // let mut f = File::open("names.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s);

    // Reducing more code lines
    let mut s = String::new();
    File::open("names.txt")?.read_to_string(&mut s)?;
    Ok(s)
    // the ? only can be used in functions that returns Result<T, E>
}
