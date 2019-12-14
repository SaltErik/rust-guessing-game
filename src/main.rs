use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  let mut tries_remaining: u8 = 10;

  let secret_number: u8 = rand::thread_rng().gen_range(1, 101); // lower inclusive, upper exclusive

  println!("I'm thinking of a number between 1 and 100...");
  println!("Can you guess what it is?");

  loop {
    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("ERROR: Failed to read line.");

    let guess: u8 = match input.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
      Ordering::Less => println!("Too small!"),
    }

    tries_remaining -= 1;

    match tries_remaining.cmp(&1) {
      Ordering::Greater => println!("{} guesses remaining!", tries_remaining),
      Ordering::Equal => println!("{} guess remaining!", tries_remaining),
      Ordering::Less => {
        println!("Out of guesses! Game over!");
        break;
      }
    }
  }
}
