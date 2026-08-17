// i18n Translations Dictionary
const translations = {
    en: {
        nav_features: "Features",
        nav_demo: "Interactive Demo",
        nav_install: "Install",
        hero_badge: "Pure Rust Native macOS Menu Bar Utility",
        hero_title: `Tactile Haptics & <br><span class="gradient-text">Mechanical Keyboard</span> on Mac`,
        hero_desc: "Adds tactile haptic feedback to your Trackpad as you move and scroll, with realistic mechanical switch click sounds as you type. Lightweight, quiet in your menu bar, zero memory overhead.",
        btn_download: "Download Haptic.dmg",
        btn_demo: "Try Live Demo",
        meta_ram: "⚡️ Lightweight < 10MB RAM",
        meta_rust: "🦀 Pure Rust & Native Cocoa",
        meta_offline: "🔒 100% Offline & Open Source",
        demo_title: "Interactive Browser Simulator",
        demo_subtitle: "Move your cursor, scroll on the virtual trackpad, or type on your physical keyboard to test the sound engine live.",
        ctrl_profile: "Switch Sound Profile:",
        ctrl_intensity: "Haptic Tick Intensity:",
        opt_light: "Light",
        opt_med: "Medium",
        opt_firm: "Firm",
        trackpad_title: "Trackpad & Scroll Test Pad",
        trackpad_desc: "Move cursor around or use mouse wheel / trackpad scroll inside this box",
        pulse_label: "Haptic Pulses:",
        kb_title: "Type on Keyboard",
        kb_badge: "Type on real keys",
        key_space: "Spacebar (Deep Thock)",
        feat_title: "Engineered for Performance",
        feat_subtitle: "Native system integration with zero lag and deeply satisfying tactile feedback.",
        f1_title: "Force Touch Trackpad Haptics",
        f1_desc: "Directly drives the hardware Taptic Engine inside MacBook and Magic Trackpads for authentic mechanical notches as you move and scroll across any application.",
        f2_title: "Speaker Audio Tick Simulation",
        f2_desc: "For regular optical mice or desktop Macs (Mac mini / Mac Studio), simulate Apple Watch-like micro-ticks through speakers.",
        f3_title: "Multi-Touch Gestures",
        f3_desc: "Haptic feedback on Pinch-to-zoom in photos/browsers, 2-finger rotation, and multi-finger swipe navigation.",
        f4_title: "Harmonic Musical Keyboard Engine (IOKit Hardware)",
        f4_desc: "Hardware-level IOHIDManager keystroke detection with ultra low-latency (<1ms) CoreAudio playback. Every key produces a distinct, melodious pitch (Major Pentatonic Scale) turning your typing into music!",
        f5_title: "Pure Rust & Native Cocoa",
        f5_desc: "Compiled to native machine code. Uses less than 10MB RAM, zero Electron bloat, and runs 100% offline with zero data tracking.",
        f6_title: "Customizable Menu Bar Controls",
        f6_desc: "Easily tune sensitivity thresholds, volume levels, vibration intensity, or toggle individual effects on and off from the status menu.",
        ins_title: "Get Started in 3 Simple Steps",
        ins_subtitle: "Quick and straightforward installation without complex dependencies.",
        step1_title: "Download DMG",
        step1_desc: "Grab the latest <code>Haptic.dmg</code> installer package directly from GitHub Releases.",
        step1_link: "Download .dmg →",
        step2_title: "Drag to Applications",
        step2_desc: "Open the DMG file and drag <code>Haptic.app</code> directly into your Applications folder.",
        step3_title: "Enable Accessibility",
        step3_desc: "Launch the app and grant Accessibility permissions in System Settings so it can monitor mouse and key events.",
        build_title: "Or build from source code with Cargo:",
        footer_desc: "Open Source Native macOS Utility written in Rust.",
        input_placeholder: "Type here to hear the mechanical switches (e.g. Hello Haptic)..."
    },
    th: {
        nav_features: "ฟังก์ชันหลัก",
        nav_demo: "ทดลองใช้งานสด",
        nav_install: "วิธีติดตั้ง",
        hero_badge: "แอปยูทิลิตี้ Menu Bar เขียนด้วย Rust สำหรับ macOS",
        hero_title: `เพิ่มสัมผัส Haptic & <br><span class="gradient-text">เสียงพิมพ์คีย์บอร์ดดนตรี</span> บน Mac`,
        hero_desc: "เพิ่มสัมผัสแรงสั่น Haptic ให้กับ Trackpad เมื่อเลื่อนเมาส์หรือ scroll และจำลองเสียงคีย์บอร์ดที่แต่ละปุ่มมีโทนเสียงดนตรีไม่ซ้ำกัน สไตล์ Pentatonic เมโลดี้เพราะๆ ขณะพิมพ์งาน ไม่กินแรม ไม่หน่วงเครื่อง",
        btn_download: "ดาวน์โหลด Haptic.dmg",
        btn_demo: "ทดลองฟังเสียงสด",
        meta_ram: "⚡️ น้ำหนักเบา กินแรม < 10MB",
        meta_rust: "🦀 Pure Rust & Native Cocoa",
        meta_offline: "🔒 ปลอดภัย ออฟไลน์ 100% โอเพนซอร์ส",
        demo_title: "ลองสัมผัสประสบการณ์สดบนเบราว์เซอร์",
        demo_subtitle: "ทดลองเลื่อนเมาส์, scroll บนแผ่น Trackpad หรือลองพิมพ์คีย์บอร์ดด้านล่างเพื่อฟังเสียงจำลองโน้ตดนตรี",
        ctrl_profile: "เลือกรูปแบบเสียงสวิตช์:",
        ctrl_intensity: "ระดับแรงสัมผัส Haptic:",
        opt_light: "เบา",
        opt_med: "ปานกลาง",
        opt_firm: "หนักแน่น",
        trackpad_title: "แผ่นทดสอบ Trackpad & Scroll",
        trackpad_desc: "เลื่อนเมาส์ไปมา หรือเลื่อนลูกกลิ้ง Scroll Wheel ภายในกรอบนี้",
        pulse_label: "จังหวะสัมผัส (Pulses):",
        kb_title: "ทดลองพิมพ์คีย์บอร์ดดนตรี",
        kb_badge: "กดแป้นพิมพ์จริงได้เลย",
        key_space: "Spacebar (Deep Sub Bass ทุ้มแน่น)",
        feat_title: "ฟังก์ชันและคุณสมบัติครบครัน",
        feat_subtitle: "ออกแบบมาเพื่อความลื่นไหล เป็นธรรมชาติ และตอบสนองรวดเร็วระดับฮาร์ดแวร์",
        f1_title: "Force Touch Trackpad Haptics",
        f1_desc: "เชื่อมต่อไปยังฮาร์ดแวร์ Taptic Engine ของ MacBook และ Magic Trackpad โดยตรง มอบแรงสั่นสะเทือนที่แม่นยำขณะเคลื่อนที่หรือ scroll ผ่านทุกแอปพลิเคชัน",
        f2_title: "จำลองสัมผัสผ่านลำโพง (Speaker Tick)",
        f2_desc: "สำหรับผู้ใช้เมาส์ทั่วไป หรือ Mac mini / Mac Studio ที่ไม่มี Trackpad แอปสามารถจำลองเสียงติ๊กเบาๆ สไตล์ Apple Watch ผ่านลำโพงได้",
        f3_title: "รองรับ Multi-Touch Gestures",
        f3_desc: "สั่นตอบสนองเวลา Pinch-to-zoom (ซูมภาพ), หมุน 2 นิ้ว (Rotate) และปัด 2 นิ้ว / 3 นิ้ว นำทาง (Swipe)",
        f4_title: "เสียงพิมพ์คีย์บอร์ดดนตรี Harmonic Tone Engine",
        f4_desc: "ดักจับการกดปุ่มระดับ IOHIDManager ฮาร์ดแวร์ พร้อมสังเคราะห์เสียงดนตรี (Pentatonic Scale) แยกความถี่ตามแต่ละปุ่ม ไร้การดีเลย์ (<1ms) สลับเสียงได้ทั้ง Musical Marimba, Cream Thock, Blue Clicky และ Typewriter",
        f5_title: "Pure Rust & Native Cocoa เบาและเร็ว",
        f5_desc: "คอมไพล์เป็นเนทีฟโค้ด กินแรมน้อยกว่า 10MB ไม่มี Electron และทำงานออฟไลน์ 100% ไม่เก็บข้อมูลใดๆ",
        f6_title: "ปรับแต่งได้ง่ายบน Menu Bar",
        f6_desc: "ปรับแต่งความไว (Sensitivity), ความดังของเสียง, ระดับแรงสั่น หรือแยกเปิด/ปิดแต่ละฟังก์ชันได้อิสระจาก Menu Bar",
        ins_title: "วิธีติดตั้งใน 3 ขั้นตอนง่ายๆ",
        ins_subtitle: "ติดตั้งง่าย ใช้งานได้ทันที ไม่ต้องลงโปรแกรมเสริม",
        step1_title: "ดาวน์โหลดไฟล์ DMG",
        step1_desc: "ดาวน์โหลดไฟล์ตัวติดตั้ง <code>Haptic.dmg</code> จากหน้า GitHub Releases",
        step1_link: "ดาวน์โหลด .dmg →",
        step2_title: "ลากเข้า Applications",
        step2_desc: "เปิดไฟล์ DMG แล้วลากไอคอน <code>Haptic.app</code> ไปวางในโฟลเดอร์ Applications",
        step3_title: "อนุญาตสิทธิ์ Accessibility",
        step3_desc: "เปิดแอป แล้วเปิดสิทธิ์ใน System Settings ➜ Privacy & Security ➜ Accessibility เพื่อให้แอปตรวจจับเมาส์และการพิมพ์ได้",
        build_title: "หรือคอมไพล์เองจาก Source Code ด้วย Cargo:",
        footer_desc: "โอเพนซอร์สแอปพลิเคชันสำหรับ macOS พัฒนาด้วยภาษา Rust",
        input_placeholder: "พิมพ์ข้อความทดสอบเสียงสวิตช์ที่นี่ (เช่น สวัสดี Haptic)..."
    }
};

// Current Language
let currentLang = 'en';

function detectUserLanguage() {
    const saved = localStorage.getItem('haptic_lang');
    if (saved && (saved === 'th' || saved === 'en')) {
        return saved;
    }
    const navLang = navigator.language || navigator.userLanguage || 'en';
    if (navLang.toLowerCase().startsWith('th')) {
        return 'th';
    }
    return 'en';
}

function setLanguage(lang) {
    currentLang = lang;
    localStorage.setItem('haptic_lang', lang);
    document.documentElement.lang = lang;

    const t = translations[lang] || translations.en;

    // Update all elements with data-i18n
    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        if (t[key]) {
            el.innerHTML = t[key];
        }
    });

    // Update input placeholder
    const demoInput = document.getElementById('demoInput');
    if (demoInput && t.input_placeholder) {
        demoInput.placeholder = t.input_placeholder;
    }

    // Update switcher buttons UI
    const langTH = document.getElementById('langTH');
    const langEN = document.getElementById('langEN');
    if (lang === 'th') {
        langTH.classList.add('active');
        langEN.classList.remove('active');
    } else {
        langEN.classList.add('active');
        langTH.classList.remove('active');
    }
}

// Web Audio API Synthesizer for Haptic Ticks & Mechanical Switches
let audioCtx = null;
let noiseBuffer = null;
let noiseBufferCtx = null;

function getAudioContext() {
    if (!audioCtx) {
        audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    }
    if (audioCtx.state === 'suspended') {
        audioCtx.resume();
    }
    return audioCtx;
}

function getNoiseBuffer(ctx) {
    if (!noiseBuffer || noiseBufferCtx !== ctx) {
        const bufferSize = ctx.sampleRate * 0.5;
        const buffer = ctx.createBuffer(1, bufferSize, ctx.sampleRate);
        const output = buffer.getChannelData(0);
        for (let i = 0; i < bufferSize; i++) {
            output[i] = Math.random() * 2 - 1;
        }
        noiseBuffer = buffer;
        noiseBufferCtx = ctx;
    }
    return noiseBuffer;
}

function playNoiseBurst(ctx, now, duration, gainVal, filterFreq = 4000) {
    const src = ctx.createBufferSource();
    src.buffer = getNoiseBuffer(ctx);
    
    const filter = ctx.createBiquadFilter();
    filter.type = 'bandpass';
    filter.frequency.setValueAtTime(filterFreq, now);
    filter.Q.setValueAtTime(1.5, now);

    const gain = ctx.createGain();
    gain.gain.setValueAtTime(gainVal, now);
    gain.gain.exponentialRampToValueAtTime(0.0001, now + duration);

    src.connect(filter);
    filter.connect(gain);
    gain.connect(ctx.destination);

    src.start(now);
    src.stop(now + duration + 0.01);
}

let currentProfile = 'thock';
let currentIntensity = 'generic';
let pulseCount = 0;

function getKeyFrequency(keyVal, code) {
    if (keyVal === ' ' || code === 'Space') return 98.00;
    if (keyVal === 'Enter' || code === 'Enter') return 130.81;
    if (keyVal === 'Backspace' || code === 'Backspace') return 146.83;
    if (keyVal === 'Tab' || code === 'Tab') return 164.81;
    if (keyVal === 'Escape' || code === 'Escape') return 523.25;

    const char = (keyVal || 'a').toLowerCase();
    const rowMap = {
        'z': 196.00, 'x': 220.00, 'c': 261.63, 'v': 293.66, 'b': 329.63, 'n': 392.00, 'm': 440.00, ',': 523.25, '.': 587.33, '/': 659.25,
        'a': 261.63, 's': 293.66, 'd': 329.63, 'f': 392.00, 'g': 440.00, 'h': 523.25, 'j': 587.33, 'k': 659.25, 'l': 783.99, ';': 880.00, "'": 1046.50,
        'q': 392.00, 'w': 440.00, 'e': 523.25, 'r': 587.33, 't': 659.25, 'y': 783.99, 'u': 880.00, 'i': 1046.50, 'o': 1174.66, 'p': 1318.51, '[': 1567.98, ']': 1760.00,
        '1': 523.25, '2': 587.33, '3': 659.25, '4': 783.99, '5': 880.00, '6': 1046.50, '7': 1174.66, '8': 1318.51, '9': 1567.98, '0': 1760.00, '-': 2093.00, '=': 2349.32
    };

    if (rowMap[char]) return rowMap[char];
    const scale = [261.63, 293.66, 329.63, 392.00, 440.00, 523.25, 587.33, 659.25, 783.99, 880.00];
    const charCode = char.charCodeAt(0) || 65;
    return scale[(charCode * 7 + 3) % scale.length];
}

// Synthesize Mechanical & Melodic Sound in Web Audio API
function playMechanicalKeySound(keyVal = 'a', code = '') {
    try {
        const ctx = getAudioContext();
        const now = ctx.currentTime;
        const freq = getKeyFrequency(keyVal, code);
        const char = (keyVal || 'a').toLowerCase();
        const kCode = code || '';

        if (currentProfile === 'thock') {
            // Holy Panda / Cream switch: Deep, creamy thock with stabilized housing acoustics
            if (keyVal === ' ' || kCode === 'Space') {
                const sub = ctx.createOscillator();
                const subGain = ctx.createGain();
                sub.type = 'sine';
                sub.frequency.setValueAtTime(125, now);
                subGain.gain.setValueAtTime(0.85, now);
                subGain.gain.exponentialRampToValueAtTime(0.001, now + 0.075);
                sub.connect(subGain);
                subGain.connect(ctx.destination);
                sub.start(now);
                sub.stop(now + 0.08);

                const cavity = ctx.createOscillator();
                const cavGain = ctx.createGain();
                cavity.type = 'sine';
                cavity.frequency.setValueAtTime(220, now);
                cavGain.gain.setValueAtTime(0.45, now);
                cavGain.gain.exponentialRampToValueAtTime(0.001, now + 0.060);
                cavity.connect(cavGain);
                cavGain.connect(ctx.destination);
                cavity.start(now);
                cavity.stop(now + 0.065);

                const snap = ctx.createOscillator();
                const snapGain = ctx.createGain();
                snap.type = 'triangle';
                snap.frequency.setValueAtTime(1400, now);
                snapGain.gain.setValueAtTime(0.25, now);
                snapGain.gain.exponentialRampToValueAtTime(0.001, now + 0.015);
                snap.connect(snapGain);
                snapGain.connect(ctx.destination);
                snap.start(now);
                snap.stop(now + 0.02);

                playNoiseBurst(ctx, now, 0.020, 0.15, 2000);
            } else if (keyVal === 'Enter' || kCode === 'Enter' || keyVal === 'Backspace' || kCode === 'Backspace') {
                const thock = ctx.createOscillator();
                const thockGain = ctx.createGain();
                thock.type = 'sine';
                thock.frequency.setValueAtTime(180, now);
                thockGain.gain.setValueAtTime(0.75, now);
                thockGain.gain.exponentialRampToValueAtTime(0.001, now + 0.060);
                thock.connect(thockGain);
                thockGain.connect(ctx.destination);
                thock.start(now);
                thock.stop(now + 0.065);

                const snap = ctx.createOscillator();
                const snapGain = ctx.createGain();
                snap.type = 'triangle';
                snap.frequency.setValueAtTime(1600, now);
                snapGain.gain.setValueAtTime(0.25, now);
                snapGain.gain.exponentialRampToValueAtTime(0.001, now + 0.015);
                snap.connect(snapGain);
                snapGain.connect(ctx.destination);
                snap.start(now);
                snap.stop(now + 0.02);

                playNoiseBurst(ctx, now, 0.018, 0.18, 2400);
            } else {
                const charCode = char.charCodeAt(0) || 65;
                const basePitch = 220 + (charCode % 12) * 12;

                const thock = ctx.createOscillator();
                const thockGain = ctx.createGain();
                thock.type = 'sine';
                thock.frequency.setValueAtTime(basePitch, now);
                thockGain.gain.setValueAtTime(0.70, now);
                thockGain.gain.exponentialRampToValueAtTime(0.001, now + 0.048);
                thock.connect(thockGain);
                thockGain.connect(ctx.destination);
                thock.start(now);
                thock.stop(now + 0.052);

                const snap = ctx.createOscillator();
                const snapGain = ctx.createGain();
                snap.type = 'triangle';
                snap.frequency.setValueAtTime(1900, now);
                snapGain.gain.setValueAtTime(0.25, now);
                snapGain.gain.exponentialRampToValueAtTime(0.001, now + 0.012);
                snap.connect(snapGain);
                snapGain.connect(ctx.destination);
                snap.start(now);
                snap.stop(now + 0.015);

                playNoiseBurst(ctx, now, 0.015, 0.16, 3200);
            }

        } else if (currentProfile === 'blue') {
            // Cherry MX Blue: Crisp click-jacket snap + housing clack
            const click = ctx.createOscillator();
            const clickGain = ctx.createGain();
            click.type = 'triangle';
            click.frequency.setValueAtTime(4200, now);
            clickGain.gain.setValueAtTime(0.65, now);
            clickGain.gain.exponentialRampToValueAtTime(0.001, now + 0.015);
            click.connect(clickGain);
            clickGain.connect(ctx.destination);
            click.start(now);
            click.stop(now + 0.020);

            const clickHigh = ctx.createOscillator();
            const clickHighGain = ctx.createGain();
            clickHigh.type = 'sine';
            clickHigh.frequency.setValueAtTime(6800, now);
            clickHighGain.gain.setValueAtTime(0.30, now);
            clickHighGain.gain.exponentialRampToValueAtTime(0.001, now + 0.010);
            clickHigh.connect(clickHighGain);
            clickHighGain.connect(ctx.destination);
            clickHigh.start(now);
            clickHigh.stop(now + 0.012);

            const bodyFreq = (keyVal === ' ' || kCode === 'Space') ? 240 : (keyVal === 'Enter' || keyVal === 'Backspace' ? 380 : 560);
            const clack = ctx.createOscillator();
            const clackGain = ctx.createGain();
            clack.type = 'sine';
            clack.frequency.setValueAtTime(bodyFreq, now);
            clackGain.gain.setValueAtTime(0.40, now);
            clackGain.gain.exponentialRampToValueAtTime(0.001, now + 0.045);
            clack.connect(clackGain);
            clackGain.connect(ctx.destination);
            clack.start(now);
            clack.stop(now + 0.050);

            playNoiseBurst(ctx, now, 0.015, 0.22, 5000);

        } else if (currentProfile === 'typewriter') {
            // Vintage Typewriter: Metal typebar strike + metal ping + tuned bell
            const strikeFreq = (keyVal === ' ' || kCode === 'Space') ? 160 : 440;
            const strike = ctx.createOscillator();
            const strikeGain = ctx.createGain();
            strike.type = 'triangle';
            strike.frequency.setValueAtTime(strikeFreq, now);
            strike.frequency.exponentialRampToValueAtTime(180, now + 0.02);
            strikeGain.gain.setValueAtTime(0.60, now);
            strikeGain.gain.exponentialRampToValueAtTime(0.001, now + 0.055);
            strike.connect(strikeGain);
            strikeGain.connect(ctx.destination);
            strike.start(now);
            strike.stop(now + 0.060);

            const ping = ctx.createOscillator();
            const pingGain = ctx.createGain();
            ping.type = 'sine';
            ping.frequency.setValueAtTime(2600, now);
            pingGain.gain.setValueAtTime(0.30, now);
            pingGain.gain.exponentialRampToValueAtTime(0.001, now + 0.065);
            ping.connect(pingGain);
            pingGain.connect(ctx.destination);
            ping.start(now);
            ping.stop(now + 0.070);

            // Tuned bell harmonic
            const ring = ctx.createOscillator();
            const ringGain = ctx.createGain();
            ring.type = 'sine';
            ring.frequency.setValueAtTime(freq, now);
            ringGain.gain.setValueAtTime(0.40, now);
            ringGain.gain.exponentialRampToValueAtTime(0.001, now + 0.075);
            ring.connect(ringGain);
            ringGain.connect(ctx.destination);
            ring.start(now);
            ring.stop(now + 0.080);

            playNoiseBurst(ctx, now, 0.025, 0.25, 3000);

        } else if (currentProfile === 'red') {
            // Cherry MX Red (Linear): Smooth, muffled bottom-out thud
            const baseFreq = (keyVal === ' ' || kCode === 'Space') ? 180 : 320;
            const thud = ctx.createOscillator();
            const thudGain = ctx.createGain();
            thud.type = 'sine';
            thud.frequency.setValueAtTime(baseFreq, now);
            thudGain.gain.setValueAtTime(0.65, now);
            thudGain.gain.exponentialRampToValueAtTime(0.001, now + 0.038);
            thud.connect(thudGain);
            thudGain.connect(ctx.destination);
            thud.start(now);
            thud.stop(now + 0.042);

            const sub = ctx.createOscillator();
            const subGain = ctx.createGain();
            sub.type = 'sine';
            sub.frequency.setValueAtTime(baseFreq * 0.5, now);
            subGain.gain.setValueAtTime(0.35, now);
            subGain.gain.exponentialRampToValueAtTime(0.001, now + 0.035);
            sub.connect(subGain);
            subGain.connect(ctx.destination);
            sub.start(now);
            sub.stop(now + 0.038);

            playNoiseBurst(ctx, now, 0.012, 0.12, 1800);

        } else if (currentProfile === 'brown') {
            // Cherry MX Brown (Tactile): Subtle tactile bump clack
            const baseFreq = (keyVal === ' ' || kCode === 'Space') ? 210 : 480;
            const bump = ctx.createOscillator();
            const bumpGain = ctx.createGain();
            bump.type = 'triangle';
            bump.frequency.setValueAtTime(1200, now);
            bumpGain.gain.setValueAtTime(0.35, now);
            bumpGain.gain.exponentialRampToValueAtTime(0.001, now + 0.018);
            bump.connect(bumpGain);
            bumpGain.connect(ctx.destination);
            bump.start(now);
            bump.stop(now + 0.020);

            const clack = ctx.createOscillator();
            const clackGain = ctx.createGain();
            clack.type = 'sine';
            clack.frequency.setValueAtTime(baseFreq, now);
            clackGain.gain.setValueAtTime(0.55, now);
            clackGain.gain.exponentialRampToValueAtTime(0.001, now + 0.042);
            clack.connect(clackGain);
            clackGain.connect(ctx.destination);
            clack.start(now);
            clack.stop(now + 0.046);

            playNoiseBurst(ctx, now, 0.016, 0.18, 2800);

        } else if (currentProfile === 'piano') {
            // Concert Grand Piano: 360ms sustain with rich string chorus and soundboard body
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();
            osc.type = 'sine';
            osc.frequency.setValueAtTime(freq, now);
            gain.gain.setValueAtTime(0.70, now);
            gain.gain.exponentialRampToValueAtTime(0.001, now + 0.360);
            osc.connect(gain);
            gain.connect(ctx.destination);
            osc.start(now);
            osc.stop(now + 0.365);

            // Left Detuned String (Unison Chorus)
            const oscL = ctx.createOscillator();
            const gainL = ctx.createGain();
            oscL.type = 'sine';
            oscL.frequency.setValueAtTime(freq * 1.0015, now);
            gainL.gain.setValueAtTime(0.35, now);
            gainL.gain.exponentialRampToValueAtTime(0.001, now + 0.320);
            oscL.connect(gainL);
            gainL.connect(ctx.destination);
            oscL.start(now);
            oscL.stop(now + 0.325);

            // 2nd Harmonic (Octave Warmth)
            const osc2 = ctx.createOscillator();
            const gain2 = ctx.createGain();
            osc2.type = 'sine';
            osc2.frequency.setValueAtTime(freq * 2.0, now);
            gain2.gain.setValueAtTime(0.45, now);
            gain2.gain.exponentialRampToValueAtTime(0.001, now + 0.240);
            osc2.connect(gain2);
            gain2.connect(ctx.destination);
            osc2.start(now);
            osc2.stop(now + 0.245);

            // 3rd Harmonic (Fifth)
            const osc3 = ctx.createOscillator();
            const gain3 = ctx.createGain();
            osc3.type = 'sine';
            osc3.frequency.setValueAtTime(freq * 3.0, now);
            gain3.gain.setValueAtTime(0.28, now);
            gain3.gain.exponentialRampToValueAtTime(0.001, now + 0.160);
            osc3.connect(gain3);
            gain3.connect(ctx.destination);
            osc3.start(now);
            osc3.stop(now + 0.165);

            // Soundboard Woody Body (110Hz)
            const body = ctx.createOscillator();
            const bodyGain = ctx.createGain();
            body.type = 'sine';
            body.frequency.setValueAtTime(110, now);
            bodyGain.gain.setValueAtTime(0.25, now);
            bodyGain.gain.exponentialRampToValueAtTime(0.001, now + 0.180);
            body.connect(bodyGain);
            bodyGain.connect(ctx.destination);
            body.start(now);
            body.stop(now + 0.185);

            // Soft felt hammer attack
            const hammer = ctx.createOscillator();
            const hammerGain = ctx.createGain();
            hammer.type = 'triangle';
            hammer.frequency.setValueAtTime(Math.min(freq * 1.5, 320), now);
            hammerGain.gain.setValueAtTime(0.30, now);
            hammerGain.gain.exponentialRampToValueAtTime(0.001, now + 0.025);
            hammer.connect(hammerGain);
            hammerGain.connect(ctx.destination);
            hammer.start(now);
            hammer.stop(now + 0.030);

        } else if (currentProfile === 'marimba') {
            // Pure wooden chime / marimba note with rich harmonic overtones
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();
            osc.type = 'sine';
            osc.frequency.setValueAtTime(freq, now);
            gain.gain.setValueAtTime(0.70, now);
            gain.gain.exponentialRampToValueAtTime(0.001, now + 0.085);
            osc.connect(gain);
            gain.connect(ctx.destination);
            osc.start(now);
            osc.stop(now + 0.090);

            const osc2 = ctx.createOscillator();
            const gain2 = ctx.createGain();
            osc2.type = 'sine';
            osc2.frequency.setValueAtTime(freq * 2.0, now);
            gain2.gain.setValueAtTime(0.35, now);
            gain2.gain.exponentialRampToValueAtTime(0.001, now + 0.060);
            osc2.connect(gain2);
            gain2.connect(ctx.destination);
            osc2.start(now);
            osc2.stop(now + 0.065);

            // Subtle thock click snap
            const snap = ctx.createOscillator();
            const snapGain = ctx.createGain();
            snap.type = 'triangle';
            snap.frequency.setValueAtTime(Math.min(freq * 4.0, 8000), now);
            snapGain.gain.setValueAtTime(0.30, now);
            snapGain.gain.exponentialRampToValueAtTime(0.001, now + 0.012);
            snap.connect(snapGain);
            snapGain.connect(ctx.destination);
            snap.start(now);
            snap.stop(now + 0.015);

        } else if (currentProfile === 'drum') {
            // Acoustic Drum Kit Web Audio Synthesizer
            // 1. Kick Drum (Space, Enter, B, V, N, C, Z, X)
            if (keyVal === ' ' || kCode === 'Space' || keyVal === 'Enter' || kCode === 'Enter' || ['b', 'v', 'n', 'c', 'z', 'x'].includes(char)) {
                const kick = ctx.createOscillator();
                const kickGain = ctx.createGain();
                kick.type = 'sine';
                kick.frequency.setValueAtTime(114, now);
                kickGain.gain.setValueAtTime(0.95, now);
                kickGain.gain.exponentialRampToValueAtTime(0.001, now + 0.240);
                kick.connect(kickGain);
                kickGain.connect(ctx.destination);
                kick.start(now);
                kick.stop(now + 0.245);

                const sub = ctx.createOscillator();
                const subGain = ctx.createGain();
                sub.type = 'sine';
                sub.frequency.setValueAtTime(57, now);
                subGain.gain.setValueAtTime(0.50, now);
                subGain.gain.exponentialRampToValueAtTime(0.001, now + 0.180);
                sub.connect(subGain);
                subGain.connect(ctx.destination);
                sub.start(now);
                sub.stop(now + 0.185);

                const click = ctx.createOscillator();
                const clickGain = ctx.createGain();
                click.type = 'triangle';
                click.frequency.setValueAtTime(2200, now);
                clickGain.gain.setValueAtTime(0.30, now);
                clickGain.gain.exponentialRampToValueAtTime(0.001, now + 0.015);
                click.connect(clickGain);
                clickGain.connect(ctx.destination);
                click.start(now);
                click.stop(now + 0.020);

            // 2. Snare Drum (J, F, D, K, S, L, A, ;, ')
            } else if (['j', 'f', 'd', 'k', 's', 'l', 'a', ';', "'"].includes(char)) {
                const shell = ctx.createOscillator();
                const shellGain = ctx.createGain();
                shell.type = 'triangle';
                shell.frequency.setValueAtTime(218, now);
                shellGain.gain.setValueAtTime(0.65, now);
                shellGain.gain.exponentialRampToValueAtTime(0.001, now + 0.200);
                shell.connect(shellGain);
                shellGain.connect(ctx.destination);
                shell.start(now);
                shell.stop(now + 0.205);

                playNoiseBurst(ctx, now, 0.140, 0.65, 3400);

            // 3. Tom 1 (High Tom) (4, Q, E)
            } else if (['4', 'q', 'e'].includes(char)) {
                const tom = ctx.createOscillator();
                const tomGain = ctx.createGain();
                tom.type = 'sine';
                tom.frequency.setValueAtTime(150, now);
                tomGain.gain.setValueAtTime(0.85, now);
                tomGain.gain.exponentialRampToValueAtTime(0.001, now + 0.220);
                tom.connect(tomGain);
                tomGain.connect(ctx.destination);
                tom.start(now);
                tom.stop(now + 0.225);

                const h2 = ctx.createOscillator();
                const h2Gain = ctx.createGain();
                h2.type = 'sine';
                h2.frequency.setValueAtTime(300, now);
                h2Gain.gain.setValueAtTime(0.30, now);
                h2Gain.gain.exponentialRampToValueAtTime(0.001, now + 0.150);
                h2.connect(h2Gain);
                h2Gain.connect(ctx.destination);
                h2.start(now);
                h2.stop(now + 0.155);

            // 4. Tom 2 (Mid Tom) (5, 6, W, R)
            } else if (['5', '6', 'w', 'r'].includes(char)) {
                const tom = ctx.createOscillator();
                const tomGain = ctx.createGain();
                tom.type = 'sine';
                tom.frequency.setValueAtTime(128, now);
                tomGain.gain.setValueAtTime(0.85, now);
                tomGain.gain.exponentialRampToValueAtTime(0.001, now + 0.220);
                tom.connect(tomGain);
                tomGain.connect(ctx.destination);
                tom.start(now);
                tom.stop(now + 0.225);

                const h2 = ctx.createOscillator();
                const h2Gain = ctx.createGain();
                h2.type = 'sine';
                h2.frequency.setValueAtTime(256, now);
                h2Gain.gain.setValueAtTime(0.30, now);
                h2Gain.gain.exponentialRampToValueAtTime(0.001, now + 0.150);
                h2.connect(h2Gain);
                h2Gain.connect(ctx.destination);
                h2.start(now);
                h2.stop(now + 0.155);

            // 5. Tom 3 (Low Tom) (7, 8, U, I)
            } else if (['7', '8', 'u', 'i'].includes(char)) {
                const tom = ctx.createOscillator();
                const tomGain = ctx.createGain();
                tom.type = 'sine';
                tom.frequency.setValueAtTime(87, now);
                tomGain.gain.setValueAtTime(0.88, now);
                tomGain.gain.exponentialRampToValueAtTime(0.001, now + 0.220);
                tom.connect(tomGain);
                tomGain.connect(ctx.destination);
                tom.start(now);
                tom.stop(now + 0.225);

                const h2 = ctx.createOscillator();
                const h2Gain = ctx.createGain();
                h2.type = 'sine';
                h2.frequency.setValueAtTime(174, now);
                h2Gain.gain.setValueAtTime(0.35, now);
                h2Gain.gain.exponentialRampToValueAtTime(0.001, now + 0.160);
                h2.connect(h2Gain);
                h2Gain.connect(ctx.destination);
                h2.start(now);
                h2.stop(now + 0.165);

            // 6. Floor Tom 4 (1, 2, 3, 9, 0, -, =)
            } else if (['1', '2', '3', '9', '0', '-', '='].includes(char)) {
                const floor = ctx.createOscillator();
                const floorGain = ctx.createGain();
                floor.type = 'sine';
                floor.frequency.setValueAtTime(65, now);
                floorGain.gain.setValueAtTime(0.92, now);
                floorGain.gain.exponentialRampToValueAtTime(0.001, now + 0.280);
                floor.connect(floorGain);
                floorGain.connect(ctx.destination);
                floor.start(now);
                floor.stop(now + 0.285);

                const h2 = ctx.createOscillator();
                const h2Gain = ctx.createGain();
                h2.type = 'sine';
                h2.frequency.setValueAtTime(130, now);
                h2Gain.gain.setValueAtTime(0.38, now);
                h2Gain.gain.exponentialRampToValueAtTime(0.001, now + 0.180);
                h2.connect(h2Gain);
                h2Gain.connect(ctx.destination);
                h2.start(now);
                h2.stop(now + 0.185);

            // 7. Closed Hi-Hat (H, G)
            } else if (['h', 'g'].includes(char)) {
                playNoiseBurst(ctx, now, 0.055, 0.65, 8400);

            // 8. Open Hi-Hat (T, Y)
            } else if (['t', 'y'].includes(char)) {
                playNoiseBurst(ctx, now, 0.180, 0.65, 7800);

            // 9. Crash / Ride Cymbals (Tab, Escape, [, ], Backspace, Arrow keys, etc.)
            } else {
                const bell = ctx.createOscillator();
                const bellGain = ctx.createGain();
                bell.type = 'sine';
                bell.frequency.setValueAtTime(680, now);
                bellGain.gain.setValueAtTime(0.50, now);
                bellGain.gain.exponentialRampToValueAtTime(0.001, now + 0.120);
                bell.connect(bellGain);
                bellGain.connect(ctx.destination);
                bell.start(now);
                bell.stop(now + 0.125);

                playNoiseBurst(ctx, now, 0.350, 0.70, 5800);
            }
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

        let freq = 2600;
        let decay = 0.015;
        let volume = 0.50;

        if (currentIntensity === 'alignment') {
            freq = 1900;
            decay = 0.022;
            volume = 0.65;
        } else if (currentIntensity === 'level') {
            freq = 1300;
            decay = 0.030;
            volume = 0.80;
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
    // Audio Autoplay Unlock
    const unlockAudio = () => {
        getAudioContext();
        document.removeEventListener('click', unlockAudio);
        document.removeEventListener('keydown', unlockAudio);
        document.removeEventListener('touchstart', unlockAudio);
    };
    document.addEventListener('click', unlockAudio);
    document.addEventListener('keydown', unlockAudio);
    document.addEventListener('touchstart', unlockAudio);

    // 1. Initialize Language
    const initialLang = detectUserLanguage();
    setLanguage(initialLang);

    const langToggleBtn = document.getElementById('langToggle');
    if (langToggleBtn) {
        langToggleBtn.addEventListener('click', () => {
            const nextLang = currentLang === 'th' ? 'en' : 'th';
            setLanguage(nextLang);
        });
    }

    // 2. Profile Switcher
    const profileButtons = document.querySelectorAll('#soundProfileGroup .toggle-btn');
    profileButtons.forEach(btn => {
        btn.addEventListener('click', () => {
            profileButtons.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            currentProfile = btn.getAttribute('data-profile');
            playMechanicalKeySound('enter', 'Enter');
        });
    });

    // 3. Intensity Switcher
    const intensityButtons = document.querySelectorAll('#hapticIntensityGroup .toggle-btn');
    intensityButtons.forEach(btn => {
        btn.addEventListener('click', () => {
            intensityButtons.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            currentIntensity = btn.getAttribute('data-intensity');
            playHapticTick();
        });
    });

    // 4. Trackpad Interaction Area
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
            void visualizer.offsetWidth;
            visualizer.classList.add('pulse');
        }
    }

    if (trackpad) {
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

        trackpad.addEventListener('click', (e) => {
            triggerHapticPulse(e.clientX, e.clientY);
        });
    }

    // 5. Keyboard Interaction
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

    if (demoInput) {
        demoInput.addEventListener('keydown', (e) => {
            playMechanicalKeySound(e.key, e.code);
            animateKey(e.key);
        });
    }

    // Global Key Listener (typing anywhere on page)
    window.addEventListener('keydown', (e) => {
        if (e.target === demoInput) return;
        if (['INPUT', 'TEXTAREA', 'SELECT'].includes(e.target.tagName)) return;

        playMechanicalKeySound(e.key, e.code);
        animateKey(e.key);

        if (demoInput) {
            if (e.key === 'Backspace') {
                demoInput.value = demoInput.value.slice(0, -1);
            } else if (e.key === 'Enter') {
                demoInput.value += '\n';
            } else if (e.key.length === 1 && !e.metaKey && !e.ctrlKey && !e.altKey) {
                demoInput.value += e.key;
            }
        }
    });

    virtualKeys.forEach(k => {
        k.addEventListener('click', () => {
            const keyVal = k.getAttribute('data-key');
            playMechanicalKeySound(keyVal, keyVal === ' ' ? 'Space' : keyVal);
            animateKey(keyVal);

            if (demoInput) {
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
            }
        });
    });
});
