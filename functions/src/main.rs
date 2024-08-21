fn main() {
    println!("Hello, world!");
    let result = another_function(12, 10);
    println!("O resultado é: {}", result);
    let y = {
        let x = 3;
        x + 1 // Return of code block
    };
    println!("O valor de y é: {}", y)
}

fn another_function(x: u32, y: u32) -> u32 {
    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);
    if x > y {
        return x;
    }
    y // Implicit return
}
