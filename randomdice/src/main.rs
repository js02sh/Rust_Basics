use rand::distributions::{Distribution, Uniform}; // throw dice
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    let mut num = 0;
    let mut input = String::new();
    println!("Which number makes the game win?(1~6)_ ");

    let mut number: i32;
    loop {
        input.clear();
        
        // Read the user Input
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        //Parse the input into an Integer
        number = input.trim().parse().expect("Invalid input");

        //Check if the number is within the desired range
        if(1..=6).contains(&number){
            println!("Valid input: {}", number);
            break;
        } else {
            println!("Invalid input. Please enter a number between 1 and 6. ");
        }
    }


    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        num += 1; // count the play number
        if throw == number {
            println!("You've won at the {}rd turn", num);
            break;
        }
    }
}