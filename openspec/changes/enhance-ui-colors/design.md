## Context

The game is a single-file Rust CLI app (`src/main.rs`) using `rand` for random number generation. All output is plain text via `println!`. The `colored` crate is the de facto standard for ANSI terminal colors in Rust — it extends `&str` and `String` with chainable color/style methods (`.red()`, `.bold()`, etc.) and respects the `NO_COLOR` environment variable automatically.

## Goals / Non-Goals

**Goals:**
- Add color and style to all game output without altering game logic
- Use a well-maintained, idiomatic crate with minimal API surface
- Keep the change isolated to `main.rs` and `Cargo.toml`

**Non-Goals:**
- Custom color themes or user-configurable colors
- Windows CMD support (ANSI colors work on Windows Terminal / modern environments)
- Animated or interactive UI elements (ncurses-style)

## Decisions

**Use `colored` crate over manual ANSI escape codes**
The `colored` crate provides readable, chainable method syntax (`.red().bold()`) vs. raw escape strings like `\x1b[31m`. It also handles `NO_COLOR` env var and TTY detection. Alternative: `owo-colors` (zero-dependency, const-friendly) — not chosen because `colored` is more widely known and its API is simpler for this scale of change.

**Color scheme**
- Header/banner: bold cyan
- Prompt text: cyan
- "Too low" hint: yellow (warm, upward feel)
- "Too high" hint: red (stop/down signal)
- Invalid input warning: bright red
- Win message: bold green
- Attempt count in win: bold white

## Risks / Trade-offs

- [Terminal compatibility] Some terminals don't support ANSI colors → Mitigation: `colored` auto-disables when output is not a TTY or `NO_COLOR` is set
- [Dependency addition] Adds a crate to the project → Mitigation: `colored` is stable, widely used, and has no transitive dependencies beyond `lazy_static` and `atty`

## Migration Plan

1. Add `colored = "2"` to `Cargo.toml`
2. Add `use colored::Colorize;` import in `main.rs`
3. Replace each `println!` string literal with a colored equivalent
4. Run `cargo run` to verify visual output

No rollback complexity — purely additive dependency and cosmetic code change.
