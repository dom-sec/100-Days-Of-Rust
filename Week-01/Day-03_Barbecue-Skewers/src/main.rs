fn check_barbecue_row(s: &str) {
    println!("{}", s);
}

fn check_barbecue(strings: &[&str]) {
    println!("Let's see what we got:");
    for s in strings {
        check_barbecue_row(s);
    }
    println!("")
}

fn main() {
    // let mut num: u8 = 1; // max 255

    // loop {
    //     if num == 255 {
    //         println!("-> OK, exiting!");
    //         break; // Exit this loop
    //     }

    //     num = num + 1; // increase the loop count
    // }
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
