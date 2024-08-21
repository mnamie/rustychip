use rustychip_core::*;

use sdl2::keyboard::Keycode;


pub fn handle_keypress(emulator: &mut RustyChip, key: &Keycode, pressed: bool) {
    match key {
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
        _ => return,
    }
}
