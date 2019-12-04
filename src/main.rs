use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  let mut attempts_left = 10;

  let secret_number = rand::thread_rng()
    .gen_range(1, 101); // lower inclusive, upper exclusive
  
  println!("I'm thinking of a number between 1 and 100...");
  println!("Can you guess what it is?");

  loop {
    let mut input = String::new();
    
    io::stdin()
      .read_line(&mut input)
      .expect("ERROR: Failed to read line.");
    
    let guess: u32 = match input
      .trim()
      .parse::<u32>() {
        Ok(num) => num,
        Err(_) => continue,
      };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }

    attempts_left -= 1;

    if attempts_left > 1 {
      println!("{} guesses remaining!", attempts_left);
    } else if attempts_left == 1 {
      println!("{} guess remaining!", attempts_left);
    } else {
      println!("Out of guesses! Game over!");
      break;
    }
  }
}
