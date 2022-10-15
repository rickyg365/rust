use chrono::{Local};

// underscore supresses error
const _DIGITS : [[&str; 11]; 7] = [
    ["┌───┐", "  ╷  ", "┌───┐", "┌───┐", "╷   ╷", "┌───┐", "┌─── ", "┌───┐", "┌───┐", "┌───┐", "     ", ],
    ["│   │", "  │  ", "    │", "    │", "│   │", "│    ", "│    ", "    │", "│   │", "│   │", "     ", ],
    ["│   │", "  │  ", "    │", "    │", "│   │", "│    ", "│    ", "    │", "│   │", "│   │", "  .  ", ],
    ["│   │", "  │  ", "┌───┘", " ───┤", "└───┤", "└───┐", "├───┐", "    │", "├───┤", "└───┤", "     ", ],
    ["│   │", "  │  ", "│    ", "    │", "    │", "    │", "│   │", "    │", "│   │", "    │", "  .  ", ],
    ["│   │", "  │  ", "│    ", "    │", "    │", "    │", "│   │", "    │", "│   │", "    │", "     ", ],
    ["└───┘", "  ╵  ", "└────", "└───┘", "    ╵", "└───┘", "└───┘", "    ╵", "└───┘", "└───┘", "     ", ],
];


fn main() {
    print!("\x1b[?25l"); // Hide Cursor
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
        print!("\x1b[7A"); // Move Cursor to the top
    }
}
