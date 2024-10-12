use rustychip_core::*;

use sdl2::event::EventPollIterator;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use std::fs::File;
use std::io::Read;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

pub struct SDLState {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl SDLState {
    pub fn init() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let window = sdl_context
            .video()
            .expect("Failed to initialize video subsystem")
            .window("RustyChip Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .expect("Failed to initialize window");

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        Self { canvas, event_pump }
    }

    pub fn poll_event_pump(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }

    pub fn draw_screen(&mut self, emulator: &RustyChip) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        let screen_buffer = emulator.frontend_display();

        self.canvas.set_draw_color(Color::RGB(48, 200, 0));

        for (i, pixel) in screen_buffer.iter().enumerate() {
            if *pixel {
                let x = (i % SCREEN_WIDTH) as u32;
                let y = (i / SCREEN_WIDTH) as u32;
                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                self.canvas.fill_rect(rect).unwrap();
            }
        }
        self.canvas.present();
    }
}

pub fn handle_keypress(emulator: &mut RustyChip, key: &Keycode, pressed: bool) {
    match *key {
        Keycode::Num1 => emulator.keypress(0x1, pressed),
        Keycode::Num2 => emulator.keypress(0x2, pressed),
        Keycode::Num3 => emulator.keypress(0x3, pressed),
        Keycode::Num4 => emulator.keypress(0xc, pressed),
        Keycode::Q => emulator.keypress(0x4, pressed),
        Keycode::W => emulator.keypress(0x5, pressed),
        Keycode::E => emulator.keypress(0x6, pressed),
        Keycode::R => emulator.keypress(0xD, pressed),
        Keycode::A => emulator.keypress(0x7, pressed),
        Keycode::S => emulator.keypress(0x8, pressed),
        Keycode::D => emulator.keypress(0x9, pressed),
        Keycode::F => emulator.keypress(0xE, pressed),
        Keycode::Z => emulator.keypress(0xA, pressed),
        Keycode::X => emulator.keypress(0x0, pressed),
        Keycode::C => emulator.keypress(0xB, pressed),
        Keycode::V => emulator.keypress(0xF, pressed),
        _ => (),
    }
}

pub fn load_rom(rom_path: &str, emulator: &mut RustyChip) {
    let mut rom = File::open(rom_path).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)
        .expect("Unable to read rom into buffer");
    emulator.load_rom(&buffer);
}
