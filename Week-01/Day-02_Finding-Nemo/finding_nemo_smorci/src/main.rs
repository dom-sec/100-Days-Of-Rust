use finding_nemo_smorci::{find_nemo_regex};
use std::io;

fn main() {

    println!("I will find Nemo in the given sentence.");
    println!("Make sure to always sepparate punctuation from the last word and awlays use punctuation at the end of the word. (ex. Here is Nemo !)");
    println!("Please enter a sentence");

    let mut buffer: String = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading input from stdin.");
    
    find_nemo_regex(&buffer);

}
