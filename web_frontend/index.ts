import init from "./wasm.js";
import Chip8Wrapper from "./wrapper.js";

async function run() {
    await init();

    const emulator = new Chip8Wrapper();

    document.addEventListener("keydown", function (evt: KeyboardEvent) {
        emulator.system.keypress(evt, true);
    });

    document.addEventListener("keyup", function (evt: KeyboardEvent) {
        emulator.system.keypress(evt, false);
    });

    document
        .getElementById("clock-rate")!
        .addEventListener("input", function (evt: Event) {
            const target = evt.target as HTMLInputElement;
            const hz = parseFloat(target.value);
            emulator.clockSpeed = (1.0 / hz) * 1000.0;
            document.getElementById("clock-display")!.textContent = `${hz} HZ`;
        });

    async function loadRom(romName: string) {
        if (emulator.anim_frame !== 0) {
            window.cancelAnimationFrame(emulator.anim_frame);
        }
        const response = await fetch(`./assets/roms/${romName}`);
        const romBuffer = new Uint8Array(await response.arrayBuffer());
        emulator.system.reset();
        emulator.system.load_rom(romBuffer);
        requestAnimationFrame((time) => emulator.mainloop(time));
    }

    document
        .getElementById("rom-grid")!
        .addEventListener("click", async function (evt: Event) {
            const target = evt.target as HTMLElement;
            const rom = target.dataset.rom;
            if (!rom) return;
            document
                .querySelectorAll(".rom-btn")
                .forEach((btn) => btn.classList.remove("active"));
            target.classList.add("active");
            await loadRom(rom);
        });
}

run().catch(console.error);
