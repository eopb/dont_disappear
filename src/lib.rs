//Use mod to namespace diffent types
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
pub mod any_key_to_continue {
    extern crate crossterm;
    use self::crossterm::input;
    use self::crossterm::Screen;
    /// Ctrl-c and delete
    pub fn default() {
        custom_msg("Press any key to continue")
    }

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
