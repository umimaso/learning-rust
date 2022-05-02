use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate secret number
    // gen_range(start..end), inclusive lower bound, exclusive upper bound
    // 1..=100 is also equivalent
    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Create a mutable variable bound to a new empty instance of String
        let mut guess = String::new();

        // Call the read_line method on the stdin handle, this will append the input to the passed arg
        io::stdin()
            .read_line(&mut guess) // Argument needs to be mutable for the append
            .expect("Failed to read line");

        // Shadow the previous value of guess with a new one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // If the guess can't be parsed to number then start a new loop iteration
        };

        println!("You guessed: {}", guess);

        // Compare guess with answer
        // Rust will infer that secret_number should be a u32 type when compared with guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop
            }
        }
    }
}
