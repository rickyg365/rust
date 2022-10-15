// Idea for future: https://stackoverflow.com/questions/73867885/how-to-capture-to-a-variable-the-ansi-escape-sequence-response-generated-by-prin
// https://notes.burke.libbey.me/ansi-escape-codes/

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use chrono::{Local, DateTime};
use std::io::Write;

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    return line.trim().to_string()
}

// underscore supresses error
const _DIGITS : [[&str; 11]; 5] = [
    ["╭───╮", "  ╷  ", "╭───╮", "╭───╮", "╷   ╷", "┌───┐", "╭─── ", "┌───┐", "╭───╮", "╭───╮", "     ", ],
    ["│   │", "  │  ", "    │", "    │", "│   │", "│    ", "│    ", "    │", "│   │", "│   │", "  .  ", ],
    ["│   │", "  │  ", "╭───╯", " ───┤", "└───┤", "└───╮", "├───╮", "    │", "├───┤", "╰───┤", "     ", ],
    ["│   │", "  │  ", "│    ", "    │", "    │", "    │", "│   │", "    │", "│   │", "    │", "  .  ", ],
    ["╰───╯", "  ╵  ", "└────", "╰───╯", "    ╵", "└───╯", "╰───╯", "    ╵", "╰───╯", "╰───╯", "     ", ],
];


#[derive(Debug)]
struct Timer {
    hour: i64,
    minute: i64,
    seconds: i64,
    start_time: DateTime<Local>,
}

impl Timer {
    fn check_timer(&self, current_time: DateTime<Local>) -> (i64, i64, i64) {
        let t = current_time.timestamp() - self.start_time.timestamp();

        let h = t/3600;
        let m_s = t % 3600;

        let m = m_s/60;
        let s = m_s%60;

        if h >= self.hour && m >= self.minute && s > self.seconds {
            return (-1, -1, -1)
        }

        return (h, m, s);
    }
}


fn main() {
    print!("\x1b[?25l"); // Hide Cursor

    let n_timer = Timer{
        hour: prompt("Hours: ").parse::<i64>().unwrap_or(0), 
        minute: prompt("Minutes: ").parse::<i64>().unwrap_or(0), 
        seconds: prompt("Seconds: ").parse::<i64>().unwrap_or(30), 
        start_time: Local::now()
    };

    loop {
        // print!("\x1b[3J"); // Erase Display use 2 or 3

        let (hr, min, sec) = n_timer.check_timer(Local::now());
    
        if (hr, min, sec) == (-1,-1,-1) {
            break;
        }

        let time_str = format!("{:02}:{:02}:{:02}", hr, min, sec);
        
        // Build lines, char by char
        for row in &_DIGITS {
            for c in time_str.chars() {
                let col = match c {
                    '0'..='9' => c as usize - '0' as usize,
                    ':' => 10,
                    _ => 10,
                }; // 0-9, :
                print!("{} ", row[col]);
            }
            println!(); // blank line
        }

        // Pause for 990ms
        std::thread::sleep(std::time::Duration::from_millis(990));
        print!("\x1b[5A"); // Move Cursor to the top
    }
    print!("\x1b[5B"); // Move Cursor to the bottom
    println!(); // blank line
}
