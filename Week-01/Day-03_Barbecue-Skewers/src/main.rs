fn print_row(s: &str) {
    println!("{}", s);
}

fn is_vegan(s: &str) -> bool {
    s.chars().all(|c| c == 'o' || c == '-')
}

fn check_barbecue(strings: &[&str]) {
    println!("");
    println!("Let's see what we got:");
    for s in strings {
        print_row(s);
    }

    let result = strings.iter().fold([0, 0], |mut acc, &row| {
        if is_vegan(row) {
            acc[0] += 1;
        } else {
            acc[1] += 1;
        }
        acc
    });

    println!("Results are {:?}", result)
}

fn main() {
    test_the_grill()
}

fn test_the_grill() {
    let barb_1 = [
        "--xo--x--ox--",
        "--xx--x--xx--",
        "--oo--o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--",
    ];
    let barb_2 = [
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ];
    let barb_3 = [
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----"
    ];

    check_barbecue(&barb_1);
    check_barbecue(&barb_2);
    check_barbecue(&barb_3);
}
