use crate::haptic::HapticPattern;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;
use std::sync::Mutex;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SoundProfile {
    ClickyBlue = 0,
    DeepThock = 1,
    Typewriter = 2,
}

impl SoundProfile {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => SoundProfile::ClickyBlue,
            1 => SoundProfile::DeepThock,
            2 => SoundProfile::Typewriter,
            _ => SoundProfile::ClickyBlue,
        }
    }
}

pub struct SoundEngine {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    // [profile 0..3][key_type 0..3] -> WAV bytes
    keyboard_wavs: Vec<Vec<Vec<u8>>>,
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

/// Generates a realistic mechanical switch WAV in memory
fn generate_switch_wav(profile: SoundProfile, key_type: usize, vol_multiplier: f64) -> Vec<u8> {
    let sample_rate = 44100.0;
    let duration = match (profile, key_type) {
        (_, 1) => 0.065, // Spacebar
        (_, 2) => 0.055, // Enter / Backspace
        (SoundProfile::ClickyBlue, _) => 0.040,
        (SoundProfile::DeepThock, _) => 0.048,
        (SoundProfile::Typewriter, _) => 0.058,
    };

    let total_samples = (sample_rate * duration) as usize;
    let mut pcm = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f64 / sample_rate;
        let sample: f64 = match profile {
            SoundProfile::ClickyBlue => {
                let click_env = (-t * 400.0).exp();
                let click = (2.0 * std::f64::consts::PI * 3500.0 * t).sin() * click_env;

                let body_freq = if key_type == 1 { 220.0 } else if key_type == 2 { 320.0 } else { 580.0 };
                let body_env = (-t * 110.0).exp();
                let body = (2.0 * std::f64::consts::PI * body_freq * t).sin() * body_env;

                let noise = ((i * 1103515245 + 12345) % 1000) as f64 / 500.0 - 1.0;
                let noise_env = (-t * 500.0).exp();

                (click * 0.70 + body * 0.40 + noise * noise_env * 0.25) * vol_multiplier
            }
            SoundProfile::DeepThock => {
                let base_freq = if key_type == 1 { 150.0 } else if key_type == 2 { 230.0 } else { 370.0 };
                let thock_env = (-t * 120.0).exp();
                let thock = (2.0 * std::f64::consts::PI * base_freq * t).sin() * thock_env;

                let sub_thock = (2.0 * std::f64::consts::PI * (base_freq * 0.5) * t).sin() * (-t * 80.0).exp();

                let pop_noise = ((i * 1664525 + 1013904223) % 1000) as f64 / 500.0 - 1.0;
                let pop_env = (-t * 300.0).exp();

                (thock * 0.75 + sub_thock * 0.40 + pop_noise * pop_env * 0.20) * vol_multiplier
            }
            SoundProfile::Typewriter => {
                let strike_freq = if key_type == 1 { 180.0 } else { 420.0 };
                let strike = (2.0 * std::f64::consts::PI * strike_freq * t).sin() * (-t * 160.0).exp();

                let metal_ring = (2.0 * std::f64::consts::PI * 2200.0 * t).sin() * (-t * 50.0).exp();
                let noise = ((i * 214013 + 2531011) % 1000) as f64 / 500.0 - 1.0;

                (strike * 0.65 + metal_ring * 0.30 + noise * (-t * 450.0).exp() * 0.30) * vol_multiplier
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

pub fn init_sound_engine(_volume: u8) {
    let mut guard = GLOBAL_SOUND_ENGINE.lock().unwrap();
    if guard.is_some() {
        return;
    }

    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[Haptic Sound] Failed to open CoreAudio output stream: {:?}", e);
            return;
        }
    };

    let profiles = [
        SoundProfile::ClickyBlue,
        SoundProfile::DeepThock,
        SoundProfile::Typewriter,
    ];

    let mut keyboard_wavs = Vec::new();
    for (_p_idx, &profile) in profiles.iter().enumerate() {
        let mut key_list = Vec::new();
        for key_type in 0..3 {
            let wav = generate_switch_wav(profile, key_type, 1.0);
            key_list.push(wav);
        }
        keyboard_wavs.push(key_list);
    }

    let patterns = [
        HapticPattern::Generic,
        HapticPattern::Alignment,
        HapticPattern::LevelChange,
    ];
    let mut tick_wavs = Vec::new();
    for (_t_idx, &pat) in patterns.iter().enumerate() {
        let wav = generate_haptic_tick_wav(pat);
        tick_wavs.push(wav);
    }

    println!("[Haptic Sound] CoreAudio direct output stream initialized.");

    *guard = Some(SoundEngine {
        _stream,
        stream_handle,
        keyboard_wavs,
        tick_wavs,
    });
}

/// Plays a subtle speaker audio tick to simulate haptic feedback
pub fn play_haptic_audio_tick(pattern: HapticPattern) {
    let mut guard = GLOBAL_SOUND_ENGINE.lock().unwrap();
    if guard.is_none() {
        drop(guard);
        init_sound_engine(70);
        guard = GLOBAL_SOUND_ENGINE.lock().unwrap();
    }

    if let Some(engine) = guard.as_ref() {
        let pat_idx = match pattern {
            HapticPattern::Generic => 0,
            HapticPattern::Alignment => 1,
            HapticPattern::LevelChange => 2,
        };

        if pat_idx < engine.tick_wavs.len() {
            let wav = &engine.tick_wavs[pat_idx];
            if let Ok(sink) = Sink::try_new(&engine.stream_handle) {
                sink.set_volume(0.85);
                if let Ok(source) = Decoder::new(Cursor::new(wav.clone())) {
                    sink.append(source);
                    sink.detach();
                }
            }
        }
    }
}

/// Returns a unique, harmonious musical pitch multiplier (speed) for every key on the keyboard
pub fn get_key_pitch(key_code: u16) -> f32 {
    match key_code {
        // Spacebar - Deepest resonance
        49 => 0.74,

        // Return / Enter - Deep resonant thock
        36 => 0.82,

        // Backspace / Delete
        51 => 0.88,

        // Tab
        48 => 0.92,

        // Escape
        53 => 1.36,

        // Bottom Row (Z, X, C, V, B, N, M, ,, ., /)
        6 => 0.84,  // Z
        7 => 0.87,  // X
        8 => 0.90,  // C
        9 => 0.93,  // V
        11 => 0.96, // B
        45 => 0.99, // N
        46 => 1.02, // M
        43 => 1.05, // ,
        47 => 1.08, // .
        44 => 1.11, // /

        // Home Row (A, S, D, F, G, H, J, K, L, ;, ')
        0 => 0.95,  // A
        1 => 0.98,  // S
        2 => 1.01,  // D
        3 => 1.05,  // F
        5 => 1.09,  // G
        4 => 1.13,  // H
        38 => 1.17, // J
        40 => 1.21, // K
        37 => 1.25, // L
        41 => 1.28, // ;
        39 => 1.31, // '

        // Top Row (Q, W, E, R, T, Y, U, I, O, P, [, ])
        12 => 1.08, // Q
        13 => 1.11, // W
        14 => 1.15, // E
        15 => 1.18, // R
        17 => 1.22, // T
        16 => 1.25, // Y
        32 => 1.29, // U
        34 => 1.33, // I
        31 => 1.37, // O
        35 => 1.41, // P
        33 => 1.44, // [
        30 => 1.48, // ]

        // Number Row (1, 2, 3, 4, 5, 6, 7, 8, 9, 0, -, =)
        18 => 1.20, // 1
        19 => 1.23, // 2
        20 => 1.27, // 3
        21 => 1.30, // 4
        23 => 1.34, // 5
        22 => 1.37, // 6
        26 => 1.41, // 7
        28 => 1.45, // 8
        25 => 1.48, // 9
        29 => 1.52, // 0
        27 => 1.55, // -
        24 => 1.58, // =

        // Arrow keys
        123 => 0.90, // Left
        124 => 1.10, // Right
        125 => 0.85, // Down
        126 => 1.20, // Up

        // Fallback for any other modifier or special key: stable musical distribution
        other => 0.85 + (((other as usize * 137 + 41) % 30) as f32 * 0.017),
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
        let profile_idx = (profile as usize).min(engine.keyboard_wavs.len() - 1);

        // HID Usages: 0x2C (44) = Space, 0x28 (40) = Enter, 0x2A (42) = Delete/Backspace
        // Keycodes: 49 = Space, 36 = Return, 51 = Delete
        let key_type = match key_code {
            0x2C | 49 => 1,      // Space
            0x28 | 0x2A | 36 | 51 => 2, // Return / Backspace
            _ => 0,              // Default key
        };

        let wav = &engine.keyboard_wavs[profile_idx][key_type];
        let pitch = get_key_pitch(key_code);

        if let Ok(sink) = Sink::try_new(&engine.stream_handle) {
            // Precise linear volume scaling: 0.0 to 1.0
            let vol_float = (volume_pct as f32 / 100.0).clamp(0.0, 1.0);
            sink.set_volume(vol_float);
            sink.set_speed(pitch);

            if let Ok(source) = Decoder::new(Cursor::new(wav.clone())) {
                sink.append(source);
                sink.detach();
            }
        }
    }
}
