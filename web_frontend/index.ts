import init from "./wasm.js";
import Chip8Wrapper from "./wrapper.js";


async function run() {
    await init();

    let emulator = new Chip8Wrapper();

    document.addEventListener("keydown",  async function(evt: KeyboardEvent) {
        emulator.system.keypress(evt, true);
    });

    document.addEventListener("keyup",  async function(evt: KeyboardEvent) {
        emulator.system.keypress(evt, false);
    });

    document.getElementById("clock-rate")!.addEventListener("change",  async function(evt: Event) {
        const target = evt.target as HTMLTextAreaElement
        emulator.clockSpeed = 1.0 / parseFloat( target.value ) * 1000.0;
    });

    document.getElementById("rom")!.addEventListener("change", async function(evt: Event) {
        if (emulator.anim_frame != 0) {
            window.cancelAnimationFrame(emulator.anim_frame);
        }

        const target = evt.target as HTMLTextAreaElement
        const file_name = target.value;
        const response = await fetch(`./assets/roms/${file_name}`);
        const arrayBuffer = await response.arrayBuffer();
        const romBuffer = new Uint8Array(arrayBuffer);

        emulator.system.load_rom(romBuffer);
        
        requestAnimationFrame((time) => emulator.mainloop(time));
    });
}

run().catch(console.error);