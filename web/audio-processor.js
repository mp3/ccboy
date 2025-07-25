class GameBoyProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.audioBuffer = [];
        this.volume = 0.5;
        
        this.port.onmessage = (event) => {
            if (event.data.type === 'audioData') {
                const data = event.data.data;
                const volume = event.data.volume;
                this.volume = volume;
                
                // Add audio data to buffer
                for (let i = 0; i < data.length; i++) {
                    this.audioBuffer.push(data[i] * volume);
                }
            }
        };
    }
    
    process(inputs, outputs, parameters) {
        const output = outputs[0];
        if (output.length === 0) return true;
        
        const leftChannel = output[0];
        const rightChannel = output[1];
        const frameCount = leftChannel.length;
        
        // Fill output buffer from our audio buffer
        for (let i = 0; i < frameCount; i++) {
            if (this.audioBuffer.length >= 2) {
                leftChannel[i] = this.audioBuffer.shift();
                rightChannel[i] = this.audioBuffer.shift();
            } else {
                // No more audio data, output silence
                leftChannel[i] = 0;
                rightChannel[i] = 0;
            }
        }
        
        // Keep processor alive
        return true;
    }
}

registerProcessor('gameboy-processor', GameBoyProcessor);