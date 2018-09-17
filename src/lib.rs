use std::io;

pub fn enter_to_continue() {
    let mut input = String::new();
    println!("Press enter to close.");
    io::stdin().read_line(&mut input).unwrap();
}
