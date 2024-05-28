use std::io;

fn find_nemo(str: &str) {
    let n = "Nemo";

    match str.find(n) {
        Some(num) => {
            println!("Result -> I found Nemo at {}!", num);
        }
        None => {
            println!("Result -> I can't find Nemo :(");
        }
    };
}

fn main() {
    println!("### Lets try to find nemo! ###");
    let mut phrase = String::new();
    let mut num: u8 = 1;

    loop {
        println!("");
        println!(
            "[loop: {}] [Please input a phrase on the screen or either write quit/exit to break the loop]", num);
        phrase.clear(); // Clear the string buffer for new input

        io::stdin()
            .read_line(&mut phrase)
            .expect("Failed to read line");

        // Trim the input to remove any trailing newline characters
        let trimmed_phrase = phrase.trim();

        if trimmed_phrase == "quit" || trimmed_phrase == "exit" || num == 255 {
            println!("Result -> OK, exiting!");
            break; // Exit this loop
        }

        find_nemo(trimmed_phrase);

        num = num + 1;
    }
}
