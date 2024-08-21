fn main() {
    branches();
    loops();
    const TARGET: char = 'F';
    let temp = temperatures(36.2, TARGET);
    println!("A temperatura em °{} é: {}", TARGET, temp);
}

fn branches() {
    let number = 0;

    if number > 5 {
        println!("Number is greater than 5")
    } else if number != 0 {
        println!("Number is different of 0")
    } else {
        println!("Number is less than 5")
    }
    
    let condition = number != 0;
    let number = if condition { 1 } else { -1 };
    println!("Number is: {}", number);
}

fn loops() {
    let mut x = 0;

    loop {
        println!("[Loop]: Valor de x é: {}", x);
        if x > 2 { break }
        x += 1;
    }

    while x != 0 {
        println!("[While]: Valor de x é: {}", x);
        x -= 1;
    }

    let a = [10, 20, 30, 40, 50];

    // Iterate a collection
    for item in a.iter() {
        println!("O valor de item é: {}", item);
    }

    // Iterate a range (.rev() reverts the range)
    for number in (x..3).rev() {
        println!("O valor de number é: {}", number);
    }
}

fn temperatures(temp: f64, target: char) -> f64 {
    if target == 'C' {
        return  (temp - 32.0) / 1.8
    }
    temp * 1.8 + 32.0
}
