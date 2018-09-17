//! Tiny crates that stops the console window form closing when the program finishes.

/// Using the `enter_to_continue` module makes is the simplest way of using this crate, however, the only key you can use with it is the enter key.
pub mod enter_to_continue {
    use std::io;
    /// ### Message then close with enter.
    /// Prompts user with message `"Press enter to close."`, waits for the user to press enter then ends to program (closing the window).
    /// Add
    /// ```rust
    /// extern crate dont_disappear;
    /// ```
    /// to the top of your file
    /// and
    /// ```rust
    /// dont_disappear::enter_to_continue::default();
    /// ```
    /// to where your program ends
    pub fn default() {
        custom_msg("Press enter to close.");
    }
    /// ### Custom message then close with enter.
    /// Prompts user with a custom message, waits for the user to press enter then ends to program (closing the window).
    /// Add
    /// ```rust
    /// extern crate dont_disappear;
    /// ```
    /// to the top of your file
    /// and
    /// ```rust
    /// dont_disappear::enter_to_continue::custom_msg("Your custom message.");
    /// ```
    /// to where your program ends
    pub fn custom_msg(msg: &str) {
        let mut input = String::new();
        println!("{}", msg);
        io::stdin().read_line(&mut input).unwrap();
    }
}
/// The `any_key_to_continue` module responds to any key press, however, can return strange characters when Ctrl-c or Delete keys are used.
pub mod any_key_to_continue {
    extern crate crossterm;
    use self::crossterm::input;
    use self::crossterm::Screen;
    /// ### Message then close with any key.
    /// Prompts user with message `"Press any key to continue"`, waits for the user to press a key then ends to program (closing the window).
    /// Add
    /// ```rust
    /// extern crate dont_disappear;
    /// ```
    /// to the top of your file
    /// and
    /// ```rust
    /// dont_disappear::any_key_to_continue::default();
    /// ```
    /// to where your program ends
    pub fn default() {
        custom_msg("Press any key to continue")
    }
    /// ### Message then close with any key.
    /// Prompts user with a custom message, waits for the user to press a key then ends to program (closing the window).
    /// Add
    /// ```rust
    /// extern crate dont_disappear;
    /// ```
    /// to the top of your file
    /// and
    /// ```rust
    /// dont_disappear::any_key_to_continue::custom_msg("Your custom message.");
    /// ```
    /// to where your program ends
    pub fn custom_msg(msg: &str) {
        println!("{}", msg);
        let screen = Screen::default();
        let input = input(&screen);

        match input.read_char() {
            Ok(_s) => (),
            Err(_e) => (),
        }
    }
}
