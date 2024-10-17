use std::fmt::{Debug, Display};

// Trait is kind a Interface
pub trait Resume {
    // Signature of the contract method
    fn summary(&self) -> String {
        // Default implementation
        String::from("(Default summary...)")
    }

    fn author_summary(&self) -> String;
}

pub struct News {
    pub title: String,
    pub local: String,
    pub author: String,
    pub content: String,
}

// Using the default implementation of the Trait Resume
impl Resume for News {
    fn author_summary(&self) -> String {
        todo!()
    }
    // fn summary(&self) -> String {
    //     format!("{}, by {} ({})", self.title, self.author, self.local)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub answer: bool,
    pub retweet: bool,
}

// Implementing new behavior to the Trait Resume
impl Resume for Tweet {
    fn summary(&self) -> String {
        format!("{}", self.content)
    }

    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

// To use the method notify, the params must to implement the Trait Resume
pub fn notify<T: Resume>(item: T) {
    // Trait limit
    println!("The last news! {}", item.summary());
}

// To use many trait limits, do this:
pub fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
  t.to_string();
  println!("{:?}", u);
  10
}
