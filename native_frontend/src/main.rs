mod cli;
mod io;

use clap::Parser;

use io::SDLState;
use rustychip_core::*;

use std::time::Duration;

use sdl2::event::Event;

fn main() {
    let args = cli::Cli::parse();
    let mut render_state = SDLState::init();
    let mut emulator = RustyChip::new();

    io::load_rom(&args.path, &mut emulator);

    'gameloop: loop {
        for evt in render_state.poll_event_pump() {
            match evt {
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    io::handle_keypress(&mut emulator, &keycode, true);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    io::handle_keypress(&mut emulator, &keycode, false);
                }
                Event::Quit { .. } => {
                    break 'gameloop;
                }
                _ => (),
            }
        }

        emulator.cycle(false);

        render_state.draw_screen(&emulator);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / args.clock_speed));
    }
}
