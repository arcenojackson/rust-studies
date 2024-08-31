use std::io;

// Declaring a Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Declaring a tuple struct
struct Color(i32, i32, i32);

fn structs() {
    let mut user1 = User {
        email: "example@mail.com".to_string(),
        username: String::from("example"),
        active: true,
        sign_in_count: 1,
    };
    println!("Hello {}", user1.username);
    // Update username of user1
    user1.username = "test".to_string();
    println!("Hello {}, with email {}", user1.username, user1.email);

    // Using struct update syntax
    let user2 = User {
        username: String::from("Example 2"),
        ..user1
    };
    println!("Hello {}, with email {}", user2.username, user2.email);

    // Using a User constructor function
    let user3 = build_user(String::from("test@mail.com"), String::from("Test"));
    println!("Hello {}, with email {}", user3.username, user3.email);
    println!("{} | {}", user3.sign_in_count, user3.active);

    // Using a tuple struct
    let black = Color(0, 0, 0);
    println!("{} | {} | {}", black.0, black.1, black.2);
}

fn build_user(email: String, username: String) -> User {
    // Using Field Init Shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

// Struct example (Rectangle)

#[derive(Debug)] // Adding traits de Debug to struct
struct Rectangle {
    length: u32,
    width: u32,
}

// Implementing methods and functions to struct Rectangle
impl Rectangle {
    // Method to return the areas from rectangle
    fn area(&self) -> u32 {
        self.length * self.width
    }

    // Method that receive another References besides the self
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // Function that returns a Rectangle instance in squares shape
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

// Normal function that receive a reference to Rectangle
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

fn rectangle() {
    let rect1 = Rectangle { length: 50, width: 30 };
    // With {:#?} we can print all the struct with values
    println!("The rect1 is {:#?}", rect1);

    // Using the function area()
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // Using the method area()
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // We can use struct functions with namespace syntax
    let sq = Rectangle::square(3);
    println!("The sq is {:#?}", sq);
    // We also can use struct methods with namespace syntax,
    // but informing a instance to &self
    println!("Can rect1 hold sq? {}", Rectangle::can_hold(&rect1, &sq));
}

fn main() {
    println!("Run program example? (Y/N)");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Fail to read the choice!");
    if choice.trim() == "N" { return structs() }
    rectangle()
}
