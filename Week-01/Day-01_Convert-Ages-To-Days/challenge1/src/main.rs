fn calc_age(age: &u32) -> u32 {
    return age * 365;
}

fn main() {

    println!("Enter age!");

    let mut age = String::new();
    std::io::stdin().read_line(&mut age).expect("Issue");
    let age: u32 = age.trim().parse().expect("Issue");

    println!("Age in days: {}", calc_age(&age));
}
