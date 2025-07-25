# Game Boy Emulator Documentation

Welcome to the Game Boy Emulator documentation! This emulator is a full-featured Nintendo Game Boy (DMG) emulator written in Rust and compiled to WebAssembly for web browsers.

## 📚 Documentation

### For Users
- **[User Guide](USER_GUIDE.md)** - How to use the emulator
- **[Compatibility List](COMPATIBILITY.md)** - Which games work
- **[Troubleshooting](TROUBLESHOOTING.md)** - Fixing common issues

### For Developers
- **[Technical Documentation](TECHNICAL.md)** - Architecture and implementation details
- **[Development Guide](../DEVELOPMENT.md)** - Building and contributing

## 🚀 Quick Start

1. Open `web/index.html` in a modern web browser
2. Click "Load ROM" and select a Game Boy ROM file (.gb)
3. Play using keyboard controls:
   - Arrow keys for D-Pad
   - Z for A button
   - X for B button
   - Enter for Start
   - Shift for Select

## ✨ Features

- **Complete Hardware Emulation**
  - All CPU instructions implemented
  - Accurate PPU (graphics) rendering
  - 4-channel audio with Web Audio API
  - Timer and interrupt system

- **Game Support**
  - Original Game Boy (DMG) games
  - MBC1, MBC3, and MBC5 cartridges
  - Battery-backed save games

- **Modern Web Interface**
  - Drag-and-drop ROM loading
  - Responsive canvas rendering
  - Local storage for save games

## 🎮 Popular Compatible Games

- Tetris
- Pokemon Red/Blue/Yellow
- Super Mario Land 1 & 2
- The Legend of Zelda: Link's Awakening
- Kirby's Dream Land
- Metroid II
- And many more!

## 🛠️ Technical Specifications

- **CPU**: Sharp LR35902 @ 4.194304 MHz
- **Resolution**: 160x144 pixels
- **Colors**: 4 shades of gray
- **Sound**: 4 channels (2 square, 1 wave, 1 noise)
- **Accuracy**: Cycle-accurate CPU, scanline-based PPU

## 📋 System Requirements

- Modern web browser (Chrome 80+, Firefox 75+, Safari 13+, Edge 80+)
- WebAssembly support
- At least 4GB RAM recommended
- Keyboard for controls

## 🤝 Contributing

This is an open-source project. Contributions are welcome! Please see the [Development Guide](../DEVELOPMENT.md) for details on:
- Building from source
- Running tests
- Architecture overview
- Coding standards

## 📄 License

This emulator is for educational purposes. Please only play games you legally own.

## 🔗 Resources

- [GitHub Repository](https://github.com/yourusername/gameboy-emulator)
- [Pan Docs](https://gbdev.io/pandocs/) - Game Boy technical reference
- [gbdev Community](https://gbdev.io/) - Game Boy development resources

---

Happy gaming! 🎮