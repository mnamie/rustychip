import init, * as wasm from "./wasm.js";

const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 15;
let CLOCK_SPEED = 500;

let anim_frame = 0;

const canvas = document.getElementById("canvas");
canvas.width = WIDTH * SCALE;
canvas.height = HEIGHT * SCALE;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

const input = document.getElementById("rom");

async function run() {
    await init();

    let emulator = new wasm.RustyChipWasm();

    document.addEventListener("keydown", function(evt) {
        emulator.keypress(evt, true);
    });

    document.addEventListener("keyup", function(evt) {
        emulator.keypress(evt, false);
    });

    document.getElementById("clock-rate").addEventListener("change", function(e) {
        console.log("Changing clock speed");
        CLOCK_SPEED = parseInt( e.target.value );
        console.log(`Clock speed is now: ${CLOCK_SPEED}`);
    });

    input.addEventListener("change", async function(evt) {
        if (anim_frame != 0) {
            window.cancelAnimationFrame(anim_frame);
        }

        let file_name = evt.target.value;
        const response = await fetch(`./assets/roms/${file_name}`);
        const arrayBuffer = await response.arrayBuffer();
        const romBuffer = new Uint8Array(arrayBuffer);

        emulator.load_rom(romBuffer);
        
        while (true) {
            requestAnimationFrame(mainloop(emulator));
        }
    }, false);
}

function mainloop(emulator) {
    emulator.cycle();
    emulator.tick_timers();

    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

    ctx.fillStyle = "green";
    emulator.draw(SCALE);
}

run().catch(console.error);