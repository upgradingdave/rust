use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        if args[1] == "guess" {
            guess_secret_number();
        } else if args[1] == "convert" {
            let fahrenheit = args[2].parse::<f64>().expect("Invalid input");
            fahrenheit_to_celsius(fahrenheit);
        } else if args[1] == "fib" {
            let n = args[2].parse::<u32>().expect("Invalid input");
            println!("Fibonacci({}) = {}", n, fibonacci(n));
        } else if args[1] == "args" {
            for (i, arg) in args.iter().enumerate() {
                println!("Argument {}: {}", i, arg);
            }
        } else {
            help();
        }
    } else {
        help();
    }
}

fn help() {
    println!("Usage: ");
    println!("  hello_rust [command]");
    println!("  -or-");
    println!("  cargo run -- [command]");
    println!("Commands:");
    println!("  guess - Guess a secret number");
    println!("  convert - Convert Fahrenheit to Celsius");
    println!("  fib - Calculate Fibonacci number");
    println!("  args - Print command-line arguments");
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn guess_secret_number() {
    println!("Guess the number");

    let secret_number = rand::rng().random_range(1..=100);

    //println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
