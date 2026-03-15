## 1. Add Dependency

- [x] 1.1 Add `colored = "2"` to `[dependencies]` in `Cargo.toml`
- [x] 1.2 Run `cargo build` to fetch and compile the `colored` crate

## 2. Update Source Code

- [x] 2.1 Add `use colored::Colorize;` import at the top of `src/main.rs`
- [x] 2.2 Style the game header: apply `.bold().cyan()` to the "=== Guess the Number! ===" string
- [x] 2.3 Style the range prompt: apply `.cyan()` to the "I'm thinking of a number..." string
- [x] 2.4 Style the input prompt: apply `.cyan()` to the "Enter your guess:" string
- [x] 2.5 Style the "Too low!" feedback: apply `.yellow()` to the too-low message
- [x] 2.6 Style the "Too high!" feedback: apply `.red()` to the too-high message
- [x] 2.7 Style the win message: apply `.bold().green()` to the correct-guess message and `.bold().white()` to the attempt count
- [x] 2.8 Style the invalid input warning: apply `.bright_red()` to the "Please enter a valid number!" message

## 3. Verify

- [x] 3.1 Run `cargo run` and manually verify all colors and styles display correctly in the terminal
- [x] 3.2 Run `cargo run | cat` and verify output is plain text with no ANSI escape codes (graceful degradation)
