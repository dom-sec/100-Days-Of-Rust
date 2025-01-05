use std::io;

fn read_age() -> String {
    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Could not read age. You have to use an integer instead");

    return age;
}

fn main() {
    loop {
        println!("Enter your age: ");
        let age = read_age();
        let age: u32 = match age.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let conversion: u32 = age * 365;

        println!("{}", conversion);
    }
}
