extern crate sdl2;
extern crate gl;

fn main() {
    // Initialize sdl
    // The _(underscore)  silences the warning about unused value
    let sdl = sdl2::init().unwrap();
    println!("[ SDL Started ]");

    // Initialize video subsystem
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
            .window("Playground", 900, 700)
            .opengl() // add opengl flag
            .resizable()
            .build()
            .unwrap();

    // Set up OpenGL, needs .opengl() call on window struct above
    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            // Handle user input
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // render window contents
        window.gl_swap_window();
    }
}
