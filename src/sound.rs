use crate::haptic::HapticPattern;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;
use std::sync::Mutex;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SoundProfile {
    GrandPiano = 0,
    MusicalMarimba = 1,
    DeepThock = 2,
    ClickyBlue = 3,
    Typewriter = 4,
}

impl SoundProfile {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => SoundProfile::GrandPiano,
            1 => SoundProfile::MusicalMarimba,
            2 => SoundProfile::DeepThock,
            3 => SoundProfile::ClickyBlue,
            4 => SoundProfile::Typewriter,
            _ => SoundProfile::GrandPiano,
        }
    }
}

pub struct SoundEngine {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    // [profile 0..4][key_code 0..128] -> WAV bytes
    key_wavs: Vec<Vec<Vec<u8>>>,
    // [pattern 0..3] -> WAV bytes
    tick_wavs: Vec<Vec<u8>>,
}

// Rodio OutputStream & handle are safe to access across threads when protected by Mutex
unsafe impl Send for SoundEngine {}
unsafe impl Sync for SoundEngine {}

static GLOBAL_SOUND_ENGINE: Mutex<Option<SoundEngine>> = Mutex::new(None);

/// Generates a subtle haptic tick sound WAV in memory
fn generate_haptic_tick_wav(pattern: HapticPattern) -> Vec<u8> {
    let sample_rate = 44100.0;
    let duration = match pattern {
        HapticPattern::Generic => 0.015,
        HapticPattern::Alignment => 0.022,
        HapticPattern::LevelChange => 0.030,
    };

    let total_samples = (sample_rate * duration) as usize;
    let mut pcm = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f64 / sample_rate;
        let sample: f64 = match pattern {
            HapticPattern::Generic => {
                let snap = (2.0 * std::f64::consts::PI * 2600.0 * t).sin() * (-t * 500.0).exp();
                let pop = (2.0 * std::f64::consts::PI * 480.0 * t).sin() * (-t * 300.0).exp();
                (snap * 0.55 + pop * 0.45) * 0.60
            }
            HapticPattern::Alignment => {
                let snap = (2.0 * std::f64::consts::PI * 1900.0 * t).sin() * (-t * 350.0).exp();
                let pop = (2.0 * std::f64::consts::PI * 340.0 * t).sin() * (-t * 180.0).exp();
                (snap * 0.60 + pop * 0.50) * 0.80
            }
            HapticPattern::LevelChange => {
                let snap = (2.0 * std::f64::consts::PI * 1300.0 * t).sin() * (-t * 260.0).exp();
                let sub = (2.0 * std::f64::consts::PI * 200.0 * t).sin() * (-t * 120.0).exp();
                (snap * 0.50 + sub * 0.65) * 0.95
            }
        };

        let clamped = sample.clamp(-1.0, 1.0);
        pcm.push((clamped * 32767.0) as i16);
    }

    encode_wav_pcm(&pcm)
}

/// Returns the exact musical frequency (Hz) for every key on the keyboard (Major Pentatonic / Diatonic)
pub fn get_key_frequency(key_code: u16) -> f64 {
    match key_code {
        // Spacebar - Deep G2 Bass (warm, resonant foundation)
        49 => 98.00,

        // Return / Enter - Solid C3 Bass
        36 => 130.81,

        // Backspace / Delete - D3 Accent
        51 => 146.83,

        // Tab - E3
        48 => 164.81,

        // Escape - High C5
        53 => 523.25,

        // Bottom Row (Z, X, C, V, B, N, M, ,, ., /)
        6 => 196.00,  // Z (G3)
        7 => 220.00,  // X (A3)
        8 => 261.63,  // C (C4 - Middle C)
        9 => 293.66,  // V (D4)
        11 => 329.63, // B (E4)
        45 => 392.00, // N (G4)
        46 => 440.00, // M (A4)
        43 => 523.25, // , (C5)
        47 => 587.33, // . (D5)
        44 => 659.25, // / (E5)

        // Home Row (A, S, D, F, G, H, J, K, L, ;, ')
        0 => 261.63,  // A (C4)
        1 => 293.66,  // S (D4)
        2 => 329.63,  // D (E4)
        3 => 392.00,  // F (G4)
        5 => 440.00,  // G (A4)
        4 => 523.25,  // H (C5)
        38 => 587.33, // J (D5)
        40 => 659.25, // K (E5)
        37 => 783.99, // L (G5)
        41 => 880.00, // ; (A5)
        39 => 1046.50, // ' (C6)

        // Top Row (Q, W, E, R, T, Y, U, I, O, P, [, ])
        12 => 392.00, // Q (G4)
        13 => 440.00, // W (A4)
        14 => 523.25, // E (C5)
        15 => 587.33, // R (D5)
        17 => 659.25, // T (E5)
        16 => 783.99, // Y (G5)
        32 => 880.00, // U (A5)
        34 => 1046.50, // I (C6)
        31 => 1174.66, // O (D6)
        35 => 1318.51, // P (E6)
        33 => 1567.98, // [ (G6)
        30 => 1760.00, // ] (A6)

        // Number Row (1, 2, 3, 4, 5, 6, 7, 8, 9, 0, -, =)
        18 => 523.25,  // 1 (C5)
        19 => 587.33,  // 2 (D5)
        20 => 659.25,  // 3 (E5)
        21 => 783.99,  // 4 (G5)
        23 => 880.00,  // 5 (A5)
        22 => 1046.50, // 6 (C6)
        26 => 1174.66, // 7 (D6)
        28 => 1318.51, // 8 (E6)
        25 => 1567.98, // 9 (G6)
        29 => 1760.00, // 0 (A6)
        27 => 2093.00, // - (C7)
        24 => 2349.32, // = (D7)

        // Arrow keys
        123 => 329.63, // Left (E4)
        125 => 261.63, // Down (C4)
        124 => 392.00, // Right (G4)
        126 => 523.25, // Up (C5)

        // Stable musical pentatonic distribution for any extra keys
        other => {
            const PENTATONIC: [f64; 10] = [
                261.63, 293.66, 329.63, 392.00, 440.00,
                523.25, 587.33, 659.25, 783.99, 880.00,
            ];
            PENTATONIC[(other as usize * 7 + 3) % PENTATONIC.len()]
        }
    }
}

/// Generates a rich, distinct musical mechanical switch WAV in memory for a specific keycode
fn generate_key_wav(profile: SoundProfile, key_code: u16, vol_multiplier: f64) -> Vec<u8> {
    let sample_rate = 44100.0;
    let freq = get_key_frequency(key_code);

    let duration = match profile {
        SoundProfile::GrandPiano => 0.130,
        SoundProfile::MusicalMarimba => 0.085,
        SoundProfile::DeepThock => 0.065,
        SoundProfile::ClickyBlue => 0.055,
        SoundProfile::Typewriter => 0.075,
    };

    let total_samples = (sample_rate * duration) as usize;
    let mut pcm = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f64 / sample_rate;
        let sample: f64 = match profile {
            SoundProfile::GrandPiano => {
                // Concert Grand Piano with rich string harmonics and subtle felt hammer strike
                let hammer_freq = (freq * 1.5).clamp(80.0, 350.0);
                let strike = (2.0 * std::f64::consts::PI * hammer_freq * t).sin() * (-t * 220.0).exp() * 0.22;

                let h1 = (2.0 * std::f64::consts::PI * freq * t).sin() * (-t * 14.0).exp() * 0.65;
                let h2 = (2.0 * std::f64::consts::PI * (freq * 2.0) * t).sin() * (-t * 20.0).exp() * 0.38;
                let h3 = (2.0 * std::f64::consts::PI * (freq * 3.0) * t).sin() * (-t * 28.0).exp() * 0.22;
                let h4 = (2.0 * std::f64::consts::PI * (freq * 4.0) * t).sin() * (-t * 36.0).exp() * 0.12;
                let h5 = (2.0 * std::f64::consts::PI * (freq * 5.0) * t).sin() * (-t * 48.0).exp() * 0.06;

                (strike + h1 + h2 + h3 + h4 + h5) * vol_multiplier
            }
            SoundProfile::MusicalMarimba => {
                // Pure wooden chime / marimba note with rich harmonic overtones
                let strike = (2.0 * std::f64::consts::PI * (freq * 4.0).min(12000.0) * t).sin() * (-t * 350.0).exp() * 0.25;
                let tone = (2.0 * std::f64::consts::PI * freq * t).sin() * (-t * 26.0).exp();
                let overtone = (2.0 * std::f64::consts::PI * (freq * 2.0) * t).sin() * (-t * 40.0).exp() * 0.35;
                let chime = (2.0 * std::f64::consts::PI * (freq * 3.0) * t).sin() * (-t * 65.0).exp() * 0.15;

                (strike + tone * 0.70 + overtone + chime) * vol_multiplier
            }
            SoundProfile::DeepThock => {
                // Creamy mechanical thock pop + distinct melodic pitch body
                let pop_freq = if key_code == 49 { 140.0 } else { freq * 0.75 };
                let thock = (2.0 * std::f64::consts::PI * pop_freq * t).sin() * (-t * 60.0).exp() * 0.60;
                let melodic = (2.0 * std::f64::consts::PI * freq * t).sin() * (-t * 35.0).exp() * 0.45;
                let snap = (2.0 * std::f64::consts::PI * 1800.0 * t).sin() * (-t * 350.0).exp() * 0.25;

                (thock + melodic + snap) * vol_multiplier
            }
            SoundProfile::ClickyBlue => {
                // Crisp clicky snap (4kHz) + vibrant musical chime
                let click = (2.0 * std::f64::consts::PI * 4200.0 * t).sin() * (-t * 350.0).exp() * 0.55;
                let chime = (2.0 * std::f64::consts::PI * freq * t).sin() * (-t * 40.0).exp() * 0.50;

                (click + chime) * vol_multiplier
            }
            SoundProfile::Typewriter => {
                // Mechanical lever click + tuned metallic acoustic ring
                let strike = (2.0 * std::f64::consts::PI * 420.0 * t).sin() * (-t * 160.0).exp() * 0.55;
                let ring = (2.0 * std::f64::consts::PI * freq * t).sin() * (-t * 30.0).exp() * 0.45;

                (strike + ring) * vol_multiplier
            }
        };

        let clamped = sample.clamp(-1.0, 1.0);
        pcm.push((clamped * 32767.0) as i16);
    }

    encode_wav_pcm(&pcm)
}

fn encode_wav_pcm(pcm: &[i16]) -> Vec<u8> {
    let data_len = (pcm.len() * 2) as u32;
    let mut wav = Vec::with_capacity(44 + data_len as usize);

    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(data_len + 36).to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&44100u32.to_le_bytes());
    wav.extend_from_slice(&(44100u32 * 2).to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());

    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    for s in pcm {
        wav.extend_from_slice(&s.to_le_bytes());
    }

    wav
}

/// Initializes the global sound engine with pre-rendered WAV buffers
pub fn init_sound_engine(vol_pct: u8) {
    let mut guard = GLOBAL_SOUND_ENGINE.lock().unwrap();
    if guard.is_some() {
        return;
    }

    let vol_multiplier = (vol_pct as f64 / 100.0).clamp(0.1, 1.0);

    // Try to obtain a default CoreAudio output stream
    if let Ok((stream, stream_handle)) = OutputStream::try_default() {
        let profiles = [
            SoundProfile::GrandPiano,
            SoundProfile::MusicalMarimba,
            SoundProfile::DeepThock,
            SoundProfile::ClickyBlue,
            SoundProfile::Typewriter,
        ];

        let mut key_wavs = Vec::with_capacity(profiles.len());

        for &profile in &profiles {
            let mut wavs_for_profile = Vec::with_capacity(128);
            for key_code in 0..128u16 {
                let wav = generate_key_wav(profile, key_code, vol_multiplier);
                wavs_for_profile.push(wav);
            }
            key_wavs.push(wavs_for_profile);
        }

        let tick_wavs = vec![
            generate_haptic_tick_wav(HapticPattern::Generic),
            generate_haptic_tick_wav(HapticPattern::Alignment),
            generate_haptic_tick_wav(HapticPattern::LevelChange),
        ];

        *guard = Some(SoundEngine {
            _stream: stream,
            stream_handle,
            key_wavs,
            tick_wavs,
        });

        println!("[Haptic Sound] CoreAudio direct output stream initialized with 128 musical key notes.");
    } else {
        eprintln!("[Haptic Sound] Warning: Could not initialize CoreAudio default audio output device.");
    }
}

/// Plays a haptic tick sound effect directly to CoreAudio
pub fn play_haptic_tick_sound(pattern: HapticPattern, volume_pct: u8) {
    if volume_pct == 0 {
        return;
    }

    let mut guard = GLOBAL_SOUND_ENGINE.lock().unwrap();
    if guard.is_none() {
        drop(guard);
        init_sound_engine(volume_pct);
        guard = GLOBAL_SOUND_ENGINE.lock().unwrap();
    }

    if let Some(engine) = guard.as_ref() {
        let idx = (pattern as usize).min(engine.tick_wavs.len() - 1);
        let wav = &engine.tick_wavs[idx];

        if let Ok(sink) = Sink::try_new(&engine.stream_handle) {
            let vol_float = (volume_pct as f32 / 100.0).clamp(0.0, 1.0);
            sink.set_volume(vol_float);

            if let Ok(source) = Decoder::new(Cursor::new(wav.clone())) {
                sink.append(source);
                sink.detach();
            }
        }
    }
}

/// Plays a mechanical key sound effect directly to the CoreAudio default device
pub fn play_keyboard_sound(key_code: u16, profile: SoundProfile, volume_pct: u8) {
    if volume_pct == 0 {
        return;
    }

    let mut guard = GLOBAL_SOUND_ENGINE.lock().unwrap();
    if guard.is_none() {
        drop(guard);
        init_sound_engine(volume_pct);
        guard = GLOBAL_SOUND_ENGINE.lock().unwrap();
    }

    if let Some(engine) = guard.as_ref() {
        let profile_idx = (profile as usize).min(engine.key_wavs.len() - 1);
        let key_idx = (key_code as usize).min(127);
        let wav = &engine.key_wavs[profile_idx][key_idx];

        if let Ok(sink) = Sink::try_new(&engine.stream_handle) {
            // Precise linear volume scaling: 0.0 to 1.0
            let vol_float = (volume_pct as f32 / 100.0).clamp(0.0, 1.0);
            sink.set_volume(vol_float);

            if let Ok(source) = Decoder::new(Cursor::new(wav.clone())) {
                sink.append(source);
                sink.detach();
            }
        }
    }
}

/// Plays a subtle speaker audio tick to simulate haptic feedback
pub fn play_haptic_audio_tick(pattern: HapticPattern) {
    play_haptic_tick_sound(pattern, 80);
}
