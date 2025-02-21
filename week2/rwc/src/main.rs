use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut contain = Vec::new();

    for ele in reader.lines() {
        let line = ele.unwrap();
        if line != "" {
            contain.push(line);
        }
    }

    let line_number = contain.len();
    let char_number = contain
        .iter()
        .map(|s| s.as_bytes().len())
        .sum::<usize>() + line_number-1;
    let word_number = contain
        .iter()
        .map(|s| s.trim().split_whitespace().count())
        .sum::<usize>();

    println!(
        "{:3}{:4} {} {}",
        line_number, word_number, char_number, filename
    );
    // Your code here :)
}

//different from linux wc  beacause the definetion of line are different