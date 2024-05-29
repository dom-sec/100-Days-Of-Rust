use std::io;
use regex::Regex; // https://crates.io/crates/regex

fn find_nemo(str: &str) {
    let re = Regex::new(r"\bNemo\b").unwrap(); // \b is a word boundary

    match re.find(str) {
        Some(m) => {
            println!("I found Nemo at {}!", m.start());
        }
        None => {
            println!("I can't find Nemo :(");
        }
    };
}

fn main() {
    println!("### Lets try to find nemo! ###");
    let mut phrase = String::new();
    let mut num: u8 = 1; // max 255

    loop {
        println!("");
        println!(
            "[loop: {}] Please input a phrase or either write quit/exit to exit", num);
        phrase.clear(); // Clear the string buffer for new input

        io::stdin()
            .read_line(&mut phrase)
            .expect("Failed to read line");

        // Trim the input to remove any trailing newline characters
        let trimmed_phrase = phrase.trim();
        
        // Exit by text or last loop
        if trimmed_phrase == "quit" || trimmed_phrase == "exit" || num == 255 {
            println!("-> OK, exiting!");
            break; // Exit this loop
        }

        find_nemo(trimmed_phrase);

        num = num + 1; // increase the loop count
    }
}
