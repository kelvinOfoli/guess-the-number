## Why

The current guessing game is functional but static — text appears instantly with no visual feedback, making the terminal experience feel flat and disengaging. Adding animations and dynamic responses will make the game feel alive, reactive, and fun to play.

## What Changes

- Add an animated startup sequence (banner, countdown, or dramatic reveal) when the game launches
- Add character-by-character typing animations for key prompts and messages
- Add animated feedback for guesses (e.g., pulsing "Too high!" / "Too low!" with directional indicators)
- Add a win celebration animation (confetti-style ASCII art, flashing text, or expanding pattern)
- Add a progress indicator showing how close the player is getting (animated "warmth" meter)
- Add subtle input cursor animation or prompt decoration while waiting for user input
- Animate the attempt counter updating after each guess

## Capabilities

### New Capabilities
- `startup-animation`: Animated intro sequence displayed when the game begins (banner reveal, title animation, countdown)
- `animated-feedback`: Dynamic, motion-based responses to user guesses (typing effect, color transitions, directional cues)
- `win-celebration`: Victory animation sequence displayed when the player guesses correctly
- `warmth-meter`: Animated proximity indicator showing how close the current guess is to the secret number

### Modified Capabilities
<!-- No existing spec-level behavior is changing — this change adds new UI layers on top of existing game logic -->

## Impact

- `src/main.rs`: Core game loop will be extended with animation calls
- `Cargo.toml`: May need additional crates for terminal control (e.g., `crossterm`, `indicatif`, or `spinners`) and timing (`std::thread::sleep` / async)
- Terminal rendering: animations use ANSI escape codes; behavior may vary on non-ANSI terminals
- No changes to game logic (random number generation, guess comparison, attempt counting)
