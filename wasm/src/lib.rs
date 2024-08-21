use rustychip_core::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use js_sys::Uint8Array;

#[wasm_bindgen]
pub struct RustyChipWasm {
    emulator: RustyChip,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl RustyChipWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<RustyChipWasm, JsValue> {
        let emulator = RustyChip::new();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas.get_context("2d")
            .unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Ok(RustyChipWasm { emulator, ctx })
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.emulator.reset();
    }

    #[wasm_bindgen]
    pub fn cycle(&mut self) {
        self.emulator.cycle(false);
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        self.emulator.cycle_timers();
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, evt: KeyboardEvent, pressed: bool) {
        let key = evt.key();
        if let Some(k) = map_key_to_button(&key) {
            self.emulator.keypress(k, pressed)
        }
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, data: Uint8Array) {
        self.emulator.reset();
        self.emulator.load_rom(&data.to_vec());
    }

    #[wasm_bindgen]
    pub fn draw(&mut self, scale: usize) {
        let disp = self.emulator.frontend_display();
        for i in 0..(SCREEN_SIZE) {
            if disp[i] {
                let x = i % SCREEN_WIDTH;
                let y = i / SCREEN_WIDTH;
                self.ctx.fill_rect(
                    (x * scale) as f64, 
                    (y * scale) as f64, 
                    scale as f64, 
                    scale as f64
                );
            }
        }
    }
}

fn map_key_to_button(key: &str) -> Option<usize> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ => None,
    }
}