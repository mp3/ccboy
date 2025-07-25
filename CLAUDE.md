# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build WebAssembly module
./build-wasm.sh
# or manually:
wasm-pack build --target web --out-dir pkg

# Run development server (auto-opens browser)
./run-server.py
# or manually:
python3 -m http.server 8000

# Run all tests
cargo test

# Run specific test suite
cargo test cpu_tests
cargo test memory_tests
cargo test ppu_tests
cargo test integration_tests

# Run a single test
cargo test test_ld_instructions -- --exact

# Build native version (not typically used)
cargo build --release

# Lint and typecheck (if requested by user)
cargo clippy
cargo fmt -- --check
```

## Architecture Overview

This is a Game Boy (DMG) emulator written in Rust, compiled to WebAssembly for web browsers.

### Core Emulation Loop
- `src/gameboy.rs`: Main emulation coordinator that orchestrates all components
- Runs at 4.194304 MHz (Game Boy clock speed)
- Each frame is 70,224 cycles (59.7 FPS)

### Major Components

1. **CPU (`src/cpu/`)**: Sharp LR35902 processor
   - `mod.rs`: Main CPU struct with fetch-decode-execute loop
   - `registers.rs`: CPU registers (A, F, B, C, D, E, H, L, SP, PC)
   - `instructions.rs`: Instruction helpers (fetch_byte, fetch_word, push/pop)
   - `opcodes.rs`: Main instruction implementations
   - `opcodes_cb.rs`: CB-prefixed instructions (bit operations)
   - All 245 opcodes implemented with cycle-accurate timing

2. **Memory (`src/memory/`)**: Memory Management Unit
   - `mmu.rs`: Main memory mapper (0x0000-0xFFFF address space)
   - `cartridge.rs`: ROM/RAM banking (MBC1, MBC3, MBC5)
   - Handles boot ROM overlay at 0x0000-0x00FF
   - Memory regions: ROM, VRAM, WRAM, OAM, I/O, HRAM

3. **PPU (`src/ppu/`)**: Graphics processor
   - `mod.rs`: State machine (OAM Scan → Drawing → HBlank → VBlank)
   - `tile_renderer.rs`: Background/window tile rendering
   - `sprite_renderer.rs`: Sprite (OBJ) rendering with priority
   - Outputs 160x144 RGBA framebuffer

4. **APU (`src/apu/`)**: Audio processor
   - `mod.rs`: Main audio mixer outputting 44.1kHz stereo
   - `square_channel.rs`: Channels 1 & 2 (square waves with sweep/envelope)
   - `wave_channel.rs`: Channel 3 (custom waveform)
   - `noise_channel.rs`: Channel 4 (LFSR noise)

5. **Supporting Components**:
   - `timer.rs`: DIV, TIMA, TMA, TAC registers
   - `joypad.rs`: 8-button input handling
   - `boot_rom.rs`: DMG boot ROM data
   - `save_state.rs`: Save state serialization/deserialization

### Web Interface (`web/`)
- `index.html`: UI with canvas, controls, settings modal
- `main.js`: WebAssembly loader, emulation loop, input handling
- `style.css`: Game Boy-inspired theme
- `audio-processor.js`: AudioWorklet for low-latency audio

### Key Design Patterns

1. **Memory-Mapped I/O**: All hardware registers are accessed through memory addresses
2. **Interrupt System**: 5 interrupt types (VBlank, LCDC, Timer, Serial, Joypad)
3. **Cycle Accuracy**: Each instruction takes exact number of cycles
4. **State Machines**: PPU modes, audio frame sequencer

### Performance Optimizations
- Palette cache in PPU to avoid recalculating colors
- Auto frame-skipping for slow devices
- AudioWorklet for reduced audio latency
- Optimized WASM build settings in `Cargo.toml`

### Save Data
- Save states: Complete emulator state serialized to JSON
- Battery saves: Cartridge RAM persisted to localStorage
- Both stored with ROM name as key

### Testing Strategy
- Unit tests for each component (CPU, Memory, PPU, etc.)
- Integration tests for full system behavior
- Test ROMs verify instruction correctness
- 100% test coverage (58/58 tests passing)