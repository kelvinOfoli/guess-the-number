# Guess The Number

A polished, terminal-based number guessing game written in Rust — complete with animated ASCII art, colorful feedback, a live warmth meter, and a victory celebration.

## Features

- **Animated startup** — ASCII art banner streams in character-by-character with a 3-2-1 countdown
- **Warmth meter** — a live progress bar updates each guess, color-coded from cold (cyan) to hot (red)
- **Directional hints** — animated arrows and colored messages tell you to guess higher or lower
- **Victory celebration** — animated star burst and a "YOU WIN" banner on correct guess; a **LEGENDARY** badge if you crack it in 5 attempts or fewer
- **Input validation** — invalid entries trigger a flashing error prompt without breaking the game loop
- **EOF handling** — gracefully exits when input is closed (e.g. piped input)

## Demo

```
  ██████╗ ██╗   ██╗███████╗███████╗███████╗
 ██╔════╝ ██║   ██║██╔════╝██╔════╝██╔════╝
  ...
  3...
  2...
  1...
  Get ready!

Enter your guess: 50
  ↑  ↑  ↑ Too low! Guess higher!
  Warmth: [████████░░░░░░░░░░░░]  42%

Enter your guess: 75
  ↓  ↓  ↓ Too high! Guess lower!
  Warmth: [████████████████░░░░]  80%

Enter your guess: 68
  ✦  ·  ★ ...
  YOU WIN
  Correct! You got it in 3 attempt(s)!
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

### Run

```bash
git clone <repo-url>
cd guessing_game
cargo run
```

## How to Play

1. The game picks a secret number between **1 and 100**
2. Type your guess and press Enter
3. Use the hints and warmth bar to zero in on the answer
4. Win by guessing the exact number — try to do it in **5 or fewer** attempts for a legendary finish!

## Dependencies

| Crate | Purpose |
|---|---|
| [`rand`](https://crates.io/crates/rand) | Random number generation |
| [`colored`](https://crates.io/crates/colored) | Terminal text colors |
| [`crossterm`](https://crates.io/crates/crossterm) | Cursor/terminal control for in-place animation |

## Project Structure

```
src/
├── main.rs        # Game loop and core logic
└── animation.rs   # All terminal animations and visual effects
```
