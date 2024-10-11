"use strict";

import init from "./wasm.js";
import Chip8Wrapper from "./wrapper.js";


async function run() {
    await init();

    let emulator = new Chip8Wrapper();

    document.addEventListener("keydown",  async function(evt) {
        emulator.system.keypress(evt, true);
    });

    document.addEventListener("keyup",  async function(evt) {
        emulator.system.keypress(evt, false);
    });

    document.getElementById("clock-rate").addEventListener("change",  async function(e) {
        emulator.clock_speed = 1.0 / parseFloat( e.target.value ) * 1000.0;
    });

    document.getElementById("rom").addEventListener("change", async function(evt) {
        if (emulator.anim_frame != 0) {
            window.cancelAnimationFrame(emulator.anim_frame);
        }

        let file_name = evt.target.value;
        const response = await fetch(`./assets/roms/${file_name}`);
        const arrayBuffer = await response.arrayBuffer();
        const romBuffer = new Uint8Array(arrayBuffer);

        emulator.system.load_rom(romBuffer);
        
        requestAnimationFrame((time) => emulator.mainloop(time));
    });
}

run().catch(console.error);