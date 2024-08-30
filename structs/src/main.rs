// Declaring a Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Declaring a tuple struct
struct Color(i32, i32, i32);

fn main() {
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

    // Using a tuple struct
    let black = Color(0, 0, 0);
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
