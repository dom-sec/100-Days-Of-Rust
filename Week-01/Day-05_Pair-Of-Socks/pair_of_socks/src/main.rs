use std::collections::HashMap;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    
        let mut socks: HashMap<char, u32> = HashMap::new();
        for i in input.chars() {
            socks.entry(i).and_modify(|e| *e += 1).or_insert(1);
        }
        let mut pair_socks = 0;
        for (_, value) in socks.iter() {
            pair_socks += value / 2;
        }
        println!("{}", pair_socks);
    }
}
