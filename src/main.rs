extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => println!("You win!"),
        }
    }
}

// Notes:
// ---------------
// ! means macro
// let keyword for variables
//
// variables are immutable by default. 'mut' makes it mutable

// String is a string type provided by the standard library
// that is a growable, utf-8 encoded bit of text

// The :: syntax in the ::new line indicates that new is
// an associated function of the String type. An associated
// function is implemented on a type, in this case String
// Like a static method

// io::stdin() is a handle to the stdin of the terminal
// & is a referenc, which gives you a way to let multiple
// parts of your code access one piece of data without needing
// to copy that data into memory multiple times
// references are immutable by default

// The Result types are enumerations, often referred to as enums.
// An enumeration is a type that can have a fixed set of values,
// and those values are called the enum’s variants.
