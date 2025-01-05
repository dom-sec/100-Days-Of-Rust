use std::io;

fn main() {
    loop {
        println!("Enter a sentence to find Nemo: ");
        let mut nemo_finding = String::new();

        io::stdin()
            .read_line(&mut nemo_finding)
            .expect("Could not read the sentence. You have to use a string instead");

        let words: Vec<&str> = nemo_finding.split_whitespace().collect();
        let mut found = false;
        for (i, &word) in words.iter().enumerate() {
            if word == "Nemo" {
                println!("I found Nemo at {}", i + 1);
                found = true;
                break;
            }
        }
        if !found {
            println!("I can't find Nemo");
        }
    }
}
