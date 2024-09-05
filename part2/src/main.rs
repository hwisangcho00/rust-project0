// you'll need these two imports to read input form the user
use std::io;
use std::io::Write;

fn main() {
  let word = "catfish";
  let mut word_so_far = String::from("_______");
  let mut guesses_left = 6;

  println!("Welcome to cis1905 Hangman!");

  loop {
    println!("The word so far is {}", word_so_far);
    println!("You have {} guesses left", guesses_left);
    print!("Please guess a letter: ");

    io::stdout().flush().expect("Error flushing stdout.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Error reading line.");

    let guess = guess.trim().to_lowercase();

    if guess.len() != 1 {
      println!("Please enter a single letter.");
      continue;
    }

    if !guess.chars().next().unwrap().is_ascii_alphabetic() {
      println!("Please enter a letter.");
      continue;
    }

    let guess_char = guess.chars().next().unwrap();

    if word.contains(guess_char) {
      for (i, c) in word.chars().enumerate() {
        if c == guess_char {
          word_so_far.replace_range(i..i+1, &guess_char.to_string());
        }
      }
    } else {
      println!("Sorry, that letter is not in the word.");
      guesses_left -= 1;
    }

    if guesses_left == 0 {
      println!("Sorry, you ran out of guesses!");
      break;
    }

    if word_so_far.contains("_") {
      continue;
    }
    
    println!("Congratulations! You guessed the secret word: {}", word);
    break;

  }

}

