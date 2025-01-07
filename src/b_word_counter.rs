// Word Counter
// Write a program that:
// 1. Reads a text file provided by the user.
// 2. Counts the occurrences of each word and displays the top 10 most frequent words.

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    // Get the file path from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} ./word.txt", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Check if the file exists and is readable
    if !Path::new(file_path).exists() {
        eprintln!("File not found: {}", file_path);
        std::process::exit(1);
    }

    // Read the file and count word occurrences
    match count_words(file_path) {
        Ok(word_counts) => {
            // Display the top 10 most frequent words
            println!("Top 10 most frequent words:");
            for (word, count) in word_counts.iter().take(10) {
                println!("{}: {}", word, count);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

}

fn count_words(file_path: &str) -> io::Result<Vec<(String, usize)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut word_counts: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            // Normalize the word to lowercase and strip punctuation
            let word = word
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>();

            if !word.is_empty() {
                *word_counts.entry(word).or_insert(0) += 1;
            }
        }
    }

    // Sort the words by frequency in descending order
    let mut sorted_word_counts: Vec<(String, usize)> = word_counts.into_iter().collect();
    sorted_word_counts.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(sorted_word_counts)
}