// Using Generics in the function params
fn bigger<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut bigger = list[0];
    for &item in list.iter() {
        if item > bigger {
            bigger = item;
        }
    }
    bigger
}

use generics_traits::{notify, some_function, News, Resume, Tweet};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = bigger(&number_list);
    println!("The biggest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = bigger(&char_list);
    println!("The biggest chat is {}", result);

    // Using the traits implemented in lib.ts
    let tweet = Tweet {
        username: String::from("arcenojackson"),
        content: String::from("Tweet contents"),
        answer: false,
        retweet: false,
    };
    println!("New tweet from {}: {}", tweet.author_summary(), tweet.summary());

    let news = News {
        title: String::from("News Title"),
        local: String::from("news local"),
        author: String::from("arcenojackson"),
        content: String::from("news content"),
    };
    println!("News available! {}", news.summary());
    notify(news);
    notify(tweet);
    // This will cause error because i32 do not implement the Trait Resume
    // notify(10);

    _ = some_function(10, 5);
}

// Good news: the way Rust implements generic types means that
// your code won’t run any slower than if you had specified concrete types
// instead of generic types as parameters!

// Rust accomplishes this by performing monomorphization of code using
// generic types at compile time.
// Monomorphization is the process of transforming generic code into
// specific code by replacing generic types with the concrete types that
// are actually used.

// What the compiler does is the opposite of the steps we took to
// create a generic type function.
// The compiler looks at all the places where the generic code is called
// and generates code for the concrete types that the generic code is called.
