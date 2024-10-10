# RustyChip: A CHIP-8 emulator in rust targeting WASM

Following along with the [ebook from @aquova](https://github.com/aquova/chip8-book) to learn Rust with WASM.

# Building:

1. Clone repository
2. Install wasm-pack: `cargo install wasm-pack`
3. Move into the `wasm` directory: `cd rustychip/wasm`
4. Run makefile: `make`
5. Host the public directory in a method of your choosing (`python3 -m http.server` works well for testing)
