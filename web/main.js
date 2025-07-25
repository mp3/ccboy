import init, { Emulator } from '../pkg/ccboy.js';

let emulator = null;
let animationId = null;
let canvas = null;
let ctx = null;
let audioContext = null;
let audioBufferQueue = [];
let nextAudioStartTime = 0;
let audioWorkletNode = null;
let useAudioWorklet = false;

// UI State
let isPaused = false;
let speedMultiplier = 1;
let volumeLevel = 0.5;
let showFPS = false;
let lastFrameTime = performance.now();
let frameCount = 0;
let currentFPS = 0;
let currentRomName = null;
let saveState = null;

// Performance optimization
let frameSkip = 0;
let autoFrameSkip = true;
let targetFPS = 60;
let frameTimes = [];
let lastRenderTime = 0;

const KEYS = {
    39: 0, // Right Arrow
    37: 1, // Left Arrow
    38: 2, // Up Arrow
    40: 3, // Down Arrow
    90: 4, // Z (A button)
    88: 5, // X (B button)
    16: 6, // Shift (Select)
    13: 7  // Enter (Start)
};

async function initEmulator() {
    await init();
    
    canvas = document.getElementById('screen');
    ctx = canvas.getContext('2d');
    ctx.imageSmoothingEnabled = false;
    
    // Initialize UI elements
    initializeUI();
    
    // Set up ROM loading
    const romInput = document.getElementById('rom-input');
    const dropZone = document.getElementById('rom-drop-zone');
    
    dropZone.addEventListener('click', () => romInput.click());
    romInput.addEventListener('change', loadRom);
    
    // Drag and drop support
    dropZone.addEventListener('dragover', handleDragOver);
    dropZone.addEventListener('drop', handleDrop);
    dropZone.addEventListener('dragleave', handleDragLeave);
    
    // Keyboard controls
    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('keyup', handleKeyUp);
    
    // Mobile controls
    initializeMobileControls();
}

function initializeUI() {
    // Pause/Resume button
    const pauseBtn = document.getElementById('pause-btn');
    pauseBtn.addEventListener('click', togglePause);
    
    // Reset button
    const resetBtn = document.getElementById('reset-btn');
    resetBtn.addEventListener('click', resetEmulator);
    
    // Save/Load state buttons
    const saveStateBtn = document.getElementById('save-state-btn');
    const loadStateBtn = document.getElementById('load-state-btn');
    saveStateBtn.addEventListener('click', saveCurrentState);
    loadStateBtn.addEventListener('click', loadSavedState);
    
    // Speed control
    const speedControl = document.getElementById('speed-control');
    speedControl.addEventListener('change', (e) => {
        speedMultiplier = parseFloat(e.target.value);
    });
    
    // Volume control
    const volumeControl = document.getElementById('volume-control');
    const volumeValue = document.getElementById('volume-value');
    volumeControl.addEventListener('input', (e) => {
        volumeLevel = e.target.value / 100;
        volumeValue.textContent = `${e.target.value}%`;
        if (audioContext && audioContext.destination) {
            // Apply volume to audio context
        }
    });
    
    // Fullscreen button
    const fullscreenBtn = document.getElementById('fullscreen-btn');
    fullscreenBtn.addEventListener('click', toggleFullscreen);
    
    // Settings modal
    const settingsBtn = document.getElementById('settings-btn');
    const settingsModal = document.getElementById('settings-modal');
    const closeBtn = settingsModal.querySelector('.close-btn');
    
    settingsBtn.addEventListener('click', () => {
        settingsModal.classList.add('show');
    });
    
    closeBtn.addEventListener('click', () => {
        settingsModal.classList.remove('show');
    });
    
    settingsModal.addEventListener('click', (e) => {
        if (e.target === settingsModal) {
            settingsModal.classList.remove('show');
        }
    });
    
    // Settings options
    const pixelPerfect = document.getElementById('pixel-perfect');
    pixelPerfect.addEventListener('change', (e) => {
        canvas.style.imageRendering = e.target.checked ? 'pixelated' : 'auto';
    });
    
    const showFPSCheckbox = document.getElementById('show-fps');
    showFPSCheckbox.addEventListener('change', (e) => {
        showFPS = e.target.checked;
        document.getElementById('fps-counter').style.display = showFPS ? 'block' : 'none';
    });
    
    // Performance settings
    const autoFrameskipCheckbox = document.getElementById('auto-frameskip');
    autoFrameskipCheckbox.addEventListener('change', (e) => {
        autoFrameSkip = e.target.checked;
        if (!autoFrameSkip) {
            frameSkip = 0; // Reset frame skip when disabled
        }
    });
    
    const maxFrameskipSelect = document.getElementById('max-frameskip');
    maxFrameskipSelect.addEventListener('change', (e) => {
        const maxSkip = parseInt(e.target.value);
        frameSkip = Math.min(frameSkip, maxSkip);
    });
    
    const enableAudio = document.getElementById('enable-audio');
    enableAudio.addEventListener('change', (e) => {
        if (audioContext) {
            if (e.target.checked) {
                audioContext.resume();
            } else {
                audioContext.suspend();
            }
        }
    });
    
    // Save data management
    const exportSaveBtn = document.getElementById('export-save');
    const importSaveBtn = document.getElementById('import-save');
    const clearSaveBtn = document.getElementById('clear-save');
    
    exportSaveBtn.addEventListener('click', exportSaveData);
    importSaveBtn.addEventListener('click', importSaveData);
    clearSaveBtn.addEventListener('click', clearSaveData);
}

function initializeMobileControls() {
    const mobileButtons = document.querySelectorAll('.dpad-btn, .action-btn, .system-btn');
    
    mobileButtons.forEach(button => {
        const key = button.dataset.key;
        
        // Touch events
        button.addEventListener('touchstart', (e) => {
            e.preventDefault();
            simulateKeyPress(key, true);
        });
        
        button.addEventListener('touchend', (e) => {
            e.preventDefault();
            simulateKeyPress(key, false);
        });
        
        // Mouse events for testing
        button.addEventListener('mousedown', () => {
            simulateKeyPress(key, true);
        });
        
        button.addEventListener('mouseup', () => {
            simulateKeyPress(key, false);
        });
    });
}

function simulateKeyPress(key, isDown) {
    const keyCode = getKeyCode(key);
    if (keyCode && emulator) {
        if (isDown) {
            emulator.key_down(KEYS[keyCode]);
        } else {
            emulator.key_up(KEYS[keyCode]);
        }
    }
}

function getKeyCode(key) {
    const keyMap = {
        'ArrowUp': 38,
        'ArrowDown': 40,
        'ArrowLeft': 37,
        'ArrowRight': 39,
        'z': 90,
        'x': 88,
        'Enter': 13,
        'Shift': 16
    };
    return keyMap[key];
}

async function loadRom(event) {
    const file = event.target.files[0];
    if (!file) return;
    
    await loadRomFile(file);
}

async function loadRomFile(file) {
    const arrayBuffer = await file.arrayBuffer();
    const romData = new Uint8Array(arrayBuffer);
    
    if (emulator) {
        cancelAnimationFrame(animationId);
    }
    
    emulator = new Emulator();
    emulator.load_rom(romData);
    
    // Initialize audio context on user interaction
    if (!audioContext) {
        audioContext = new (window.AudioContext || window.webkitAudioContext)({
            sampleRate: 44100,
            latencyHint: 'interactive'  // Lower latency
        });
        
        // Try to use AudioWorklet for better performance
        initAudioWorklet();
    }
    
    // Update UI
    currentRomName = file.name;
    document.getElementById('rom-drop-zone').classList.add('loaded');
    enableControls();
    updateGameInfo(file, romData);
    
    // Load save data if exists
    loadSaveFromStorage();
    
    // Start emulation
    isPaused = false;
    runEmulator();
}

function enableControls() {
    document.querySelectorAll('.control-btn').forEach(btn => {
        btn.disabled = false;
    });
    document.getElementById('speed-control').disabled = false;
    document.getElementById('volume-control').disabled = false;
}

function updateGameInfo(file, romData) {
    document.getElementById('game-info').style.display = 'block';
    document.getElementById('game-title').textContent = file.name.replace('.gb', '');
    document.getElementById('game-size').textContent = `${(file.size / 1024).toFixed(1)} KB`;
    
    // Detect cartridge type
    const cartridgeType = romData[0x147];
    const cartridgeNames = {
        0x00: 'ROM Only',
        0x01: 'MBC1',
        0x02: 'MBC1+RAM',
        0x03: 'MBC1+RAM+Battery',
        0x05: 'MBC2',
        0x06: 'MBC2+Battery',
        0x08: 'ROM+RAM',
        0x09: 'ROM+RAM+Battery',
        0x0B: 'MMM01',
        0x0C: 'MMM01+RAM',
        0x0D: 'MMM01+RAM+Battery',
        0x0F: 'MBC3+Timer+Battery',
        0x10: 'MBC3+Timer+RAM+Battery',
        0x11: 'MBC3',
        0x12: 'MBC3+RAM',
        0x13: 'MBC3+RAM+Battery',
        0x19: 'MBC5',
        0x1A: 'MBC5+RAM',
        0x1B: 'MBC5+RAM+Battery',
        0x1C: 'MBC5+Rumble',
        0x1D: 'MBC5+Rumble+RAM',
        0x1E: 'MBC5+Rumble+RAM+Battery'
    };
    
    document.getElementById('game-cartridge').textContent = cartridgeNames[cartridgeType] || `Unknown (0x${cartridgeType.toString(16).toUpperCase()})`;
}

function runEmulator() {
    let skipFrames = 0;
    
    function frame(currentTime) {
        if (!isPaused) {
            // Calculate frame timing
            const deltaTime = currentTime - lastRenderTime;
            lastRenderTime = currentTime;
            
            // Auto frame skip calculation
            if (autoFrameSkip) {
                frameTimes.push(deltaTime);
                if (frameTimes.length > 30) {
                    frameTimes.shift();
                }
                
                const avgFrameTime = frameTimes.reduce((a, b) => a + b, 0) / frameTimes.length;
                const targetFrameTime = 1000 / targetFPS;
                
                if (avgFrameTime > targetFrameTime * 1.2) {
                    frameSkip = Math.min(3, frameSkip + 1);
                } else if (avgFrameTime < targetFrameTime * 0.9) {
                    frameSkip = Math.max(0, frameSkip - 1);
                }
            }
            
            // Run frames based on speed multiplier
            for (let i = 0; i < speedMultiplier; i++) {
                emulator.run_frame();
            }
            
            // Render screen (with frame skipping)
            if (skipFrames <= 0) {
                const screenData = emulator.get_screen_buffer();
                const imageData = ctx.createImageData(160, 144);
                
                // Optimized pixel copy
                const data = imageData.data;
                for (let i = 0; i < screenData.length; i += 4) {
                    data[i] = screenData[i];
                    data[i + 1] = screenData[i + 1];
                    data[i + 2] = screenData[i + 2];
                    data[i + 3] = screenData[i + 3];
                }
                
                ctx.putImageData(imageData, 0, 0);
                skipFrames = frameSkip;
            } else {
                skipFrames--;
            }
            
            // Process audio
            if (audioContext && speedMultiplier === 1) {
                const audioData = emulator.get_audio_buffer();
                if (audioData.length > 0) {
                    processAudio(audioData);
                }
            }
            
            // Update FPS counter
            if (showFPS) {
                updateFPS();
            }
        }
        
        animationId = requestAnimationFrame(frame);
    }
    
    animationId = requestAnimationFrame(frame);
}

async function initAudioWorklet() {
    try {
        await audioContext.audioWorklet.addModule('audio-processor.js');
        audioWorkletNode = new AudioWorkletNode(audioContext, 'gameboy-processor');
        audioWorkletNode.connect(audioContext.destination);
        useAudioWorklet = true;
    } catch (e) {
        console.log('AudioWorklet not supported, using fallback');
        useAudioWorklet = false;
    }
}

function processAudio(audioData) {
    if (useAudioWorklet && audioWorkletNode) {
        // Send audio data to worklet for processing
        audioWorkletNode.port.postMessage({
            type: 'audioData',
            data: audioData,
            volume: volumeLevel
        });
    } else {
        // Fallback to standard Web Audio API
        processAudioFallback(audioData);
    }
}

function processAudioFallback(audioData) {
    const frameCount = audioData.length / 2; // Stereo
    const audioBuffer = audioContext.createBuffer(2, frameCount, 44100);
    
    // Optimized stereo data split and volume application
    const leftChannel = audioBuffer.getChannelData(0);
    const rightChannel = audioBuffer.getChannelData(1);
    
    for (let i = 0; i < frameCount; i++) {
        const leftSample = audioData[i * 2] * volumeLevel;
        const rightSample = audioData[i * 2 + 1] * volumeLevel;
        leftChannel[i] = leftSample;
        rightChannel[i] = rightSample;
    }
    
    // Create and schedule buffer with optimized timing
    const source = audioContext.createBufferSource();
    source.buffer = audioBuffer;
    source.connect(audioContext.destination);
    
    const currentTime = audioContext.currentTime;
    const bufferDuration = audioBuffer.duration;
    
    // Better scheduling logic
    if (nextAudioStartTime < currentTime) {
        nextAudioStartTime = currentTime + 0.005; // 5ms buffer
    }
    
    source.start(nextAudioStartTime);
    nextAudioStartTime += bufferDuration;
    
    // Prevent audio drift
    const maxLookahead = 0.1; // 100ms
    if (nextAudioStartTime > currentTime + maxLookahead) {
        nextAudioStartTime = currentTime + maxLookahead / 2;
    }
}

function updateFPS() {
    frameCount++;
    const currentTime = performance.now();
    const deltaTime = currentTime - lastFrameTime;
    
    if (deltaTime >= 1000) {
        currentFPS = Math.round((frameCount * 1000) / deltaTime);
        const fpsElement = document.getElementById('fps-counter');
        
        // Enhanced performance display
        let perfInfo = `FPS: ${currentFPS}`;
        
        // Add frame skip info
        if (autoFrameSkip && frameSkip > 0) {
            perfInfo += ` | Skip: ${frameSkip}`;
        }
        
        // Add average frame time
        if (frameTimes.length > 0) {
            const avgFrameTime = frameTimes.reduce((a, b) => a + b, 0) / frameTimes.length;
            perfInfo += ` | ${avgFrameTime.toFixed(1)}ms`;
        }
        
        // Color code based on performance
        if (currentFPS >= 58) {
            fpsElement.style.color = '#0F0'; // Green
        } else if (currentFPS >= 50) {
            fpsElement.style.color = '#FF0'; // Yellow
        } else {
            fpsElement.style.color = '#F00'; // Red
        }
        
        fpsElement.textContent = perfInfo;
        frameCount = 0;
        lastFrameTime = currentTime;
    }
}

function togglePause() {
    isPaused = !isPaused;
    const pauseBtn = document.getElementById('pause-btn');
    pauseBtn.innerHTML = isPaused ? '<i class="fas fa-play"></i> Resume' : '<i class="fas fa-pause"></i> Pause';
}

function resetEmulator() {
    if (emulator && currentRomName) {
        const input = document.getElementById('rom-input');
        if (input.files.length > 0) {
            loadRomFile(input.files[0]);
        }
    }
}

function saveCurrentState() {
    if (emulator) {
        const state = emulator.get_save_state();
        if (state) {
            saveState = {
                timestamp: Date.now(),
                romName: currentRomName,
                state: state
            };
            localStorage.setItem(`gb_savestate_${currentRomName}`, JSON.stringify(saveState));
            alert('State saved!');
        } else {
            alert('Failed to save state');
        }
    }
}

function loadSavedState() {
    // First try to load from localStorage
    const savedData = localStorage.getItem(`gb_savestate_${currentRomName}`);
    if (savedData) {
        saveState = JSON.parse(savedData);
    }
    
    if (saveState && saveState.romName === currentRomName && saveState.state) {
        if (emulator.load_save_state(saveState.state)) {
            alert('State loaded!');
        } else {
            alert('Failed to load state');
        }
    } else {
        alert('No save state available for this game');
    }
}

function toggleFullscreen() {
    if (!document.fullscreenElement) {
        document.documentElement.requestFullscreen();
    } else {
        document.exitFullscreen();
    }
}

function handleDragOver(e) {
    e.preventDefault();
    e.stopPropagation();
    e.target.classList.add('dragover');
}

function handleDragLeave(e) {
    e.preventDefault();
    e.stopPropagation();
    e.target.classList.remove('dragover');
}

function handleDrop(e) {
    e.preventDefault();
    e.stopPropagation();
    e.target.classList.remove('dragover');
    
    const files = e.dataTransfer.files;
    if (files.length > 0) {
        const file = files[0];
        if (file.name.endsWith('.gb') || file.name.endsWith('.gbc')) {
            loadRomFile(file);
        } else {
            alert('Please drop a Game Boy ROM file (.gb)');
        }
    }
}

function handleKeyDown(event) {
    if (emulator && KEYS.hasOwnProperty(event.keyCode)) {
        event.preventDefault();
        emulator.key_down(KEYS[event.keyCode]);
    }
}

function handleKeyUp(event) {
    if (emulator && KEYS.hasOwnProperty(event.keyCode)) {
        event.preventDefault();
        emulator.key_up(KEYS[event.keyCode]);
    }
}

// Save data functions
function loadSaveFromStorage() {
    if (currentRomName) {
        const saveKey = `gb_save_${currentRomName}`;
        const saveDataStr = localStorage.getItem(saveKey);
        if (saveDataStr) {
            try {
                const saveData = JSON.parse(saveDataStr);
                const bytes = new Uint8Array(saveData);
                emulator.load_save_data(bytes);
                console.log('Save data loaded for', currentRomName);
            } catch (e) {
                console.error('Failed to load save data:', e);
            }
        }
    }
}

function saveSaveToStorage() {
    if (emulator && currentRomName) {
        const saveData = emulator.get_save_data();
        if (saveData && saveData.length > 0) {
            const saveKey = `gb_save_${currentRomName}`;
            // Convert Uint8Array to array for JSON storage
            const saveArray = Array.from(saveData);
            localStorage.setItem(saveKey, JSON.stringify(saveArray));
        }
    }
}

function exportSaveData() {
    if (emulator && currentRomName) {
        // First make sure current save data is saved
        saveSaveToStorage();
        
        const saveData = emulator.get_save_data();
        if (saveData && saveData.length > 0) {
            const blob = new Blob([saveData], { type: 'application/octet-stream' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `${currentRomName}.sav`;
            a.click();
            URL.revokeObjectURL(url);
        } else {
            alert('No save data found for this game');
        }
    }
}

function importSaveData() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.sav';
    input.onchange = async (e) => {
        const file = e.target.files[0];
        if (file && emulator) {
            const arrayBuffer = await file.arrayBuffer();
            const data = new Uint8Array(arrayBuffer);
            emulator.load_save_data(data);
            saveSaveToStorage(); // Save to localStorage
            alert('Save data imported successfully!');
        }
    };
    input.click();
}

function clearSaveData() {
    if (currentRomName && confirm('Are you sure you want to clear all save data for this game?')) {
        const saveKey = `gb_save_${currentRomName}`;
        localStorage.removeItem(saveKey);
        alert('Save data cleared');
    }
}

// Auto-save periodically
setInterval(() => {
    if (emulator && !isPaused) {
        saveSaveToStorage();
    }
}, 10000); // Save every 10 seconds

initEmulator();