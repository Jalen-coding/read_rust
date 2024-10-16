use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead, Read};

fn main() {
    println!("Enter a file path with double \\\\ at each new folder");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read line");
    let file_path = file_path.trim();

    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    println!("{}",count_words(file_path));
    println!("{}",count_lines(file_path));
    println!("{}", count_letters(file_path));
}



fn count_words(file_path: &str) -> u32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut word_count = 0;
    for line in reader.lines() {
        for _word in line.unwrap().split_whitespace() {
            word_count += 1;
        }
    }
    word_count
}

fn count_lines(file_path: &str) -> u32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut line_count = 0;
    for _ in reader.lines() {
        line_count += 1;
    }
    line_count
}

fn count_letters(file_path: &str) -> usize {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let letters = contents.chars().filter(|c| c.is_alphabetic()).count();

    letters
}