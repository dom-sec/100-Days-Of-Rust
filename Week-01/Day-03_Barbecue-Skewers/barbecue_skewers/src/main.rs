fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        let mut meat = 0;
        let mut non_meat = 0;
        for _ in 0..n {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut is_meat = false;
            for ingredient in input.chars() {
                if ingredient == 'x' {
                    is_meat = true;
                    break;
                }
            }
            if is_meat {
                meat += 1;
            } else {
                non_meat += 1;
            }
        }
        println!("No. of vegetarian skewers: {}", non_meat);
        println!("No. of non-vegetarian skewers: {}", meat);
    }
}
