# Dont Disappear
Tiny crates that stops the console window form closing when the program finishes.
[![pipeline status](https://gitlab.com/efunb/dont_disappear/badges/master/pipeline.svg)](https://gitlab.com/efunb/noughts_and_crosses/commits/master)
[![License](https://img.shields.io/crates/l/dont_disappear.svg)](https://crates.io/crates/dont_disappear)
[![Latest version](https://img.shields.io/crates/v/dont_disappear.svg)](https://crates.io/crates/dont_disappear)
[![downloads-badge](https://img.shields.io/crates/d/dont_disappear.svg)](https://crates.io/crates/dont_disappear)

## Why you need it.

When making an app without a GUI sometimes you display some information before the program ends. If you send this program to someone and they run it in the windows command prompt or many other consoles the window may close before showing the data. This crate allows the user to wait until they have read the data before they close the window.

## Features

### Close with enter.

This waits for the user to press enter then ends to program (closing the window).

Add

```rust
extern crate dont_disappear;
```
to the top of your file and 

```rust
dont_disappear::enter_to_continue();
```
to where your program ends
## Docs

[API docs](https://docs.rs/dont_disappear/)

## Downloads

[Docs](https://gitlab.com/efunb/dont_disappear/-/jobs/artifacts/master/download?job=docs)


**If you are viewing this from GitHub then this is a read only copy. Please contribute to the GitLab copy [here](https://gitlab.com/efunb/dont_disappear).**

