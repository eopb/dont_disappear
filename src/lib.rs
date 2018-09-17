use std::io;
/// ### Close with enter.
/// This waits for the user to press enter then ends to program (closing the window).
/// Add
/// ```rust
/// extern crate dont_disappear;
/// ```
/// to the top of your file and
/// ```rust
/// dont_disappear::enter_to_continue();
/// ```
/// to where your program ends
pub fn enter_to_continue() {
    let mut input = String::new();
    println!("Press enter to close.");
    io::stdin().read_line(&mut input).unwrap();
}
