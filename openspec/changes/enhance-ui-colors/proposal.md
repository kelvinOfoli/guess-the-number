## Why

The guessing game currently uses plain `println!` output with no visual distinction between prompts, hints, and results. Adding terminal colors and styled output will make the game more engaging and easier to read at a glance.

## What Changes

- Add the `colored` crate as a dependency for ANSI terminal color support
- Color-code all game output: prompts in cyan, "too low" hints in yellow, "too high" hints in red, and the win message in bold green
- Add a styled ASCII banner/header on game start
- Display attempt count with visual emphasis on win

## Capabilities

### New Capabilities

- `colored-output`: Colored, styled terminal output for all game messages using the `colored` crate

### Modified Capabilities

- (none — no existing spec-level behavior changes)

## Impact

- `src/main.rs`: All `println!` calls updated to use colored string methods
- `Cargo.toml`: New dependency on `colored` crate
- No breaking changes to game logic or behavior; purely cosmetic
