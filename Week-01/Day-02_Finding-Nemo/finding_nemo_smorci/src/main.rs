use finding_nemo_smorci::{get_input, find_nemo_iterator};

fn main() {

    println!("I will find Nemo in the given sentence.");
    println!("Make sure to always sepparate punctuation from the last word. (ex. Here is Nemo !)");
    println!("Please enter a sentence");

    let buffer: String = get_input().expect("Unable to read input from stdin.");

    find_nemo_iterator(&buffer);

}
