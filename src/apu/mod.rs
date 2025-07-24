use crate::memory::Memory;

const SAMPLE_RATE: u32 = 44100;
const CYCLES_PER_SAMPLE: f32 = 4194304.0 / SAMPLE_RATE as f32;

pub struct Apu {
    channel1: SquareChannel,
    channel2: SquareChannel,
    channel3: WaveChannel,
    channel4: NoiseChannel,
    
    // Control registers
    nr50: u8,  // Master volume & VIN
    nr51: u8,  // Sound panning
    nr52: u8,  // Sound on/off
    
    // Frame sequencer
    frame_sequencer: u8,
    frame_sequencer_counter: u32,
    
    // Audio buffer
    sample_counter: f32,
    audio_buffer: Vec<f32>,
    enabled: bool,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            channel1: SquareChannel::new(true),
            channel2: SquareChannel::new(false),
            channel3: WaveChannel::new(),
            channel4: NoiseChannel::new(),
            
            nr50: 0,
            nr51: 0,
            nr52: 0,
            
            frame_sequencer: 0,
            frame_sequencer_counter: 0,
            
            sample_counter: 0.0,
            audio_buffer: Vec::new(),
            enabled: false,
        }
    }
    
    pub fn update(&mut self, cycles: u8, memory: &mut Memory) {
        if !self.enabled {
            return;
        }
        
        // Update frame sequencer (512 Hz)
        self.frame_sequencer_counter += cycles as u32;
        if self.frame_sequencer_counter >= 8192 {
            self.frame_sequencer_counter -= 8192;
            self.step_frame_sequencer();
        }
        
        // Generate samples
        self.sample_counter += cycles as f32;
        while self.sample_counter >= CYCLES_PER_SAMPLE {
            self.sample_counter -= CYCLES_PER_SAMPLE;
            self.generate_sample();
        }
        
        // Sync with memory
        self.sync_registers(memory);
    }
    
    pub fn get_audio_buffer(&mut self) -> Vec<f32> {
        let buffer = self.audio_buffer.clone();
        self.audio_buffer.clear();
        buffer
    }
    
    fn step_frame_sequencer(&mut self) {
        match self.frame_sequencer {
            0 => {
                self.channel1.clock_length();
                self.channel2.clock_length();
                self.channel3.clock_length();
                self.channel4.clock_length();
            }
            2 => {
                self.channel1.clock_length();
                self.channel2.clock_length();
                self.channel3.clock_length();
                self.channel4.clock_length();
                self.channel1.clock_sweep();
            }
            4 => {
                self.channel1.clock_length();
                self.channel2.clock_length();
                self.channel3.clock_length();
                self.channel4.clock_length();
            }
            6 => {
                self.channel1.clock_length();
                self.channel2.clock_length();
                self.channel3.clock_length();
                self.channel4.clock_length();
                self.channel1.clock_sweep();
            }
            7 => {
                self.channel1.clock_envelope();
                self.channel2.clock_envelope();
                self.channel4.clock_envelope();
            }
            _ => {}
        }
        
        self.frame_sequencer = (self.frame_sequencer + 1) & 7;
    }
    
    fn generate_sample(&mut self) {
        let mut left = 0.0;
        let mut right = 0.0;
        
        // Mix channels
        if self.channel1.enabled {
            let sample = self.channel1.get_sample();
            if (self.nr51 & 0x10) != 0 { right += sample; }
            if (self.nr51 & 0x01) != 0 { left += sample; }
        }
        
        if self.channel2.enabled {
            let sample = self.channel2.get_sample();
            if (self.nr51 & 0x20) != 0 { right += sample; }
            if (self.nr51 & 0x02) != 0 { left += sample; }
        }
        
        if self.channel3.enabled {
            let sample = self.channel3.get_sample();
            if (self.nr51 & 0x40) != 0 { right += sample; }
            if (self.nr51 & 0x04) != 0 { left += sample; }
        }
        
        if self.channel4.enabled {
            let sample = self.channel4.get_sample();
            if (self.nr51 & 0x80) != 0 { right += sample; }
            if (self.nr51 & 0x08) != 0 { left += sample; }
        }
        
        // Apply master volume
        let left_vol = ((self.nr50 & 0x07) + 1) as f32 / 8.0;
        let right_vol = (((self.nr50 >> 4) & 0x07) + 1) as f32 / 8.0;
        
        left *= left_vol / 4.0;
        right *= right_vol / 4.0;
        
        // Stereo output
        self.audio_buffer.push(left);
        self.audio_buffer.push(right);
    }
    
    fn sync_registers(&mut self, memory: &mut Memory) {
        // Read control registers
        self.nr50 = memory.read_byte(0xFF24);
        self.nr51 = memory.read_byte(0xFF25);
        let new_nr52 = memory.read_byte(0xFF26);
        
        // Handle sound on/off
        if (new_nr52 & 0x80) != 0 && !self.enabled {
            self.enabled = true;
            self.reset();
        } else if (new_nr52 & 0x80) == 0 && self.enabled {
            self.enabled = false;
            self.reset();
        }
        
        if self.enabled {
            // Update channel registers
            self.channel1.sync_registers(memory);
            self.channel2.sync_registers(memory);
            self.channel3.sync_registers(memory);
            self.channel4.sync_registers(memory);
            
            // Update status bits
            let mut status = 0x80;
            if self.channel1.enabled { status |= 0x01; }
            if self.channel2.enabled { status |= 0x02; }
            if self.channel3.enabled { status |= 0x04; }
            if self.channel4.enabled { status |= 0x08; }
            memory.write_byte(0xFF26, status);
        } else {
            // Clear all sound registers when disabled
            for addr in 0xFF10..=0xFF25 {
                memory.write_byte(addr, 0);
            }
            memory.write_byte(0xFF26, 0);
        }
    }
    
    fn reset(&mut self) {
        self.channel1 = SquareChannel::new(true);
        self.channel2 = SquareChannel::new(false);
        self.channel3 = WaveChannel::new();
        self.channel4 = NoiseChannel::new();
        self.frame_sequencer = 0;
        self.frame_sequencer_counter = 0;
    }
}

// Square wave channel (channels 1 and 2)
struct SquareChannel {
    // Registers
    nr0: u8,  // Sweep (channel 1 only)
    nr1: u8,  // Duty & length
    nr2: u8,  // Envelope
    nr3: u8,  // Frequency low
    nr4: u8,  // Frequency high & control
    
    // Internal state
    frequency: u16,
    frequency_timer: u16,
    duty_position: u8,
    length_counter: u8,
    envelope_timer: u8,
    envelope_period: u8,
    current_volume: u8,
    sweep_timer: u8,
    sweep_period: u8,
    sweep_shadow: u16,
    sweep_enabled: bool,
    has_sweep: bool,
    enabled: bool,
}

impl SquareChannel {
    fn new(has_sweep: bool) -> Self {
        Self {
            nr0: 0,
            nr1: 0,
            nr2: 0,
            nr3: 0,
            nr4: 0,
            frequency: 0,
            frequency_timer: 0,
            duty_position: 0,
            length_counter: 0,
            envelope_timer: 0,
            envelope_period: 0,
            current_volume: 0,
            sweep_timer: 0,
            sweep_period: 0,
            sweep_shadow: 0,
            sweep_enabled: false,
            has_sweep,
            enabled: false,
        }
    }
    
    fn sync_registers(&mut self, memory: &mut Memory) {
        let base = if self.has_sweep { 0xFF10 } else { 0xFF15 };
        
        if self.has_sweep {
            self.nr0 = memory.read_byte(base);
        }
        self.nr1 = memory.read_byte(base + 1);
        self.nr2 = memory.read_byte(base + 2);
        self.nr3 = memory.read_byte(base + 3);
        let new_nr4 = memory.read_byte(base + 4);
        
        // Trigger
        if (new_nr4 & 0x80) != 0 && (self.nr4 & 0x80) == 0 {
            self.trigger();
        }
        
        self.nr4 = new_nr4;
        self.frequency = ((self.nr4 as u16 & 0x07) << 8) | self.nr3 as u16;
    }
    
    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_counter == 0 {
            self.length_counter = 64;
        }
        self.frequency_timer = (2048 - self.frequency) * 4;
        self.envelope_timer = self.envelope_period;
        self.current_volume = (self.nr2 >> 4) & 0x0F;
        
        if self.has_sweep {
            self.sweep_shadow = self.frequency;
            self.sweep_period = (self.nr0 >> 4) & 0x07;
            self.sweep_timer = if self.sweep_period != 0 { self.sweep_period } else { 8 };
            self.sweep_enabled = self.sweep_period != 0 || ((self.nr0 >> 3) & 0x07) != 0;
            
            if ((self.nr0 >> 3) & 0x07) != 0 {
                self.calculate_sweep();
            }
        }
    }
    
    fn get_sample(&mut self) -> f32 {
        self.frequency_timer = self.frequency_timer.saturating_sub(1);
        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.frequency) * 4;
            self.duty_position = (self.duty_position + 1) & 7;
        }
        
        let duty = match (self.nr1 >> 6) & 0x03 {
            0 => 0b00000001,
            1 => 0b10000001,
            2 => 0b10000111,
            3 => 0b01111110,
            _ => unreachable!(),
        };
        
        let output = if (duty >> self.duty_position) & 1 != 0 {
            self.current_volume
        } else {
            0
        };
        
        (output as f32 / 15.0) * 2.0 - 1.0
    }
    
    fn clock_length(&mut self) {
        if (self.nr4 & 0x40) != 0 && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }
    
    fn clock_envelope(&mut self) {
        if self.envelope_period != 0 {
            if self.envelope_timer > 0 {
                self.envelope_timer -= 1;
            }
            
            if self.envelope_timer == 0 {
                self.envelope_timer = self.envelope_period;
                
                if (self.nr2 & 0x08) != 0 {
                    if self.current_volume < 15 {
                        self.current_volume += 1;
                    }
                } else {
                    if self.current_volume > 0 {
                        self.current_volume -= 1;
                    }
                }
            }
        }
    }
    
    fn clock_sweep(&mut self) {
        if !self.has_sweep || !self.sweep_enabled {
            return;
        }
        
        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }
        
        if self.sweep_timer == 0 {
            self.sweep_timer = if self.sweep_period != 0 { self.sweep_period } else { 8 };
            
            if self.sweep_enabled && self.sweep_period != 0 {
                let new_freq = self.calculate_sweep();
                if new_freq <= 2047 && ((self.nr0 >> 3) & 0x07) != 0 {
                    self.frequency = new_freq;
                    self.sweep_shadow = new_freq;
                    self.calculate_sweep();
                }
            }
        }
    }
    
    fn calculate_sweep(&mut self) -> u16 {
        let shift = self.nr0 & 0x07;
        let mut new_freq = self.sweep_shadow >> shift;
        
        if (self.nr0 & 0x08) != 0 {
            new_freq = self.sweep_shadow.wrapping_sub(new_freq);
        } else {
            new_freq = self.sweep_shadow.wrapping_add(new_freq);
        }
        
        if new_freq > 2047 {
            self.enabled = false;
        }
        
        new_freq
    }
}

// Wave channel (channel 3)
struct WaveChannel {
    // Registers
    nr30: u8,  // DAC on/off
    nr31: u8,  // Length
    nr32: u8,  // Volume
    nr33: u8,  // Frequency low
    nr34: u8,  // Frequency high & control
    
    // Wave pattern RAM
    wave_ram: [u8; 16],
    
    // Internal state
    frequency: u16,
    frequency_timer: u16,
    position: u8,
    length_counter: u16,
    volume_shift: u8,
    enabled: bool,
    dac_enabled: bool,
}

impl WaveChannel {
    fn new() -> Self {
        Self {
            nr30: 0,
            nr31: 0,
            nr32: 0,
            nr33: 0,
            nr34: 0,
            wave_ram: [0; 16],
            frequency: 0,
            frequency_timer: 0,
            position: 0,
            length_counter: 0,
            volume_shift: 0,
            enabled: false,
            dac_enabled: false,
        }
    }
    
    fn sync_registers(&mut self, memory: &mut Memory) {
        self.nr30 = memory.read_byte(0xFF1A);
        self.nr31 = memory.read_byte(0xFF1B);
        self.nr32 = memory.read_byte(0xFF1C);
        self.nr33 = memory.read_byte(0xFF1D);
        let new_nr34 = memory.read_byte(0xFF1E);
        
        // Read wave RAM
        for i in 0..16 {
            self.wave_ram[i] = memory.read_byte(0xFF30 + i as u16);
        }
        
        self.dac_enabled = (self.nr30 & 0x80) != 0;
        self.volume_shift = match (self.nr32 >> 5) & 0x03 {
            0 => 4,  // Mute
            1 => 0,  // 100%
            2 => 1,  // 50%
            3 => 2,  // 25%
            _ => unreachable!(),
        };
        
        // Trigger
        if (new_nr34 & 0x80) != 0 && (self.nr34 & 0x80) == 0 {
            self.trigger();
        }
        
        self.nr34 = new_nr34;
        self.frequency = ((self.nr34 as u16 & 0x07) << 8) | self.nr33 as u16;
    }
    
    fn trigger(&mut self) {
        self.enabled = self.dac_enabled;
        if self.length_counter == 0 {
            self.length_counter = 256;
        }
        self.frequency_timer = (2048 - self.frequency) * 2;
        self.position = 0;
    }
    
    fn get_sample(&mut self) -> f32 {
        if !self.dac_enabled {
            return 0.0;
        }
        
        self.frequency_timer = self.frequency_timer.saturating_sub(1);
        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.frequency) * 2;
            self.position = (self.position + 1) & 31;
        }
        
        let sample_index = self.position / 2;
        let sample = if self.position & 1 == 0 {
            self.wave_ram[sample_index as usize] >> 4
        } else {
            self.wave_ram[sample_index as usize] & 0x0F
        };
        
        let output = sample >> self.volume_shift;
        (output as f32 / 15.0) * 2.0 - 1.0
    }
    
    fn clock_length(&mut self) {
        if (self.nr34 & 0x40) != 0 && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }
}

// Noise channel (channel 4)
struct NoiseChannel {
    // Registers
    nr41: u8,  // Length
    nr42: u8,  // Envelope
    nr43: u8,  // Polynomial counter
    nr44: u8,  // Control
    
    // Internal state
    length_counter: u8,
    envelope_timer: u8,
    envelope_period: u8,
    current_volume: u8,
    lfsr: u16,
    timer: u16,
    enabled: bool,
}

impl NoiseChannel {
    fn new() -> Self {
        Self {
            nr41: 0,
            nr42: 0,
            nr43: 0,
            nr44: 0,
            length_counter: 0,
            envelope_timer: 0,
            envelope_period: 0,
            current_volume: 0,
            lfsr: 0x7FFF,
            timer: 0,
            enabled: false,
        }
    }
    
    fn sync_registers(&mut self, memory: &mut Memory) {
        self.nr41 = memory.read_byte(0xFF20);
        self.nr42 = memory.read_byte(0xFF21);
        self.nr43 = memory.read_byte(0xFF22);
        let new_nr44 = memory.read_byte(0xFF23);
        
        self.envelope_period = self.nr42 & 0x07;
        
        // Trigger
        if (new_nr44 & 0x80) != 0 && (self.nr44 & 0x80) == 0 {
            self.trigger();
        }
        
        self.nr44 = new_nr44;
    }
    
    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_counter == 0 {
            self.length_counter = 64;
        }
        self.timer = self.get_period();
        self.envelope_timer = self.envelope_period;
        self.current_volume = (self.nr42 >> 4) & 0x0F;
        self.lfsr = 0x7FFF;
    }
    
    fn get_period(&self) -> u16 {
        let divisor = match self.nr43 & 0x07 {
            0 => 8,
            n => (n as u16) * 16,
        };
        divisor << (self.nr43 >> 4)
    }
    
    fn get_sample(&mut self) -> f32 {
        self.timer = self.timer.saturating_sub(1);
        if self.timer == 0 {
            self.timer = self.get_period();
            
            let bit = (self.lfsr & 1) ^ ((self.lfsr >> 1) & 1);
            self.lfsr = (self.lfsr >> 1) | (bit << 14);
            
            if (self.nr43 & 0x08) != 0 {
                self.lfsr &= !0x40;
                self.lfsr |= bit << 6;
            }
        }
        
        let output = if (self.lfsr & 1) == 0 {
            self.current_volume
        } else {
            0
        };
        
        (output as f32 / 15.0) * 2.0 - 1.0
    }
    
    fn clock_length(&mut self) {
        if (self.nr44 & 0x40) != 0 && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }
    
    fn clock_envelope(&mut self) {
        if self.envelope_period != 0 {
            if self.envelope_timer > 0 {
                self.envelope_timer -= 1;
            }
            
            if self.envelope_timer == 0 {
                self.envelope_timer = self.envelope_period;
                
                if (self.nr42 & 0x08) != 0 {
                    if self.current_volume < 15 {
                        self.current_volume += 1;
                    }
                } else {
                    if self.current_volume > 0 {
                        self.current_volume -= 1;
                    }
                }
            }
        }
    }
}