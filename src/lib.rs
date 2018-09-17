use std::io;
/// ### Close with enter.
/// Prompts user with message `"Press enter to close."`, waits for the user to press enter then ends to program (closing the window).
/// Add
/// ```rust
/// extern crate dont_disappear;
/// ```
/// to the top of your file
/// and
/// ```rust
/// dont_disappear::enter_to_continue();
/// ```
/// to where your program ends
pub fn enter_to_continue() {
    custom_enter_to_continue("Press enter to close.".to_string());
}
/// ### Close with enter.
/// Prompts user with a custom message, waits for the user to press enter then ends to program (closing the window).
/// Add
/// ```rust
/// extern crate dont_disappear;
/// ```
/// to the top of your file
/// and
/// ```rust
/// dont_disappear::custom_enter_to_continue("Your custom message.".to_string());
/// ```
/// to where your program ends
pub fn custom_enter_to_continue(msg: String) {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).unwrap();
}
