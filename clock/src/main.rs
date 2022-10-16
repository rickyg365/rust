use chrono::{Local};

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


fn main() {
    print!("\x1b[?25l"); // Hide Cursor

    let mut iter = 0;
    let max_iter = prompt("How many seconds to run clock: ").parse::<i32>().unwrap_or(60);

    loop {
        // print!("\x1b[3J"); // Erase Display use 2 or 3
        let t = Local::now();
        let time: String = t.format("%I:%M:%S").to_string();
        
        for row in &_DIGITS {
            for c in time.chars() {

                let col = match c {
                    '0'..='9' => c as usize - '0' as usize,
                    ':' => 10,
                    _ => 10,
                }; // 0-9
                print!("{} ", row[col]);

            }
            println!(); // blank line
        }
        // Pause for 990ms
        std::thread::sleep(std::time::Duration::from_millis(990));
        print!("\x1b[5A"); // Move Cursor to the top
        if iter >= max_iter {
            break;
        }
        iter += 1;
    }
    print!("\x1b[5B"); // Move Cursor to the bottom
    println!(); // blank line
}