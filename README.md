# Game Boy Emulator

A fully-featured Nintendo Game Boy emulator written in Rust with WebAssembly support.

## Features

- **Complete CPU emulation** - All 245 opcodes of the Sharp LR35902 processor
- **Accurate PPU rendering** - Tile-based graphics with sprite support
- **4-channel audio** - Square waves, wave pattern, and noise channels
- **Memory Bank Controllers** - MBC1, MBC3, and MBC5 support
- **Boot ROM** - Included DMG boot ROM for authentic startup
- **WebAssembly support** - Runs directly in modern web browsers
- **Debug interface** - CPU state inspection and memory access

## Project Status

The emulator is feature-complete and can run most Game Boy games!

### Completed ✅
- [x] Full CPU implementation (LR35902 instruction set)
- [x] Memory management with banking support
- [x] PPU with background, window, and sprite layers
- [x] APU with all 4 sound channels
- [x] Timer and interrupt system
- [x] Joypad input handling
- [x] Boot ROM support
- [x] MBC1/3/5 cartridge support
- [x] WebAssembly build with web interface
- [x] Debug features for development

### Future Enhancements
- [ ] Game Boy Color support
- [ ] Save states
- [ ] Rewind functionality
- [ ] More MBC types
- [ ] Link cable emulation

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

- `cpu/`: LR35902 CPU emulation with all opcodes
- `memory/`: Memory management and cartridge handling
- `ppu/`: Graphics rendering with tile and sprite support
- `apu/`: Audio processing with 4 channels
- `timer.rs`: Timer and divider registers
- `joypad.rs`: Input handling
- `gameboy.rs`: Main emulation loop coordination
- `boot_rom.rs`: DMG boot ROM data
- `debug.rs`: Debugging utilities

## Documentation

Comprehensive documentation is available in the `docs/` directory:

- [User Guide](docs/USER_GUIDE.md) - How to use the emulator
- [Game Compatibility](docs/COMPATIBILITY.md) - List of tested games
- [Troubleshooting](docs/TROUBLESHOOTING.md) - Common issues and solutions
- [Technical Documentation](docs/TECHNICAL.md) - Architecture and implementation details

## License

This project is for educational purposes.