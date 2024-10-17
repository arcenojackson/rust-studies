use std::fmt::Display;

struct Par<T> {
    x: T,
    y: T,
}

impl<T> Par<T> {
    fn novo(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Par<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("O maior membro é x = {}", self.x);
        } else {
            println!("O maior membro é y = {}", self.y);
        }
    }
}
