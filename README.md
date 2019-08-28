# Don't Disappear
Tiny crate that prevents the console window from closing when the program finishes.
[![pipeline status](https://gitlab.com/efunb/dont_disappear/badges/master/pipeline.svg)](https://gitlab.com/efunb/noughts_and_crosses/commits/master)
[![License](https://img.shields.io/crates/l/dont_disappear.svg)](https://crates.io/crates/dont_disappear)
[![Latest version](https://img.shields.io/crates/v/dont_disappear.svg)](https://crates.io/crates/dont_disappear)
[![Latest Docs](https://docs.rs/dont_disappear/badge.svg)](https://docs.rs/dont_disappear/)
[![downloads-badge](https://img.shields.io/crates/d/dont_disappear.svg)](https://crates.io/crates/dont_disappear)

## Why you need it.

When making an app without a GUI sometimes you display some information before the program ends. If you send this program to someone and they run it in the windows command prompt or some other console the window may close before showing the data. This crate allows the user to wait until they have read the data before they close the window.

![Example of Don't Disappear in action](example.gif)
*Don't Disappear in action preventing data form disappearing when a program ends.*

## Help

If you run into any issues or need help with using `dont_disappear` in your project please email [incoming+efunb/dont_disappear@incoming.gitlab.com](mailto:incoming+efunb/dont_disappear@incoming.gitlab.com)

## How to use example.

This is one example of one of the ways you could use one of the `dont_disappear` functions.

```rust, no_run
extern crate dont_disappear;

fn main() {
    println!("Here is some data");
    dont_disappear::any_key_to_continue::default();
}
```


## Features

[Close with any key.](https://docs.rs/dont_disappear/2.1.8/dont_disappear/any_key_to_continue/index.html)

[Close with enter key.](https://docs.rs/dont_disappear/2.1.8/dont_disappear/enter_to_continue/index.html)

[Close with window manager.](https://docs.rs/dont_disappear/2.1.8/dont_disappear/fn.press_close.html)

## Examples

### Any key to continue
[Raw](https://gitlab.com/efunb/dont_disappear/raw/master/examples/any_key_to_continue.rs)

[Download example for Windows](https://gitlab.com/efunb/dont_disappear/-/jobs/artifacts/master/raw/files/any_key_to_continue.exe?job=windows-optimized)

[Download example for Linux](https://gitlab.com/efunb/dont_disappear/-/jobs/artifacts/master/raw/files/any_key_to_continue?job=linux-optimized)

```
cargo run --example any_key_to_continue
```

### Enter to continue
[Raw](https://gitlab.com/efunb/dont_disappear/raw/master/examples/enter_to_continue.rs)

[Download example for Windows](https://gitlab.com/efunb/dont_disappear/-/jobs/artifacts/master/raw/files/enter_to_continue.exe?job=windows-optimized)

[Download example for Linux](https://gitlab.com/efunb/dont_disappear/-/jobs/artifacts/master/raw/files/enter_to_continue?job=linux-optimized)

```
cargo run --example enter_to_continue
```

### Close with window manager.
[Raw](https://gitlab.com/efunb/dont_disappear/raw/master/examples/press_close.rs)

[Download example for Windows](https://gitlab.com/efunb/dont_disappear/-/jobs/artifacts/master/raw/files/press_close.exe?job=windows-optimized)

[Download example for Linux](https://gitlab.com/efunb/dont_disappear/-/jobs/artifacts/master/raw/files/press_close?job=linux-optimized)

```
cargo run --example press_close
```



## Docs

[API Documentation](https://docs.rs/dont_disappear/)

## Dependencies

This project uses crossterm in the `any_key_to_continue` module.

## Downloads

[All examples Windows](https://gitlab.com/efunb/dont_disappear/-/jobs/artifacts/master/download?job=windows-optimized)

[All examples Linux](https://gitlab.com/efunb/dont_disappear/-/jobs/artifacts/master/download?job=linux-optimized)

## **Warning**

**If you are viewing this from GitHub then this is a read only copy. Please contribute to the GitLab copy [here](https://gitlab.com/efunb/dont_disappear).**

