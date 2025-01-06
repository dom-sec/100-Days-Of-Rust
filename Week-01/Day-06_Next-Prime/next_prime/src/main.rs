fn is_prime(n: i32) -> bool {
    for i in 2..(n as f64).sqrt() as i32 + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    
        let mut n: i32 = input.trim().parse().unwrap();
        loop {
            if is_prime(n) {
                println!("{}", n);
                break;
            }
            n += 1;
        }
    }
}
