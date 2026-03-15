## Context

The game is a single-file Rust CLI app (`src/main.rs`) that uses `colored` for ANSI color output and `rand` for random number generation. Currently all output is instant — `println!` calls with no timing, motion, or progressive reveal. The terminal is blocking (stdin reads synchronously), and the game loop is a simple `loop {}` with `read_line`.

Adding animations in a blocking terminal context means using `std::thread::sleep` for timing, `print!` + `stdout().flush()` for character-by-character output, and ANSI escape codes (via `crossterm`) for cursor control and screen effects.

## Goals / Non-Goals

**Goals:**
- Animated startup banner when the game launches
- Typing/reveal effect for key messages (intro text, feedback lines)
- Reactive animated feedback for "Too high" / "Too low" with directional ASCII motion
- Animated warmth indicator (proximity bar) that updates after each guess
- Victory celebration animation on correct guess
- Keep all animation non-blocking from the user's perspective (animations are short, then yield to input)

**Non-Goals:**
- Full TUI rewrite (no ncurses/ratatui full-screen layout)
- Async runtime (tokio/async-std) — overkill for a simple CLI game
- Sound effects
- Cross-platform terminal feature detection / graceful degradation
- Persistent high-score or save state

## Decisions

### Decision 1: Use `crossterm` for terminal control

**Chosen**: Add `crossterm` as a dependency for cursor movement, clearing lines, and raw-mode-free ANSI control.

**Rationale**: `colored` only handles color. For animations that overwrite lines, move the cursor, or clear parts of the screen, we need `crossterm`. It's the de-facto standard for Rust terminal manipulation, is cross-platform, and doesn't require enabling raw mode for basic use.

**Alternative considered**: Hand-roll ANSI escape codes (`\x1b[...]`). Works on Unix but fragile, non-portable, harder to read.

### Decision 2: Synchronous `thread::sleep`-based animation

**Chosen**: Use `std::thread::sleep(Duration::from_millis(...))` between animation frames.

**Rationale**: The game is inherently synchronous (blocking stdin read). Introducing an async runtime for cosmetic animation would be massive over-engineering. Sleep-based frame stepping is simple, readable, and appropriate for short animations (< 2 seconds).

**Alternative considered**: `tokio` async with `tokio::time::sleep`. Rejected — no benefit for a single-threaded, single-user CLI game.

### Decision 3: Animation helpers extracted into a module

**Chosen**: Create `src/animation.rs` with pure functions: `type_out()`, `animate_feedback()`, `startup_sequence()`, `win_celebration()`, `warmth_bar()`.

**Rationale**: Keeps `main.rs` clean. Functions are stateless and easy to test/adjust independently. Avoids dumping all animation logic inline into the game loop.

**Alternative considered**: Keep everything in `main.rs`. Rejected — `main.rs` would become unreadable noise.

### Decision 4: Warmth meter as an overwritten single line

**Chosen**: After each guess, print a warmth bar `[████░░░░░░] 40% warm` on a dedicated line, then overwrite it on the next guess using `crossterm::cursor::MoveUp` + `crossterm::terminal::Clear(ClearType::CurrentLine)`.

**Rationale**: Gives a persistent, updating indicator without scrolling the terminal. Simple to implement with crossterm line-clearing primitives.

**Alternative considered**: Re-print the bar each time without clearing (appends to scroll). Rejected — messy output.

### Decision 5: Startup animation uses ASCII art title + character reveal

**Chosen**: The game opens with a large ASCII-art title that "types in" letter by letter, followed by a 3-2-1 countdown before the game begins.

**Rationale**: Immediately signals "this is a polished experience." Sets the tone. Short enough (< 3s) not to frustrate repeat players.

**Alternative considered**: Simple spinner while "generating" the secret number. Works but less dramatic and doesn't brand the game.

## Risks / Trade-offs

- **Animation timing feels slow on fast readers** → Keep delays short (20-40ms per character for typing, 300ms per countdown step); add a `--fast` or `--no-anim` flag as a follow-up if needed
- **Cursor artifacts if user Ctrl-C mid-animation** → Accept for now; a signal handler to restore cursor is a future improvement
- **`crossterm` adds a compile dependency** → Small and well-maintained; acceptable trade-off for the quality improvement
- **Line-overwrite approach breaks if terminal width is very narrow** → Warmth bar will truncate gracefully if we clamp it to `min(terminal_width, 40)` columns

## Migration Plan

1. Add `crossterm` to `Cargo.toml`
2. Create `src/animation.rs` with all animation functions
3. Update `src/main.rs` to `mod animation;` and replace static `println!` calls with animated equivalents
4. Test in a standard terminal (iTerm2/Terminal.app on macOS, Windows Terminal on Windows)
5. No rollback needed — pure additive change to a local CLI tool

## Open Questions

- Should there be a `--no-anim` flag from the start, or add it only if players complain? (Suggest: skip for now, add later)
- Should the warmth meter reveal the "temperature zone" (e.g., within 10, within 25) or keep it abstract? (Suggest: keep abstract to avoid giving away too much)
