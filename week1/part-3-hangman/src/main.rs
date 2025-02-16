// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<(char, bool)> = secret_word.chars().map(|s| (s, false)).collect();
    println!("Welcome to CS110L Hangman!");
    // Uncomment for debugging:
    println!("random word: {}", secret_word);
    let mut contain: Vec<char> = Vec::new();
    let mut count = 0;
    show(secret_word_chars, &mut contain, &mut count);
}

fn show(mut secret_word_chars: Vec<(char, bool)>, contain: &mut Vec<char>, count: &mut u32) {
    loop {
        print!("The word so far is ");
        secret_word_chars.iter().for_each(|(val, a)| {
            if *a == true {
                print!("{}", val)
            } else {
                print!("-")
            }
        });
        println!("");

        print!("You have guessed the following letters: ");
        contain.iter().for_each(|s| print!("{}", s));

        // Your code here! :)
        println!(
            "\nYou have {} guesses left",
            secret_word_chars
                .iter()
                .filter(|(_, a)| *a == false)
                .count()
        );

        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let mut flag = false;

        match guess.trim().chars().next() {
            Some(a) => {
                let guess = a;
                contain.push(guess);
                for (val, a) in secret_word_chars.iter_mut() {
                    if *val == guess {
                        *a = true;
                        flag = true;
                    }
                }
            }
            None => {
                flag = true;
            }
        }

        if flag == false {
            println!("Sorry, that letter is not in the word");
            *count += 1;
        }

        println!("");

        if secret_word_chars
            .iter()
            .filter(|(_, a)| *a == false)
            .count()
            == 0
        {
            println!("Congratulations you guessed the secret word: lobster!");
            break;
        }

        if *count >= NUM_INCORRECT_GUESSES {
            println!("Sorry, you ran out of guesses!");
            break;
        }
    }
}
