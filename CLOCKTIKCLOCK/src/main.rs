use chrono::Local;
use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::{io::stdout, thread, time::Duration};

fn main() -> std::io::Result<()> {
    loop {
        let now = Local::now();
        let time_str = now.format("%H:%M:%S").to_string();

        // Clear the terminal
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        // Print styled clock
        execute!(
            stdout(),
            SetForegroundColor(Color::Cyan),
            Print(big_time(&time_str)),
            ResetColor
        )?;

        thread::sleep(Duration::from_millis(1000));
    }
}


fn big_time(time: &str) -> String {
    // Simple ASCII art digits
    let digits = [
        ["███", "█ █", "█ █", "█ █", "███"], // 0
        ["  █", "  █", "  █", "  █", "  █"], // 1
        ["███", "  █", "███", "█  ", "███"], // 2
        ["███", "  █", "███", "  █", "███"], // 3
        ["█ █", "█ █", "███", "  █", "  █"], // 4
        ["███", "█  ", "███", "  █", "███"], // 5
        ["███", "█  ", "███", "█ █", "███"], // 6
        ["███", "  █", "  █", "  █", "  █"], // 7
        ["███", "█ █", "███", "█ █", "███"], // 8
        ["███", "█ █", "███", "  █", "███"], // 9
        ["   ", " ░ ", "   ", " ░ ", "   "], // :
    ];

    let mut output = String::new();
    for row in 0..5 {
        for ch in time.chars() {
            let idx = match ch {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                ':' => 10,
                _ => continue,
            };
            output.push_str(digits[idx][row]);
            output.push(' '); // spacing
        }
        output.push('\n');
    }
    output
}
