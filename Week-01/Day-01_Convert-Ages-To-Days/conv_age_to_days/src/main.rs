use std::io;

fn calc_age(num: u32) -> u32 {
    const DAYS_IN_A_YEAR: u32 = 365; 
    
    num * DAYS_IN_A_YEAR
}

fn main() {

    // Print some messages on the screen
    println!("Let me convert your age to days!");
    println!("Please input your age in years.");

    // Create a variable to store the user input
    // mut is mutable (variables are immutable unless explicitly stated)
    let mut age = String::new();

    // Receiving User Input
    io::stdin()
        .read_line(&mut age) 
        // read_line takes the typed text into std input and append that into 
        // a string (without overwriting its contents), so we therefore pass 
        // that string as an argument. The string argument needs to be mutable 
        // so the method can change the string’s content.& indicates that 
        // this argument is a reference.
        // References are also immutable by default, hence, we write 
        // `&mut guess` to make it mutable
        .expect("Failed to read line");
        // read_line returns a Result value
        // Result’s variants are Ok and Err. The Ok variant indicates the 
        // operation was successful, and inside Ok is the successful value. 
        // The Err variant means the operation failed, and Err contains info 
        // about how or why the operation failed.
        // Values of the Result type, have methods defined on them. 
        // An instance of Result has an expect method that you can call.
        // If this instance of Result is an Err value, expect will cause 
        // the program to crash and display the message that you passed 
        // as an argument to expect. If the read_line method returns an Err, 
        // it would likely be the result of an error in the OS.

    // We show the input to the user. Using {age} as a placeholder
    println!("Your input is: {age}");

    // Parse the string to u32
    // https://doc.rust-lang.org/std/primitive.str.html#method.parse
    let parsed_num: Result<u32, _> = age.trim().parse();

    // parsed_num is type Result, we need to accomodate for the two scenarios:
    match parsed_num {
        Ok(parsed) => {
            // Perform the calculation
            println!("Your result is: {} days", calc_age(parsed));
        }
        Err(_) => {
            println!("Failed to parse the string to u32");
        }
    }

}
