use std::io;

// Get user input
pub fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

// Find Nemo
// TODO make sure Nemo is a separate word?
pub fn find_nemo(buffer: &str) {

    let bytes = buffer.trim().as_bytes();
    let mut iter = bytes.iter().peekable();
    let mut found: bool = false;
    let mut index = 0_usize;

    println!("Index at the start is {index}");

    while iter.peek() != None {
        if iter.peek() == Some(&&b'N') {
            iter.next();
            if iter.peek() == Some(&&b'e') {
                iter.next();
                if iter.peek() == Some(&&b'm') {
                    iter.next();
                    if iter.peek() == Some(&&b'o') {
                        iter.next();
                        if iter.peek() == Some(&&b' ') {
                            found = true;
                            // Save string slice before nemo
                            let slice: &str = &buffer[..index];
                            println!("Here is the strin slice {slice}");
                            // Split into iterator based on whitespaces
                            let split = slice.split(' ');
                            // Count number of words, next is nemo (we don't have to add one
                            // because count consumes None at the end of the iterator
                            let place_of_nemo: usize = split.count();

                            println!("Found Nemo at {place_of_nemo}!");

                        }
                    }
                }
            }
        } else {
            iter.next();
            index += 1_usize;
        }
    }

    if !found {
        println!("I can't find Nemo :(");
    }
}
