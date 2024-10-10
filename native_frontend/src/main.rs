mod draw;
mod keyboard;

use draw::draw_screen;
use keyboard::handle_keypress;
use rustychip_core::*;

use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

use sdl2::event::Event;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 2 {
        println!("Usage: rustychip path/to/game int_clock_speed");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("RustyChip Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut emulator = RustyChip::new();

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    emulator.load_rom(&buffer);

    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    handle_keypress(&mut emulator, &keycode, true);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    handle_keypress(&mut emulator, &keycode, false);
                }
                Event::Quit { .. } => {
                    break 'gameloop;
                }
                _ => (),
            }
        }

        emulator.cycle(false);

        draw_screen(&emulator, &mut canvas);

        std::thread::sleep(Duration::new(
            0,
            1_000_000_000u32 / args[2].parse::<u32>().unwrap(),
        ));
    }
}
