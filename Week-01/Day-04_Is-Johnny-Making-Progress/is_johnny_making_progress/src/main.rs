fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        let mut prev = 1e9 as i32;
        let mut progress_days = 0;
        for _ in 0..n {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let current: i32 = input.trim().parse().unwrap();
            if current > prev {
                progress_days += 1;
            }
            prev = current
        }
        println!("Progress days: {progress_days}");
    }
}
