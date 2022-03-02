extern crate sdl2;

fn main() {
    // Initialize sdl
    // The _(underscore)  silences the warning about unused value
    let sdl = sdl2::init().unwrap();
    println!("[ SDL Started ]");

    // Initialize video subsystem
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
            .window("Playground", 900, 700)
            .resizable()
            .build()
            .unwrap();
    
    let mut event_pump = sdl.event_pump().unwrap();
    `main: loop {
        for event in event_pump.poll_iter() {
            // Handle user input
            match event {
                sdl2::event::Event::Quit {..} => break `main,
                _ => {},
            }
        }

        // render window contents

    }
}
