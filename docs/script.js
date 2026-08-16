// Web Audio API Synthesizer for Haptic Ticks & Mechanical Switches
let audioCtx = null;

function getAudioContext() {
    if (!audioCtx) {
        audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    }
    if (audioCtx.state === 'suspended') {
        audioCtx.resume();
    }
    return audioCtx;
}

let currentProfile = 'thock';
let currentIntensity = 'generic';
let pulseCount = 0;

// Synthesize Mechanical Switch Sound in Web Audio API
function playMechanicalKeySound(keyType = 'normal') {
    try {
        const ctx = getAudioContext();
        const now = ctx.currentTime;

        if (currentProfile === 'thock') {
            // Cream / Holy Panda Deep Thock
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();
            
            const baseFreq = keyType === 'space' ? 160 : keyType === 'enter' ? 240 : 380;
            osc.type = 'sine';
            osc.frequency.setValueAtTime(baseFreq * 1.8, now);
            osc.frequency.exponentialRampToValueAtTime(baseFreq, now + 0.015);
            
            gain.gain.setValueAtTime(0.7, now);
            gain.gain.exponentialRampToValueAtTime(0.001, now + 0.045);
            
            osc.connect(gain);
            gain.connect(ctx.destination);
            
            osc.start(now);
            osc.stop(now + 0.05);

            // Pop Noise
            const bufferSize = ctx.sampleRate * 0.02;
            const buffer = ctx.createBuffer(1, bufferSize, ctx.sampleRate);
            const data = buffer.getChannelData(0);
            for (let i = 0; i < bufferSize; i++) {
                data[i] = Math.random() * 2 - 1;
            }
            const noise = ctx.createBufferSource();
            noise.buffer = buffer;
            const noiseFilter = ctx.createBiquadFilter();
            noiseFilter.type = 'lowpass';
            noiseFilter.frequency.value = 1200;

            const noiseGain = ctx.createGain();
            noiseGain.gain.setValueAtTime(0.18, now);
            noiseGain.gain.exponentialRampToValueAtTime(0.001, now + 0.02);

            noise.connect(noiseFilter);
            noiseFilter.connect(noiseGain);
            noiseGain.connect(ctx.destination);

            noise.start(now);
            noise.stop(now + 0.025);

        } else if (currentProfile === 'blue') {
            // Clicky Blue Switch
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();

            osc.type = 'triangle';
            osc.frequency.setValueAtTime(3600, now);
            osc.frequency.exponentialRampToValueAtTime(800, now + 0.02);

            gain.gain.setValueAtTime(0.6, now);
            gain.gain.exponentialRampToValueAtTime(0.001, now + 0.035);

            osc.connect(gain);
            gain.connect(ctx.destination);

            osc.start(now);
            osc.stop(now + 0.04);

            // Metallic transient
            const snap = ctx.createOscillator();
            const snapGain = ctx.createGain();
            snap.type = 'sine';
            snap.frequency.setValueAtTime(6000, now);
            snapGain.gain.setValueAtTime(0.3, now);
            snapGain.gain.exponentialRampToValueAtTime(0.001, now + 0.008);
            snap.connect(snapGain);
            snapGain.connect(ctx.destination);
            snap.start(now);
            snap.stop(now + 0.01);

        } else if (currentProfile === 'typewriter') {
            // Vintage Typewriter
            const strike = ctx.createOscillator();
            const strikeGain = ctx.createGain();
            const strikeFreq = keyType === 'space' ? 220 : 480;

            strike.type = 'triangle';
            strike.frequency.setValueAtTime(strikeFreq * 2, now);
            strike.frequency.exponentialRampToValueAtTime(strikeFreq, now + 0.015);
            strikeGain.gain.setValueAtTime(0.65, now);
            strikeGain.gain.exponentialRampToValueAtTime(0.001, now + 0.05);

            strike.connect(strikeGain);
            strikeGain.connect(ctx.destination);
            strike.start(now);
            strike.stop(now + 0.055);

            // Ring
            const ring = ctx.createOscillator();
            const ringGain = ctx.createGain();
            ring.type = 'sine';
            ring.frequency.setValueAtTime(2200, now);
            ringGain.gain.setValueAtTime(0.2, now);
            ringGain.gain.exponentialRampToValueAtTime(0.001, now + 0.06);
            ring.connect(ringGain);
            ringGain.connect(ctx.destination);
            ring.start(now);
            ring.stop(now + 0.065);
        }
    } catch (e) {
        console.error("Audio error:", e);
    }
}

// Synthesize Haptic Tick Sound (Digital Crown / Trackpad simulated tick)
function playHapticTick() {
    try {
        const ctx = getAudioContext();
        const now = ctx.currentTime;

        const osc = ctx.createOscillator();
        const gain = ctx.createGain();

        let freq = 2400;
        let decay = 0.012;
        let volume = 0.45;

        if (currentIntensity === 'alignment') {
            freq = 1800;
            decay = 0.018;
            volume = 0.6;
        } else if (currentIntensity === 'level') {
            freq = 1200;
            decay = 0.024;
            volume = 0.75;
        }

        osc.type = 'sine';
        osc.frequency.setValueAtTime(freq, now);
        osc.frequency.exponentialRampToValueAtTime(freq * 0.4, now + decay);

        gain.gain.setValueAtTime(volume, now);
        gain.gain.exponentialRampToValueAtTime(0.001, now + decay);

        osc.connect(gain);
        gain.connect(ctx.destination);

        osc.start(now);
        osc.stop(now + decay + 0.005);
    } catch (e) {
        console.error("Haptic audio error:", e);
    }
}

// Setup Interactive Elements
document.addEventListener('DOMContentLoaded', () => {
    // 1. Profile Switcher
    const profileButtons = document.querySelectorAll('#soundProfileGroup .toggle-btn');
    profileButtons.forEach(btn => {
        btn.addEventListener('click', () => {
            profileButtons.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            currentProfile = btn.getAttribute('data-profile');
            playMechanicalKeySound('enter');
        });
    });

    // 2. Intensity Switcher
    const intensityButtons = document.querySelectorAll('#hapticIntensityGroup .toggle-btn');
    intensityButtons.forEach(btn => {
        btn.addEventListener('click', () => {
            intensityButtons.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            currentIntensity = btn.getAttribute('data-intensity');
            playHapticTick();
        });
    });

    // 3. Trackpad Interaction Area
    const trackpad = document.getElementById('demoTrackpad');
    const visualizer = document.getElementById('hapticPulse');
    const pulseCountEl = document.getElementById('pulseCount');

    let lastX = 0;
    let lastY = 0;
    let accumulatedDist = 0;
    let accumulatedScroll = 0;
    const DIST_THRESHOLD = 35;
    const SCROLL_THRESHOLD = 20;

    function triggerHapticPulse(x, y) {
        pulseCount++;
        pulseCountEl.textContent = pulseCount;

        playHapticTick();

        if (x !== undefined && y !== undefined) {
            const rect = trackpad.getBoundingClientRect();
            visualizer.style.left = `${x - rect.left}px`;
            visualizer.style.top = `${y - rect.top}px`;
            visualizer.classList.remove('pulse');
            void visualizer.offsetWidth; // Trigger reflow
            visualizer.classList.add('pulse');
        }
    }

    trackpad.addEventListener('mousemove', (e) => {
        if (lastX !== 0 && lastY !== 0) {
            const dx = e.clientX - lastX;
            const dy = e.clientY - lastY;
            const dist = Math.sqrt(dx * dx + dy * dy);
            accumulatedDist += dist;

            if (accumulatedDist >= DIST_THRESHOLD) {
                triggerHapticPulse(e.clientX, e.clientY);
                accumulatedDist = 0;
            }
        }
        lastX = e.clientX;
        lastY = e.clientY;
    });

    trackpad.addEventListener('mouseleave', () => {
        lastX = 0;
        lastY = 0;
    });

    trackpad.addEventListener('wheel', (e) => {
        e.preventDefault();
        accumulatedScroll += Math.abs(e.deltaY) + Math.abs(e.deltaX);

        if (accumulatedScroll >= SCROLL_THRESHOLD) {
            triggerHapticPulse(e.clientX, e.clientY);
            accumulatedScroll = 0;
        }
    }, { passive: false });

    // 4. Keyboard Interaction
    const demoInput = document.getElementById('demoInput');
    const virtualKeys = document.querySelectorAll('.key');

    function animateKey(keyChar) {
        const matchingKey = Array.from(virtualKeys).find(k => {
            const val = k.getAttribute('data-key');
            if (val === keyChar || (val && val.toUpperCase() === keyChar.toUpperCase())) return true;
            if (keyChar === ' ' && val === ' ') return true;
            if (keyChar === 'Backspace' && val === 'Backspace') return true;
            if (keyChar === 'Enter' && val === 'Enter') return true;
            return false;
        });

        if (matchingKey) {
            matchingKey.classList.add('pressed');
            setTimeout(() => matchingKey.classList.remove('pressed'), 120);
        }
    }

    demoInput.addEventListener('keydown', (e) => {
        const keyType = e.key === ' ' ? 'space' : (e.key === 'Enter' || e.key === 'Backspace') ? 'enter' : 'normal';
        playMechanicalKeySound(keyType);
        animateKey(e.key);
    });

    virtualKeys.forEach(k => {
        k.addEventListener('click', () => {
            const keyVal = k.getAttribute('data-key');
            const keyType = keyVal === ' ' ? 'space' : (keyVal === 'Enter' || keyVal === 'Backspace') ? 'enter' : 'normal';
            playMechanicalKeySound(keyType);
            animateKey(keyVal);

            if (keyVal === 'Backspace') {
                demoInput.value = demoInput.value.slice(0, -1);
            } else if (keyVal === 'Enter') {
                demoInput.value += '\n';
            } else if (keyVal === 'Shift') {
                // Do nothing
            } else {
                demoInput.value += keyVal;
            }
            demoInput.focus();
        });
    });
});
