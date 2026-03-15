## 1. Dependencies & Module Setup

- [x] 1.1 Add `crossterm` to `Cargo.toml` dependencies
- [x] 1.2 Create `src/animation.rs` module file
- [x] 1.3 Add `mod animation;` declaration in `src/main.rs`

## 2. Core Animation Utilities

- [x] 2.1 Implement `type_out(text: &str, delay_ms: u64)` function in `animation.rs` — prints text character by character with `stdout().flush()` after each char
- [x] 2.2 Implement `sleep_ms(ms: u64)` convenience wrapper around `std::thread::sleep`
- [x] 2.3 Implement `clear_line()` utility using `crossterm` to move cursor up and clear the current line (for warmth bar in-place updates)

## 3. Startup Animation

- [x] 3.1 Implement `startup_sequence()` in `animation.rs` — ASCII-art title banner that types in letter by letter
- [x] 3.2 Add the 3-2-1 countdown to `startup_sequence()` with ~350ms pause between each number
- [x] 3.3 Add intro flavor text reveal ("I'm thinking of a number between 1 and 100") using `type_out()` after the countdown
- [x] 3.4 Call `startup_sequence()` at the top of `main()` before the game loop

## 4. Animated Guess Feedback

- [x] 4.1 Implement `animate_too_low()` in `animation.rs` — yellow "Too low! ↑↑↑" with progressive character reveal
- [x] 4.2 Implement `animate_too_high()` in `animation.rs` — red "Too high! ↓↓↓" with progressive character reveal
- [x] 4.3 Implement `animate_invalid_input()` in `animation.rs` — flashing/repeated "!!! Please enter a valid number !!!" effect
- [x] 4.4 Replace static `println!` feedback calls in `main.rs` with `animate_too_low()`, `animate_too_high()`, and `animate_invalid_input()`

## 5. Warmth Meter

- [x] 5.1 Implement `warmth_percentage(guess: u32, secret: u32) -> u8` — maps distance to a 0-100 warmth value (closer = higher)
- [x] 5.2 Implement `animate_warmth_bar(warmth: u8, is_first: bool)` — renders the `[████░░░░░░]` bar filling left to right; uses `clear_line()` when `is_first` is false to overwrite previous bar
- [x] 5.3 Apply warmth-based color (blue ≤30%, yellow 31-69%, red ≥70%) to the bar using `colored`
- [x] 5.4 Call `animate_warmth_bar()` after each incorrect guess in `main.rs`, passing a flag to track whether it's the first guess

## 6. Victory Celebration

- [x] 6.1 Implement `win_celebration(attempts: u32)` in `animation.rs` — multi-frame ASCII starburst or confetti animation
- [x] 6.2 Add attempt-based intensity branching: ≤5 attempts triggers an enhanced (larger/more colorful) animation, >5 triggers standard
- [x] 6.3 After the celebration frames, use `type_out()` to reveal the final "Correct! You got it in N attempt(s)!" message character by character
- [x] 6.4 Replace the static win `println!` in `main.rs` with `win_celebration(attempts)`

## 7. Polish & Testing

- [x] 7.1 Play through the game end-to-end and verify all animations trigger at the right moments
- [x] 7.2 Confirm warmth bar overwrites in place (no duplicate lines) on guesses 2+
- [x] 7.3 Confirm startup animation completes fully before the first prompt appears
- [x] 7.4 Check that `cargo build --release` compiles cleanly with no warnings
