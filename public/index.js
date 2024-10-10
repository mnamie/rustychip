import init, * as wasm from "./wasm.js";

const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 15;

class Chip8Wrapper {
    constructor() {
        this.anim_frame = 0;
        this.system = new wasm.RustyChipWasm();
        this.clock_speed = 1.0 / 100.0 * 1000;
        this.previousTime = 0.0;
        this.deltaTime = 0.0;
    }
}

const canvas = document.getElementById("canvas");
canvas.width = WIDTH * SCALE;
canvas.height = HEIGHT * SCALE;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

const input = document.getElementById("rom");

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

    input.addEventListener("change", async function(evt) {
        if (emulator.anim_frame != 0) {
            window.cancelAnimationFrame(emulator.anim_frame);
        }

        let file_name = evt.target.value;
        const response = await fetch(`./assets/roms/${file_name}`);
        const arrayBuffer = await response.arrayBuffer();
        const romBuffer = new Uint8Array(arrayBuffer);

        emulator.system.load_rom(romBuffer);
        
        requestAnimationFrame((time) => mainloop(emulator, time));
    }, false);
}

function mainloop(emulator, time) {
    const dt = time - emulator.previousTime;
    emulator.deltaTime = emulator.deltaTime + dt;
    emulator.previousTime = time;

    console.log(`${dt}`);

    while (emulator.deltaTime > emulator.clock_speed) {
        emulator.system.cycle();
        emulator.system.tick_timers();
        emulator.deltaTime = emulator.deltaTime - emulator.clock_speed;
    }

    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);
    ctx.fillStyle = "green";
    emulator.system.draw(SCALE);

    requestAnimationFrame((time) => mainloop(emulator, time));
}

run().catch(console.error);