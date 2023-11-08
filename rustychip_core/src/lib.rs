use rand::random;

const RAM_SIZE: usize = 4096;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

pub struct RustyChip {
    pc: u16,
    memory: [u8; RAM_SIZE],
    display: [bool; SCREEN_SIZE],
    v: [u8; 16],
    i: u16,
    sp: u16,
    stack: [u16; 16],
    keys: [bool; 16],
    dt: u8,
    st: u8,
}

impl RustyChip {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: 0x200,
            memory: [0; RAM_SIZE],
            display: [false; SCREEN_SIZE],
            v: [0; 16],
            i: 0,
            sp: 0,
            stack: [0; 16],
            keys: [false; 16],
            dt: 0,
            st: 0,
        };

        new_emu.memory[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }

    // Stack functions
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    // Reset emulator
    pub fn reset(&mut self) {
        self.pc = 0x200;
        self.memory = [0; RAM_SIZE];
        self.display = [false; SCREEN_SIZE];
        self.v = [0; 16];
        self.i = 0;
        self.sp = 0;
        self.stack = [0; 16];
        self.keys = [false; 16];
        self.dt = 0;
        self.st = 0;
        self.memory[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    // Run a CPU cycle
    pub fn cycle(&mut self) {
        let op = self.fetch_opcode();
        self.interpret(op);
    }

    pub fn frontend_display(&self) -> &[bool] {
        &self.display
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let start = 0x200 as usize;
        let end = (0x200 as usize) + data.len();
        self.memory[start..end].copy_from_slice(data);
    }

    // Fetch the opcode for the current program counter value
    fn fetch_opcode(&mut self) -> u16 {
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;
        let op = (high_byte << 8) | low_byte;
        self.pc += 2;
        op
    }

    fn interpret(&mut self, op: u16) {
        let hex1 = (op & 0xF000) >> 12;
        let hex2 = (op & 0x0F00) >> 8;
        let hex3 = (op & 0x00f0) >> 4;
        let hex4 = op & 0x000F;

        match (hex1, hex2, hex3, hex4) {
            // 0000: NOP
            (0, 0, 0, 0) => return,

            // 00E0: CLS (clear screen)
            (0, 0, 0xE, 0) => {
                self.display = [false; SCREEN_SIZE];
            },

            // 00EE: RET (return)
            (0, 0, 0xE, 0xE) => {
                let return_address = self.pop();
                self.pc = return_address;
            },

            // 1NNN: JMP
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            },

            // 2NNN: CALL
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.pc);
                self.pc = nnn;
            },

            // 3XNN: SKP if VX == NN
            (3, _, _, _) => {
                let x = hex2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v[x] == nn {
                    self.pc += 2;
                }
            },

            // 4XNN: SKP if VX != NN
            (4, _, _, _) => {
                let x = hex2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v[x] != nn {
                    self.pc += 2;
                }
            },

            // 5XY0: SKP if VX == VY
            (5, _, _, 0) => {
                let x = hex2 as usize;
                let y = hex3 as usize;
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            },

            // 6XNN: VX = N
            (6, _, _, _) => {
                let x = hex2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v[x] = nn;
            },

            // 7XNN: VX += NN
            (7, _, _, _) => {
                let x = hex2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v[x] = self.v[x].wrapping_add(nn);
            },

            // 8XY0: VX = VY
            (8, _, _, 0) => {
                let x = hex2 as usize;
                let y = hex3 as usize;
                self.v[x] = self.v[y];
            },

            // 8XY1: Bitwise OR
            (8, _, _, 1) => {
                let x = hex2 as usize;
                let y = hex3 as usize;
                self.v[x] |= self.v[y];
            },

            // 8XY2: Bitwise AND
            (8, _, _, 2) => {
                let x = hex2 as usize;
                let y = hex3 as usize;
                self.v[x] &= self.v[y];
            },

            // 8XY3: Bitwise XOR
            (8, _, _, 3) => {
                let x = hex2 as usize;
                let y = hex3 as usize;
                self.v[x] ^= self.v[y];
            },

            // 8XY4: ADD w/ carry over flag
            (8, _, _, 4) => {
                let x = hex2 as usize;
                let y = hex3 as usize;
                let (new_val, carry) = self.v[x].overflowing_add(self.v[y]);
                let carry_flag = if carry {1} else {0};
                
                self.v[x] = new_val;
                self.v[0xF] = carry_flag;
            },

            // 8XY5: MINUS w/ underflow flag
            (8, _, _, 5) => {
                let x = hex2 as usize;
                let y = hex3 as usize;
                let (new_val, under) = self.v[x].overflowing_sub(self.v[y]);
                let under_flag = if under {0} else {1};
                
                self.v[x] = new_val;
                self.v[0xF] = under_flag;
            },

            // 8XY6: VX >>= 1
            (8, _, _, 6) => {
                let x = hex2 as usize;
                let lsb = self.v[x] & 1;
                self.v[x] >>= 1;
                self.v[0xF] = lsb;
            },

            // 8XY7: VX = VY - VX
            (8, _, _, 7) => {
                let x = hex2 as usize;
                let y = hex3 as usize;

                let (new_val, under) = self.v[y].overflowing_sub(self.v[x]);
                let under_flag = if under {0} else {1};

                self.v[x] = new_val;
                self.v[0xF] = under_flag;
            },

            // 8XYE: VX <<= 1
            (8, _, _, 0xE) => {
                let x = hex2 as usize;
                let msb = (self.v[x] >> 7) & 1;
                self.v[x] <<= 1;
                self.v[0xF] = msb;
            },

            // 9XY0: SKP if VX != VY
            (9, _, _, 0) => {
                let x = hex2 as usize;
                let y = hex3 as usize;
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            },

            // ANNN: I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i = nnn;
            },

            // BNNN: JMP to V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc += (self.v[0] as u16) + nnn;
            },

            // CXNN: VX = rand() & NN
            (0xC, _, _, _) => {
                let x = hex2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();
                self.v[x] = rng & nn;
            },

            // DXYN: DRAW
            (0xD, _, _, _) => {
                // Fetch (x, y) for sprite
                let vec2_x = self.v[hex2 as usize] as u16;
                let vec2_y = self.v[hex3 as usize] as u16;
                let height = hex4; // Height of sprite

                let mut flipped: bool = false;
                for y_line in 0..height {
                    let addr = self.i + y_line as u16;
                    let pixels = self.memory[addr as usize];

                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (vec2_x + x_line) as usize % SCREEN_WIDTH;
                            let y = (vec2_y + y_line) as usize % SCREEN_HEIGHT;
                            let idx = x + SCREEN_WIDTH * y;
                            flipped |= self.display[idx];
                            self.display[idx] ^= true;
                        }
                    }
                }

                if flipped {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
            },

            // EX9E: SKIP if key is pressed
            (0xE, _, 9, 0xE) => {
                let x = hex2 as usize;
                let vx = self.v[x];
                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
            },

            // EXA1: SKIP if key not pressed
            (0xE, _, 0xA, 1) => {
                let x = hex2 as usize;
                let vx = self.v[x];
                let key = self.keys[vx as usize];
                if !key {
                    self.pc += 2;
                }
            },

            // FX07: VX = DT
            (0xF, _, 0, 7) => {
                let x = hex2 as usize;
                self.v[x] = self.dt;
            },

            // FX0A: Pause until key press
            (0xF, _, 0, 0xA) => {
                let x = hex2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            },

            // FX15: DT = VX
            (0xF, _, 1, 5) => {
                let x = hex2 as usize;
                self.dt = self.v[x];
            },

            // FX18: ST = VX
            (0xF, _, 1, 8) => {
                let x = hex2 as usize;
                self.st = self.v[x];
            },

            // FX1E: I += VX
            (0xF, _, 1, 0xE) => {
                let x = hex2 as usize;
                let vx = self.v[x] as u16;
                self.i = self.i.wrapping_add(vx);
            },

            // FX29: I = font address
            (0xF, _, 2, 9) => {
                let x = hex2 as usize;
                let c = self.v[x] as u16;
                self.i = c * 5;
            },

            // FX33: I = BCD of VX
            (0xF, _, 3, 3) => {
                let x = hex2 as usize;
                let vx = self.v[x] as f32;

                let hundreds = (vx / 100.0).floor() as u8;
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10.0) as u8;

                self.memory[self.i as usize] = hundreds;
                self.memory[(self.i + 1) as usize] = tens;
                self.memory[(self.i + 2) as usize] = ones;
            },

            // FX55: I = V0 - VX
            (0xF, _, 5, 5) => {
                let x = hex2 as usize;
                let i = self.i as usize;
                for idx in 0..=x {
                    self.memory[i + idx] = self.v[idx];
                }
            },

            // FX65: V0 - VX = I
            (0xF, _, 6, 5) => {
                let x = hex2 as usize;
                let i = self.i as usize;
                for idx in 0..=x {
                    self.v[idx] = self.memory[i + idx];
                }
            },

            // Fallback for unimplemented
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }

    // Cycle display and sound timers
    pub fn cycle_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // BEEP
            }
            self.st -= 1;
        }
    }
}