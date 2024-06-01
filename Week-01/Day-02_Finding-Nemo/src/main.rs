use std::io;

fn find_nemo(str: &str) -> String {

    let arr: Vec<&str> = str.split(' ').collect();

    let word = "Nemo";

    match arr.iter().position(|&el| el == word) {
        Some(index) => {
            return format!("I found Nemo at {}!", index + 1);
        }
        None => {
            return "I can't find Nemo :(".to_string();
        }
    }

}

fn main() {
    println!("### Lets try to find nemo! ###");
    let mut phrase = String::new();
    let mut num: u8 = 1; // max 255

    loop {
        println!("");
        println!(
            "[loop: {}] Please input a phrase or either write quit/exit to exit",
            num
        );
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

        let result = find_nemo(&trimmed_phrase);

        println!("{}", &result);

        num = num + 1; // increase the loop count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text1() {
        assert_eq!(find_nemo("I am finding Nemo !"), "I found Nemo at 4!"); 
        // fail
    }

    #[test]
    fn text2() {
        assert_eq!(find_nemo("Nemo is me"), "I found Nemo at 1!")
        // fail
    }
    #[test]
    fn text3() {
        assert_eq!(find_nemo("I Nemo am"), "I found Nemo at 2!")
    }

    #[test]
    fn text4() {
        assert_eq!(find_nemo("I'm not NeMo"), "I can't find Nemo :(")
    }

    #[test]
    fn text5() {
        assert_eq!(find_nemo("Nemo N°1 and Nemo N°2 are different"), "I found Nemo at 1!")
        // fail
    }

    #[test]
    fn text6() {
        assert_eq!(find_nemo("Nemos are not Nemor nor Nemox or Nemoi"), "I can't find Nemo :(")
    }

}