var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import init from "./wasm.js";
import Chip8Wrapper from "./wrapper.js";
function run() {
    return __awaiter(this, void 0, void 0, function* () {
        yield init();
        let emulator = new Chip8Wrapper();
        document.addEventListener("keydown", function (evt) {
            return __awaiter(this, void 0, void 0, function* () {
                emulator.system.keypress(evt, true);
            });
        });
        document.addEventListener("keyup", function (evt) {
            return __awaiter(this, void 0, void 0, function* () {
                emulator.system.keypress(evt, false);
            });
        });
        document.getElementById("clock-rate").addEventListener("change", function (evt) {
            return __awaiter(this, void 0, void 0, function* () {
                const target = evt.target;
                emulator.clockSpeed = 1.0 / parseFloat(target.value) * 1000.0;
            });
        });
        document.getElementById("rom").addEventListener("change", function (evt) {
            return __awaiter(this, void 0, void 0, function* () {
                if (emulator.anim_frame != 0) {
                    window.cancelAnimationFrame(emulator.anim_frame);
                }
                const target = evt.target;
                const file_name = target.value;
                const response = yield fetch(`./assets/roms/${file_name}`);
                const arrayBuffer = yield response.arrayBuffer();
                const romBuffer = new Uint8Array(arrayBuffer);
                emulator.system.load_rom(romBuffer);
                requestAnimationFrame((time) => emulator.mainloop(time));
            });
        });
    });
}
run().catch(console.error);
