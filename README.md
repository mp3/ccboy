# Game Boy Emulator

A Nintendo Game Boy emulator written in Rust with WebAssembly support.

## Features

- Game Boy CPU emulation (Sharp LR35902)
- Memory management with cartridge support
- PPU (Picture Processing Unit) for graphics rendering
- Timer and interrupt system
- Joypad input handling
- WebAssembly support for running in web browsers

## Project Status

This emulator is currently in early development. Basic structure is in place, but many opcodes and features are not yet implemented.

### Completed
- [x] Project structure with WebAssembly support
- [x] Basic CPU framework
- [x] Memory management system
- [x] Basic PPU structure
- [x] Timer implementation
- [x] Joypad input framework
- [x] Web interface

### TODO
- [ ] Complete CPU instruction set (245 opcodes)
- [ ] Full PPU implementation with sprite support
- [ ] Audio Processing Unit (APU)
- [ ] Memory Bank Controllers (MBC1, MBC3, MBC5)
- [ ] Save states
- [ ] Debugging features

## Building

### Prerequisites

- Rust (latest stable version)
- wasm-pack (for WebAssembly builds)

### Native Build

```bash
cargo build --release
```

### WebAssembly Build

```bash
./build-wasm.sh
```

Or manually:

```bash
wasm-pack build --target web --out-dir pkg
```

## Quick Start

1. Build the WebAssembly module:
   ```bash
   ./build-wasm.sh
   ```

2. Start the web server:
   ```bash
   ./run-server.py
   ```
   
   Or manually:
   ```bash
   python3 -m http.server 8000
   ```

3. The browser will automatically open to `http://localhost:8000/web/`

4. Load a Game Boy ROM file (.gb) using the "Load ROM" button

## Controls

- Arrow Keys: D-Pad
- Z: A Button
- X: B Button
- Enter: Start
- Shift: Select

## Architecture

The emulator is structured as follows:

- `cpu/`: LR35902 CPU emulation
- `memory/`: Memory management and cartridge handling
- `ppu/`: Graphics rendering
- `timer.rs`: Timer and divider registers
- `joypad.rs`: Input handling
- `gameboy.rs`: Main emulation loop coordination

## License

This project is for educational purposes.