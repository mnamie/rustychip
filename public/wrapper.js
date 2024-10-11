"use strict";

import * as wasm from "./wasm.js";

const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 15;

export default class Chip8Wrapper {
    constructor() {
        this.anim_frame = 0;
        this.system = new wasm.RustyChipWasm();
        this.clock_speed = 1.0 / 100.0 * 1000;
        this.previousTime = 0.0;
        this.deltaTime = 0.0;

        
        this.canvas = document.getElementById("canvas");
        this.canvas.width = WIDTH * SCALE;
        this.canvas.height = HEIGHT * SCALE;

        this.ctx = canvas.getContext("2d");
        this.ctx.fillStyle = "black";
        this.ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);
    }

    mainloop(time) {
        const dt = time - this.previousTime;
        this.deltaTime = this.deltaTime + dt;
        this.previousTime = time;
        
        while (this.deltaTime > this.clock_speed) {
            this.system.cycle();
            this.system.tick_timers();
            this.deltaTime = this.deltaTime - this.clock_speed;
        }
    
        this.ctx.fillStyle = "black";
        this.ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);
        this.ctx.fillStyle = "green";
        this.system.draw(SCALE);
    
        requestAnimationFrame((time) => this.mainloop(time));
    }
}
