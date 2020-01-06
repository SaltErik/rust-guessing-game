use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Eq)]
pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }
    Guess { value }
  }
}

impl Ord for Guess {
  fn cmp(&self, other: &Self) -> Ordering {
      self.value.cmp(&other.value)
  }
}

impl PartialOrd for Guess {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl PartialEq for Guess {
  fn eq(&self, other: &Self) -> bool {
      self.value == other.value
  }
}

fn main() {
  let mut tries_remaining = 10;

  let secret_number = Guess::new(rand::thread_rng().gen_range(1, 101)); // lower inclusive, upper exclusive

  println!("I'm thinking of a number between 1 and 100...");
  println!("Can you guess what it is?");

  loop {
    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("ERROR: Failed to read line.");

    let guess = match input.trim().parse() {
      Ok(num) => Guess::new(num),
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
