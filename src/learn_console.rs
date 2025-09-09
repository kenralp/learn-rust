use std::io;

pub fn entry() {
    println!("[from: learn_console]");
    println!("Enter your input:");

    let mut input_buffer = String::new();

    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Show this if read_line() failed");

    println!("You entered: {input_buffer}");
}
