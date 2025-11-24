use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let mut command_run = true;

    if args.len() > 1 {
        if args[1] == "guess" {
            guess_secret_number();
        } else if args[1] == "convert" {
            let fahrenheit = args[2].parse::<f64>().expect("Invalid input");
            fahrenheit_to_celsius(fahrenheit);
        } else if args[1] == "fib" {
            if args.len() < 3 {
                println!("Invalid input");
                command_run = false;
            } else {
                let n = args[2].parse::<u32>().expect("Invalid input");
                println!("Fibonacci({}) = {}", n, fibonacci(n));
            }
        } else if args[1] == "args" {
            for (i, arg) in args.iter().enumerate() {
                println!("Argument {}: {}", i, arg);
            }
        } else if args[1] == "owner" {
            ownership();
        } else if args[1] == "rectangle" {
            if args.len() < 4 {
                println!("Invalid input");
                command_run = false;
            } else {
                let width = args[2].parse::<u32>().expect("Invalid input");
                let height = args[3].parse::<u32>().expect("Invalid input");
                let rect = Rectangle { width, height };
                println!("Calculating area of rectangle, {:?}", rect);
                println!("Area of rectangle: {}", rect.area());
                let rect2 = Rectangle { width: 5, height: 5 };
                println!("Can hold another rectangle {:?}? {}", rect2, rect.can_hold(&rect2));
                let rect3 = Rectangle { width: 10, height: 10 };
                println!("Can hold another rectangle {:?}? {}", rect3, rect.can_hold(&rect3));
                println!("Square version of your rectangle: {:?}", Rectangle::square(rect.width));
            }
        } else if args[1] == "person" {
            if args.len() < 6 {
                println!("Invalid input");
                command_run = false;
            } else {
                let name = args[2].to_string();
                let age = args[3].parse::<u32>().expect("Invalid input");
                let height = args[4].parse::<u32>().expect("Invalid input");
                let weight = args[5].parse::<u32>().expect("Invalid input");
                let person = Person { age, name, height, weight };
                println!("Person: {:?}", person);
                let person2 = Person { age: 15, name: "Zack".to_string(), height: 170, weight: 150 };
                println!("Person2: {:?}", person2);
                println!("Person is older than Person2: {}", person.older(&person2));
            }
        } else if args[1] == "maps" {
            fun_with_maps();
        }
    }
    
    if !command_run {
        help();            
    }
}

fn ownership() {
    let mut s = String::from("hello");
    s.push_str(", world!"); 
    println!("{s}"); 
}

#[derive(Debug)]
struct Person {
    name: String, 
    age: u32,
    height: u32,
    weight: u32,
}

impl Person {
    fn older(&self, other: &Person) -> bool {
        self.age - other.age > 0
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        dbg!(self);
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
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
    println!("  owner - Run the ownership sample function");
    println!("  rectangle [width] [height] - Calculate area of a rectangle");
    println!("  maps - Work with HashMaps");
    println!("");
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

use std::collections::HashMap;

fn fun_with_maps() {
    let mut scores = HashMap::new();
        
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
        
    let team_name = String::from("Blue");
    let score1 = scores.get(&team_name).copied().unwrap_or(0);
    
    println!("The score for team {} is {}", team_name, score1);
    
    let score2 = scores.get(&String::from("Yellow")).unwrap_or(&0);
    
    println!("The score for team Yellow is {}", score2);
    
    println!("score1: {} plus score2: {} equals {}", score1, score2, score1 + score2);
}
