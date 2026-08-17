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
        input_placeholder: "Type here to play musical keyboard notes (e.g. Hello Haptic)..."
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
        input_placeholder: "พิมพ์ข้อความทดสอบเสียงดนตรีที่นี่ (เช่น สวัสดี Haptic)..."
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

function getAudioContext() {
    if (!audioCtx) {
        audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    }
    if (audioCtx.state === 'suspended') {
        audioCtx.resume();
    }
    return audioCtx;
}

let currentProfile = 'piano';
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

        if (currentProfile === 'piano') {
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

        } else if (currentProfile === 'drum') {
            // Calibrated Acoustic Drum Kit Web Audio Synthesizer (Reference Standard Frequencies)
            const char = (keyVal || 'a').toLowerCase();
            const kCode = code || '';

            // 1. Bass Drum / Kick – Exactly 114 Hz (Spacebar, Enter, B, V, N, C, Z, X)
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

                // Beater click
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

            // 2. Snare Drum – Exactly 218 Hz (J, F, D, K, S, L, A, ;, ')
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

                // Snare wire rattle
                const rattle = ctx.createOscillator();
                const rattleGain = ctx.createGain();
                rattle.type = 'sawtooth';
                rattle.frequency.setValueAtTime(3400, now);
                rattleGain.gain.setValueAtTime(0.65, now);
                rattleGain.gain.exponentialRampToValueAtTime(0.001, now + 0.140);
                rattle.connect(rattleGain);
                rattleGain.connect(ctx.destination);
                rattle.start(now);
                rattle.stop(now + 0.145);

            // 3. Tom 1 (High Tom) – Exactly 150 Hz (4, Q, E)
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

            // 4. Tom 2 (Mid Tom) – Exactly 128 Hz (5, 6, W, R)
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

            // 5. Tom 3 (Low Tom) – Exactly 87 Hz (7, 8, U, I)
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

            // 6. Floor Tom 4 – Exactly 65 Hz (1, 2, 3, 9, 0, -, =)
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

            // 7. Closed Hi-Hat (H, G, O, P)
            } else if (['h', 'g', 'o', 'p'].includes(char)) {
                const hat = ctx.createOscillator();
                const hatGain = ctx.createGain();
                hat.type = 'sawtooth';
                hat.frequency.setValueAtTime(8400, now);
                hatGain.gain.setValueAtTime(0.65, now);
                hatGain.gain.exponentialRampToValueAtTime(0.001, now + 0.055);
                hat.connect(hatGain);
                hatGain.connect(ctx.destination);
                hat.start(now);
                hat.stop(now + 0.060);

            // 8. Open Hi-Hat (T, Y)
            } else if (['t', 'y'].includes(char)) {
                const openHat = ctx.createOscillator();
                const openHatGain = ctx.createGain();
                openHat.type = 'sawtooth';
                openHat.frequency.setValueAtTime(7800, now);
                openHatGain.gain.setValueAtTime(0.65, now);
                openHatGain.exponentialRampToValueAtTime(0.001, now + 0.200);
                openHat.connect(openHatGain);
                openHatGain.connect(ctx.destination);
                openHat.start(now);
                openHat.stop(now + 0.205);

            // 9. Crash Cymbal (Accents: Tab, Escape, [, ])
            } else if (['tab', 'escape', '[', ']'].includes(char) || kCode === 'Tab' || kCode === 'Escape') {
                const crash = ctx.createOscillator();
                const crashGain = ctx.createGain();
                crash.type = 'sawtooth';
                crash.frequency.setValueAtTime(5800, now);
                crashGain.gain.setValueAtTime(0.75, now);
                crashGain.gain.exponentialRampToValueAtTime(0.001, now + 0.420);
                crash.connect(crashGain);
                crashGain.connect(ctx.destination);
                crash.start(now);
                crash.stop(now + 0.425);

            // 10. Ride Cymbal (Rhythm Timekeeping & Bell: Backspace, \, Arrow Keys)
            } else if (['backspace', '\\'].includes(char) || kCode === 'Backspace' || kCode.startsWith('Arrow')) {
                const bell = ctx.createOscillator();
                const bellGain = ctx.createGain();
                bell.type = 'sine';
                bell.frequency.setValueAtTime(680, now);
                bellGain.gain.setValueAtTime(0.50, now);
                bellGain.gain.exponentialRampToValueAtTime(0.001, now + 0.150);
                bell.connect(bellGain);
                bellGain.connect(ctx.destination);
                bell.start(now);
                bell.stop(now + 0.155);

                const ride = ctx.createOscillator();
                const rideGain = ctx.createGain();
                ride.type = 'sawtooth';
                ride.frequency.setValueAtTime(4200, now);
                rideGain.gain.setValueAtTime(0.40, now);
                rideGain.gain.exponentialRampToValueAtTime(0.001, now + 0.450);
                ride.connect(rideGain);
                rideGain.connect(ctx.destination);
                ride.start(now);
                ride.stop(now + 0.455);

            // 11. Cowbell & Extra Percussion
            } else {
                const cow = ctx.createOscillator();
                const cowGain = ctx.createGain();
                cow.type = 'triangle';
                cow.frequency.setValueAtTime(560, now);
                cowGain.gain.setValueAtTime(0.70, now);
                cowGain.gain.exponentialRampToValueAtTime(0.001, now + 0.150);
                cow.connect(cowGain);
                cowGain.connect(ctx.destination);
                cow.start(now);
                cow.stop(now + 0.155);
            }

        } else if (currentProfile === 'marimba' || currentProfile === 'thock') {
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
            osc.stop(now + 0.09);

            // 2nd Harmonic
            const osc2 = ctx.createOscillator();
            const gain2 = ctx.createGain();
            osc2.type = 'sine';
            osc2.frequency.setValueAtTime(freq * 2.0, now);
            gain2.gain.setValueAtTime(0.35, now);
            gain2.gain.exponentialRampToValueAtTime(0.001, now + 0.06);
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

        } else if (currentProfile === 'blue') {
            // Clicky Blue Switch + melodic chime
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();

            osc.type = 'sine';
            osc.frequency.setValueAtTime(freq, now);
            gain.gain.setValueAtTime(0.55, now);
            gain.gain.exponentialRampToValueAtTime(0.001, now + 0.05);
            osc.connect(gain);
            gain.connect(ctx.destination);
            osc.start(now);
            osc.stop(now + 0.055);

            // High mechanical click
            const snap = ctx.createOscillator();
            const snapGain = ctx.createGain();
            snap.type = 'triangle';
            snap.frequency.setValueAtTime(4200, now);
            snapGain.gain.setValueAtTime(0.50, now);
            snapGain.gain.exponentialRampToValueAtTime(0.001, now + 0.015);
            snap.connect(snapGain);
            snapGain.connect(ctx.destination);
            snap.start(now);
            snap.stop(now + 0.02);

        } else if (currentProfile === 'typewriter') {
            // Vintage Typewriter + tuned bell ring
            const strike = ctx.createOscillator();
            const strikeGain = ctx.createGain();
            strike.type = 'triangle';
            strike.frequency.setValueAtTime(420, now);
            strike.frequency.exponentialRampToValueAtTime(180, now + 0.02);
            strikeGain.gain.setValueAtTime(0.55, now);
            strikeGain.gain.exponentialRampToValueAtTime(0.001, now + 0.04);
            strike.connect(strikeGain);
            strikeGain.connect(ctx.destination);
            strike.start(now);
            strike.stop(now + 0.045);

            // Tuned bell ring
            const ring = ctx.createOscillator();
            const ringGain = ctx.createGain();
            ring.type = 'sine';
            ring.frequency.setValueAtTime(freq, now);
            ringGain.gain.setValueAtTime(0.45, now);
            ringGain.gain.exponentialRampToValueAtTime(0.001, now + 0.075);
            ring.connect(ringGain);
            ringGain.connect(ctx.destination);
            ring.start(now);
            ring.stop(now + 0.08);
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
            playMechanicalKeySound('enter');
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

    demoInput.addEventListener('keydown', (e) => {
        playMechanicalKeySound(e.key, e.code);
        animateKey(e.key);
    });

    virtualKeys.forEach(k => {
        k.addEventListener('click', () => {
            const keyVal = k.getAttribute('data-key');
            playMechanicalKeySound(keyVal, '');
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
