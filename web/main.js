import init, { Emulator } from '../pkg/ccboy.js';

let emulator = null;
let animationId = null;
let canvas = null;
let ctx = null;
let audioContext = null;
let audioBufferQueue = [];
let nextAudioStartTime = 0;

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
    
    document.getElementById('load-rom').addEventListener('click', () => {
        document.getElementById('rom-input').click();
    });
    
    document.getElementById('rom-input').addEventListener('change', loadRom);
    
    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('keyup', handleKeyUp);
}

async function loadRom(event) {
    const file = event.target.files[0];
    if (!file) return;
    
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
            sampleRate: 44100
        });
    }
    
    runEmulator();
}

function runEmulator() {
    function frame() {
        emulator.run_frame();
        
        // Render screen
        const screenData = emulator.get_screen_buffer();
        const imageData = ctx.createImageData(160, 144);
        
        for (let i = 0; i < screenData.length; i++) {
            imageData.data[i] = screenData[i];
        }
        
        ctx.putImageData(imageData, 0, 0);
        
        // Process audio
        if (audioContext) {
            const audioData = emulator.get_audio_buffer();
            if (audioData.length > 0) {
                processAudio(audioData);
            }
        }
        
        animationId = requestAnimationFrame(frame);
    }
    
    frame();
}

function processAudio(audioData) {
    const frameCount = audioData.length / 2; // Stereo
    const audioBuffer = audioContext.createBuffer(2, frameCount, 44100);
    
    // Split stereo data
    for (let i = 0; i < frameCount; i++) {
        audioBuffer.getChannelData(0)[i] = audioData[i * 2];     // Left
        audioBuffer.getChannelData(1)[i] = audioData[i * 2 + 1]; // Right
    }
    
    // Create and schedule buffer
    const source = audioContext.createBufferSource();
    source.buffer = audioBuffer;
    source.connect(audioContext.destination);
    
    const currentTime = audioContext.currentTime;
    const startTime = Math.max(currentTime, nextAudioStartTime);
    source.start(startTime);
    
    // Update next start time
    nextAudioStartTime = startTime + audioBuffer.duration;
    
    // Clean up old buffers to prevent memory buildup
    if (nextAudioStartTime > currentTime + 1.0) {
        nextAudioStartTime = currentTime + 0.1;
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

initEmulator();