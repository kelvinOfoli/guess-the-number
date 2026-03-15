use colored::Colorize;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// ---------------------------------------------------------------------------
// Core utilities
// ---------------------------------------------------------------------------

pub fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

pub fn type_out(text: &str, delay_ms: u64) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for ch in text.chars() {
        write!(handle, "{}", ch).unwrap();
        handle.flush().unwrap();
        thread::sleep(Duration::from_millis(delay_ms));
    }
    writeln!(handle).unwrap();
}

/// Overwrite the previous line in-place (used by warmth bar updates).
pub fn clear_prev_line() {
    let mut stdout = io::stdout();
    stdout.execute(cursor::MoveUp(1)).unwrap();
    stdout
        .execute(terminal::Clear(terminal::ClearType::CurrentLine))
        .unwrap();
}

// ---------------------------------------------------------------------------
// Startup animation
// ---------------------------------------------------------------------------

pub fn startup_sequence() {
    let banner = r#"
  ██████╗ ██╗   ██╗███████╗███████╗███████╗
 ██╔════╝ ██║   ██║██╔════╝██╔════╝██╔════╝
 ██║  ███╗██║   ██║█████╗  ███████╗███████╗
 ██║   ██║██║   ██║██╔══╝  ╚════██║╚════██║
 ╚██████╔╝╚██████╔╝███████╗███████║███████║
  ╚═════╝  ╚═════╝ ╚══════╝╚══════╝╚══════╝
   ████████╗██╗  ██╗███████╗
   ╚══██╔══╝██║  ██║██╔════╝
      ██║   ███████║█████╗
      ██║   ██╔══██║██╔══╝
      ██║   ██║  ██║███████╗
      ╚═╝   ╚═╝  ╚═╝╚══════╝
  ███╗   ██╗██╗   ██╗███╗   ███╗██████╗ ███████╗██████╗
  ████╗  ██║██║   ██║████╗ ████║██╔══██╗██╔════╝██╔══██╗
  ██╔██╗ ██║██║   ██║██╔████╔██║██████╔╝█████╗  ██████╔╝
  ██║╚██╗██║██║   ██║██║╚██╔╝██║██╔══██╗██╔══╝  ██╔══██╗
  ██║ ╚████║╚██████╔╝██║ ╚═╝ ██║██████╔╝███████╗██║  ██║
  ╚═╝  ╚═══╝ ╚═════╝ ╚═╝     ╚═╝╚═════╝ ╚══════╝╚═╝  ╚═╝
"#;

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for ch in banner.chars() {
        let colored_char = match ch {
            '█' | '╔' | '╗' | '╚' | '╝' | '║' | '═' | '╠' | '╣' | '╦' | '╩' | '╬' => {
                format!("{}", ch.to_string().cyan().bold())
            }
            _ => format!("{}", ch),
        };
        write!(handle, "{}", colored_char).unwrap();
        handle.flush().unwrap();
        thread::sleep(Duration::from_millis(2));
    }
    drop(handle);

    println!();

    // Countdown 3-2-1
    for n in (1u8..=3).rev() {
        let label = match n {
            3 => "  3...".bold().white(),
            2 => "  2...".bold().yellow(),
            _ => "  1...".bold().bright_red(),
        };
        println!("{}", label);
        sleep_ms(400);
    }

    print!("  ");
    type_out("Get ready!", 60);
    sleep_ms(300);
    println!();
}

// ---------------------------------------------------------------------------
// Feedback animations
// ---------------------------------------------------------------------------

pub fn animate_too_low() {
    let arrows = "  ↑  ↑  ↑";
    let msg = " Too low! Guess higher!";

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for ch in arrows.chars() {
        write!(handle, "{}", ch.to_string().yellow().bold()).unwrap();
        handle.flush().unwrap();
        thread::sleep(Duration::from_millis(40));
    }
    drop(handle);

    type_out(&format!("{}", msg.yellow().bold()), 25);
    sleep_ms(100);
}

pub fn animate_too_high() {
    let arrows = "  ↓  ↓  ↓";
    let msg = " Too high! Guess lower!";

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for ch in arrows.chars() {
        write!(handle, "{}", ch.to_string().red().bold()).unwrap();
        handle.flush().unwrap();
        thread::sleep(Duration::from_millis(40));
    }
    drop(handle);

    type_out(&format!("{}", msg.red().bold()), 25);
    sleep_ms(100);
}

pub fn animate_invalid_input() {
    let frames = [
        "  !!! Please enter a valid number !!!",
        "  ??? Please enter a valid number ???",
        "  !!! Please enter a valid number !!!",
    ];
    for frame in &frames {
        print!("\r{}", frame.bright_red().bold());
        io::stdout().flush().unwrap();
        sleep_ms(180);
    }
    println!();
}

// ---------------------------------------------------------------------------
// Warmth meter
// ---------------------------------------------------------------------------

pub fn warmth_percentage(guess: u32, secret: u32) -> u8 {
    let distance = (guess as i64 - secret as i64).unsigned_abs() as u32;
    // Max possible distance on 1-100 range is 99
    let warmth = 100u32.saturating_sub(distance * 100 / 99);
    warmth.min(100) as u8
}

pub fn animate_warmth_bar(warmth: u8, is_first: bool) {
    if !is_first {
        clear_prev_line();
    }

    let total_blocks = 20usize;
    let filled = (warmth as usize * total_blocks / 100).min(total_blocks);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let label = "  Warmth: [";
    write!(handle, "{}", label.dimmed()).unwrap();
    handle.flush().unwrap();

    for i in 0..total_blocks {
        let block = if i < filled { "█" } else { "░" };
        let colored_block = if warmth <= 30 {
            block.cyan().bold().to_string()
        } else if warmth <= 69 {
            block.yellow().bold().to_string()
        } else {
            block.bright_red().bold().to_string()
        };
        write!(handle, "{}", colored_block).unwrap();
        handle.flush().unwrap();
        thread::sleep(Duration::from_millis(30));
    }

    writeln!(handle, "{}  {}%", "]".dimmed(), warmth).unwrap();
}

// ---------------------------------------------------------------------------
// Victory celebration
// ---------------------------------------------------------------------------

const STAR_SMALL: &[&str] = &[
    "         *    .  *       .",
    "    .  *    *    .    *   ",
    "  *    .  *    *    .  *  ",
    "    *    .    *    *      ",
    "  .    *    .    *    .   ",
];

const STAR_BIG: &[&str] = &[
    "  ✦  ·  ★  ·  ✦  ·  ★  ·  ✦",
    "·  ✧  ★  ·  ✦  ★  ·  ✧  ·  ",
    "  ★  ·  ✦  ·  ★  ·  ✦  ·  ★",
    "·  ✦  ·  ★  ·  ✦  ·  ★  ·  ",
    "  ✧  ★  ·  ✦  ·  ★  ·  ✧  · ",
    "·  ·  ✦  ★  ·  ✦  ★  ·  ·  ",
];

fn print_celebration_frame(lines: &[&str], colors: &[&str]) {
    let palette = [
        |s: &str| s.yellow().bold().to_string(),
        |s: &str| s.cyan().bold().to_string(),
        |s: &str| s.bright_magenta().bold().to_string(),
        |s: &str| s.bright_green().bold().to_string(),
        |s: &str| s.bright_red().bold().to_string(),
    ];
    let _ = colors; // colors param reserved for future use
    for (i, line) in lines.iter().enumerate() {
        let colored = palette[i % palette.len()](line);
        println!("{}", colored);
    }
}

pub fn win_celebration(attempts: u32) {
    let excellent = attempts <= 5;

    println!();

    if excellent {
        // Enhanced multi-frame burst for excellent performance
        let frames = 5;
        for frame in 0..frames {
            // Clear previous frame lines (6 lines of big stars)
            if frame > 0 {
                for _ in 0..STAR_BIG.len() {
                    clear_prev_line();
                }
            }
            let offset = frame % 2;
            let shifted: Vec<&str> = STAR_BIG
                .iter()
                .enumerate()
                .map(|(i, l)| if (i + offset) % 2 == 0 { l } else { *l })
                .collect();
            print_celebration_frame(&shifted, &[]);
            sleep_ms(150);
        }
        // Clear last frame
        for _ in 0..STAR_BIG.len() {
            clear_prev_line();
        }
    } else {
        // Standard animation
        let frames = 4;
        for frame in 0..frames {
            if frame > 0 {
                for _ in 0..STAR_SMALL.len() {
                    clear_prev_line();
                }
            }
            print_celebration_frame(STAR_SMALL, &[]);
            sleep_ms(200);
        }
        for _ in 0..STAR_SMALL.len() {
            clear_prev_line();
        }
    }

    // Big YOU WIN banner
    let win_lines = if excellent {
        vec![
            "╦ ╦  ╔═╗  ╦ ╦    ╦ ╦  ╦  ╔╗╦",
            "╚╦╝  ║ ║  ║ ║    ║║║  ║  ║╚╣",
            " ╩   ╚═╝  ╚═╝    ╚╩╝  ╩  ╩ ╩",
            "",
            "        ★  LEGENDARY  ★",
        ]
    } else {
        vec![
            "╦ ╦  ╔═╗  ╦ ╦    ╦ ╦  ╦  ╔╗╦",
            "╚╦╝  ║ ║  ║ ║    ║║║  ║  ║╚╣",
            " ╩   ╚═╝  ╚═╝    ╚╩╝  ╩  ╩ ╩",
        ]
    };

    for (i, line) in win_lines.iter().enumerate() {
        let colored = match i % 3 {
            0 => line.bright_yellow().bold().to_string(),
            1 => line.cyan().bold().to_string(),
            _ => line.bright_green().bold().to_string(),
        };
        println!("{}", colored);
        sleep_ms(80);
    }

    println!();
    sleep_ms(300);
}
