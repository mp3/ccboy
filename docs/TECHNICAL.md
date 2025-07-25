# Technical Documentation

## Architecture Overview

The Game Boy emulator is written in Rust and compiled to WebAssembly for browser execution. It accurately emulates the original DMG (Dot Matrix Game) hardware.

## Core Components

### CPU (Sharp LR35902)
- Modified Z80 processor running at 4.194304 MHz
- 8-bit data bus, 16-bit address bus
- 245 unique opcodes fully implemented
- Cycle-accurate timing

### Memory Map
```
0x0000-0x3FFF : ROM Bank 0 (16KB)
0x4000-0x7FFF : ROM Bank N (16KB, switchable)
0x8000-0x9FFF : Video RAM (8KB)
0xA000-0xBFFF : External RAM (8KB, cartridge)
0xC000-0xDFFF : Work RAM (8KB)
0xE000-0xFDFF : Echo RAM (mirror of 0xC000-0xDDFF)
0xFE00-0xFE9F : Sprite Attribute Table (OAM)
0xFEA0-0xFEFF : Prohibited area
0xFF00-0xFF7F : I/O Registers
0xFF80-0xFFFE : High RAM (127 bytes)
0xFFFF        : Interrupt Enable Register
```

### PPU (Picture Processing Unit)
- 160x144 pixel display
- 4 shades of gray (2-bit color)
- 20x18 tile background
- 40 sprites (8x8 or 8x16)
- 60 FPS (59.7275)

#### PPU Modes
1. **OAM Search** (80 cycles): Searches for sprites on current line
2. **Pixel Transfer** (172 cycles): Draws the scanline
3. **H-Blank** (204 cycles): Horizontal blanking period
4. **V-Blank** (4560 cycles): Vertical blanking period

### APU (Audio Processing Unit)
- 4 sound channels:
  - Channel 1: Square wave with sweep
  - Channel 2: Square wave with envelope
  - Channel 3: Programmable wave
  - Channel 4: Noise generator
- Stereo sound via NR51 register
- Master volume control

### Timer System
- DIV: Divider register (16384 Hz)
- TIMA: Timer counter
- TMA: Timer modulo
- TAC: Timer control
- Configurable frequencies: 4096, 262144, 65536, or 16384 Hz

### Interrupts
Five interrupt sources (in priority order):
1. **V-Blank** (0x40): End of frame
2. **LCD STAT** (0x48): LCD status changes
3. **Timer** (0x50): Timer overflow
4. **Serial** (0x58): Serial transfer (not implemented)
5. **Joypad** (0x60): Button press

## Cartridge Support

### MBC Types
- **ROM Only**: 32KB games without banking
- **MBC1**: Up to 2MB ROM, 32KB RAM
- **MBC3**: Up to 2MB ROM, 32KB RAM, RTC
- **MBC5**: Up to 8MB ROM, 128KB RAM

### Save System
Battery-backed RAM is automatically persisted to browser localStorage.

## Performance Optimizations

### WebAssembly Optimizations
- Compiled with `--release` flag
- wasm-opt level 3 optimizations
- No debug symbols in production

### Rendering Pipeline
1. PPU updates internal framebuffer during scanline rendering
2. Complete frame copied to canvas at V-Blank
3. Browser handles vsync and presentation

### Audio Pipeline
1. APU generates samples at 48000 Hz
2. Samples buffered in circular buffer
3. Web Audio API handles playback timing

## Accuracy Notes

### Timing Accuracy
- CPU instructions: Cycle-accurate
- PPU rendering: Scanline-accurate
- Interrupts: Cycle-accurate
- Timer: Cycle-accurate

### Known Differences from Hardware
1. **Audio**: Slight differences in noise channel
2. **PPU**: Sprite priority quirks in edge cases
3. **CPU**: Undefined opcodes behave differently

## Building from Source

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Build Commands
```bash
# Debug build
cargo build

# Release build
cargo build --release

# WebAssembly build
wasm-pack build --target web --out-dir pkg

# Run tests
cargo test

# Run with optimizations
./build-wasm.sh
```

## Testing

The emulator includes comprehensive test suites:
- Unit tests for each component
- Integration tests for full system behavior
- Test ROMs for validation

### Running Tests
```bash
# All tests
cargo test

# Specific test suite
cargo test --test cpu_tests
cargo test --test memory_tests
cargo test --test ppu_tests
cargo test --test integration_tests
```

## Debugging

### Debug Features
- CPU state inspection via `get_cpu_state()`
- Memory dump via `read_memory()`
- Disassembler for instruction analysis

### Browser Developer Tools
1. Open DevTools Console
2. The emulator logs errors and warnings
3. Use Performance tab for profiling

## Contributing

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use `cargo clippy` for linting
- Add tests for new features

### Architecture Guidelines
1. Keep components modular and independent
2. Prefer accuracy over performance
3. Document hardware quirks
4. Add tests for edge cases

## References

- [Pan Docs](https://gbdev.io/pandocs/): Comprehensive GB documentation
- [GBEDG](https://hacktix.github.io/GBEDG/): Emulator development guide
- [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf): Official CPU documentation
- [The Ultimate Game Boy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI): Hardware deep dive