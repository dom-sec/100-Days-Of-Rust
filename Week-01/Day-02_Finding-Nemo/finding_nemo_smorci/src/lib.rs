//! This crate provides functions to find the whitespace separated word `Nemo` in a given sentence
//!
use regex::Regex;

/// Find Nemo Regex Solution
pub fn find_nemo_regex (buffer: &str) {

    let nemo_regex = Regex::new(r"\s*Nemo\s+").expect("Wrong regex pattern.");
    match nemo_regex.find(buffer.trim()) {
        Some(value) => {
            let slice: &str = &buffer[..value.start()+1];
            let split = slice.split(' ');
            let place_of_nemo: usize = split.count();
            println!("Found Nemo at {place_of_nemo}!");
        },
        None => println!("I can't find Nemo :("),
    }
}
