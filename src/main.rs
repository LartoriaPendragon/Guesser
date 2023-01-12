use io::stdin; //Imports io from the standard library, this allows for input from user
use std::io;
use rand::Rng;
use std::cmp::{Ordering};

fn main() {

    println!("Guess the number!"); //Declarative macro that prints a string to the screen, this is output the user wil see

    let secret_number = rand::thread_rng().gen_range(1..=100); //Generates a random variable between 1 and 100 and a
    //amends it to the variable called secret_number

    loop { //All code contained within this loop brace repeats indefinitely from the top down

    println!("Please input your number"); //Prints a prompt requesting for input from user

    let mut guess = String::new(); //The let statement creates a new mutable variable called guess,
    //the equal sign is what we want to bind the variable to. In this case its a string. The new function called new
    // creates an empty string

    stdin()
        .read_line(&mut guess) //the read_line method collects input from the user in this case, &mut guess tells it what
        //string to store the data in. The job of read_line is to take whatever the user types into standard input and
        //append it into the empty string we created earlier. The string argument needs to be mutable so the method can
        //change the strings content
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {guess}"); //prints out the user input

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
                break //Exits the loop once the user successfully guesses the secret number

              }
        }

    }

}
