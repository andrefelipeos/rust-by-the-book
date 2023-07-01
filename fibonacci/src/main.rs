use std::io;

fn main() {
    println!("In the Fibonacci sequence:");
    println!(" - the first element is 0 and has index 0");
    println!(" - the second element is 1 and has index 1");
    println!(" - the following elements are computed as the sum of their two closest predecessors");

    let number: u32 = loop {
        println!("Please input a number.");

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line.");

        match number.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };

    println!("The Fibonacci number at index {} is {}.", number, fibonacci_number(number));
    println!("The first {} Fibonacci numbers are:", number);
    fibonacci_sequence(number);
}

fn fibonacci_number(index: u32) -> u32 {
    if index <= 1 {
        return index;
    }

    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut fib: u32 = a + b;

    for _ in 3..=index {
        a = b;
        b = fib;
        fib = a + b;
    }

    fib
}

fn fibonacci_sequence(length: u32) {
    let mut a: u32 = 0;
    let mut b: u32 = 1;

    if length > 0 {
        print!("{}", a);
    }

    if length > 1 {
        print!(" {}", b);
    }

    for _ in 3..=length {
        let fib: u32 = a + b;
        print!(" {}", fib);
        a = b;
        b = fib;
    }

    println!();
}

