# Troubleshooting Guide

## Common Issues and Solutions

### Game Won't Start

#### Black Screen
**Problem**: The screen remains black after loading a ROM.

**Solutions**:
1. Wait a few seconds - some games have slow intro sequences
2. Try pressing Start (Enter) or A (Z) to skip intros
3. Ensure the ROM file is not corrupted
4. Try a different game to verify the emulator works

#### "Invalid ROM" Error
**Problem**: The emulator rejects the ROM file.

**Solutions**:
1. Verify the file has a `.gb` extension
2. Ensure it's an original Game Boy ROM (not GBC or GBA)
3. Check the file isn't corrupted (compare checksums)
4. Try downloading the ROM from a different source

### Audio Issues

#### No Sound
**Problem**: Game runs but no audio plays.

**Solutions**:
1. Click anywhere on the page (browsers require user interaction for audio)
2. Check system volume and unmute
3. Ensure browser tab isn't muted
4. Try a different browser
5. Check if other web audio works

#### Crackling or Distorted Audio
**Problem**: Audio plays but sounds bad.

**Solutions**:
1. Close other browser tabs to free resources
2. Disable browser extensions
3. Try Chrome or Firefox (better Web Audio support)
4. Lower system audio quality to 48kHz

#### Audio Delay
**Problem**: Sound effects lag behind the action.

**Solutions**:
1. Refresh the page
2. Use a different browser
3. Close other applications
4. Enable hardware acceleration in browser settings

### Performance Problems

#### Slow Gameplay
**Problem**: Game runs slower than normal.

**Solutions**:
1. Close unnecessary browser tabs
2. Disable browser extensions (especially ad blockers)
3. Use Chrome or Firefox for best performance
4. Ensure hardware acceleration is enabled
5. Try incognito/private mode

#### Stuttering or Frame Drops
**Problem**: Game stutters or skips frames.

**Solutions**:
1. Plug in laptop (battery saving modes reduce performance)
2. Close other applications
3. Disable browser dev tools
4. Lower screen resolution
5. Disable V-Sync in graphics settings

### Control Issues

#### Keys Not Responding
**Problem**: Keyboard inputs don't work.

**Solutions**:
1. Click on the game canvas to give it focus
2. Check Caps Lock is off
3. Try alternative control scheme (WASD instead of arrows)
4. Disable browser extensions that capture keys
5. Test in a different browser

#### Stuck Keys
**Problem**: Character keeps moving without input.

**Solutions**:
1. Press and release the stuck key
2. Click outside and back on the game
3. Refresh the page

### Save Game Issues

#### Game Won't Save
**Problem**: Progress is lost when reloading.

**Solutions**:
1. Ensure the game supports battery saves (check compatibility list)
2. Don't use incognito/private mode
3. Allow cookies and local storage
4. Wait a few seconds after saving before closing
5. Check browser storage isn't full

#### Corrupted Save
**Problem**: Save file causes game to crash or reset.

**Solutions**:
1. Clear browser storage for the site
2. Delete localStorage entry for the game
3. Start a new save file

### Browser-Specific Issues

#### Chrome
- Enable hardware acceleration in settings
- Disable "Lite mode" if enabled
- Update to latest version

#### Firefox
- Disable tracking protection for the site
- Enable WebAssembly in about:config
- Clear cache if performance degrades

#### Safari
- Enable WebAssembly in Developer menu
- Allow auto-play for the site
- Update to Safari 13 or newer

#### Edge
- Use Chromium-based Edge (version 80+)
- Disable SmartScreen for local files
- Enable developer mode if needed

### Advanced Troubleshooting

#### Check Browser Console
1. Press F12 to open Developer Tools
2. Go to Console tab
3. Look for red error messages
4. Report errors with screenshots

#### Memory Issues
**Symptoms**: Browser tab crashes or "Aw, Snap!" errors

**Solutions**:
1. Close other tabs and applications
2. Increase browser memory limit
3. Disable extensions
4. Use 64-bit browser version

#### WebAssembly Errors
**Symptoms**: "WebAssembly not supported" or similar

**Solutions**:
1. Update browser to latest version
2. Enable WebAssembly in browser flags
3. Try a different browser
4. Check if antivirus is blocking WASM

## Getting Help

If problems persist:

1. **Check Compatibility List**: Ensure your game is supported
2. **Try Test ROM**: Load Tetris to verify basic functionality
3. **Browser Console**: Check for error messages (F12)
4. **Different Browser**: Try Chrome, Firefox, or Edge
5. **Report Issue**: Include:
   - Browser name and version
   - Operating system
   - Game name
   - Error messages
   - Steps to reproduce

## Quick Fixes Checklist

- [ ] Refresh the page
- [ ] Click on game canvas
- [ ] Check audio is unmuted
- [ ] Close other tabs
- [ ] Try different browser
- [ ] Clear browser cache
- [ ] Disable extensions
- [ ] Update browser
- [ ] Allow pop-ups and storage
- [ ] Check game compatibility