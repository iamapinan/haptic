# ⚡️ Haptic Touch & Scroll for macOS (Rust)

แอปพลิเคชันขนาดเล็กและเบามาก (~340 KB) สำหรับ macOS ที่พัฒนาด้วยภาษา **Rust** เพื่อจำลองการส่ง **Haptic Feedback (Taptic Engine / Force Touch)** บน MacBook Trackpad หรือ Magic Trackpad เมื่อคุณ:
1. **เลื่อนเมาส์ (Mouse Movement)** — รู้สึกถึงแรงสั่น/คลิกเบาๆ เมื่อเคอร์เซอร์เคลื่อนที่ผ่านระยะที่กำหนด
2. **เลื่อนหน้าจอ (Scroll Wheel)** — รู้สึกถึงแรงต้าน/จังหวะกึกๆ (Tactile Notches) เหมือนกำลังหมุนล้อเมาส์จริง

---

## ✨ ฟีเจอร์หลัก (Features)

- **⚡️ Native Menu Bar Item (Status Bar)**: ไอคอน `⚡️` บนแถบเมนูด้านบนขวา ไม่เกะกะ Dock
- **🎯 Force Touch Haptic**: ใช้งาน Apple AppKit `NSHapticFeedbackManager` แบบ Real-time
- **🕹 แยกการเปิด/ปิดได้อิสระ**:
  - เปิด/ปิด Haptics ทั้งหมด (`Enable Haptics`)
  - เปิด/ปิด เฉพาะ Mouse Movement (`Mouse Movement Haptic`)
  - เปิด/ปิด เฉพาะ Scroll Wheel (`Scroll Wheel Haptic`)
- **🎚 เลือกระดับความแรงของการสั่น (Haptic Patterns)**:
  - `Generic (Light)` — สั่นคลิกเบาๆ นุ่มมือ
  - `Alignment (Medium)` — สั่นกึกปานกลาง (Snap notch)
  - `Level Change (Firm)` — สั่นหนักแน่น ชัดเจน
- **⚙️ ปรับระดับความไว (Sensitivity)**:
  - **Mouse Sensitivity**: High (25px), Medium (50px), Low (100px)
  - **Scroll Sensitivity**: High (Sensitive), Medium (Normal), Low (Coarse)
- **🧪 ปุ่มทดสอบ Haptic**: ทดสอบคลิกแรงสั่นได้ทันทีจากเมนู
- **🔒 Accessibility Helper**: ตรวจสอบและนำทางไปเปิดสิทธิ์ Accessibility ใน macOS System Settings ได้ด้วยคลิกเดียว

---

## 🚀 วิธีเปิดใช้งาน (Getting Started)

### วิธีที่ 1: รันผ่าน Cargo
```bash
cargo run --release
```

### วิธีที่ 2: รันผ่าน macOS `.app` Bundle
สร้างและเปิดแอป:
```bash
./bundle.sh
open Haptic.app
```

---

## 🔐 การตั้งค่าสิทธิ์ Accessibility (จำเป็นสำหรับ macOS)

เนื่องจากแอปต้องตรวจจับการเลื่อนเมาส์และการ Scroll จากทุกหน้าต่าง (Global Event Tap) บน macOS:
1. เมื่อเปิดแอปครั้งแรก macOS จะแสดงหน้าต่างแจ้งเตือนขอสิทธิ์ Accessibility
2. ไปที่ **System Settings (การตั้งค่าระบบ) ➜ Privacy & Security (ความเป็นส่วนตัวและความปลอดภัย) ➜ Accessibility (การช่วยการเข้าถึง)**
3. เปิดสวิตช์อนุญาตให้กับ `Haptic` หรือ Terminal / Cargo binary

---

## 🛠 โครงสร้างโปรเจกต์ (Architecture)

- [`src/main.rs`](file:///Users/apinan/Developments/haptic/src/main.rs) — Entry point, เริ่มต้น NSApplication แบบ Accessory (Menu bar only)
- [`src/haptic.rs`](file:///Users/apinan/Developments/haptic/src/haptic.rs) — Objective-C Binding กับ `NSHapticFeedbackManager`
- [`src/event_tap.rs`](file:///Users/apinan/Developments/haptic/src/event_tap.rs) — Background Thread ดักจับ Mouse & Scroll Events พร้อมระบบ Distance Accumulator & Throttling
- [`src/config.rs`](file:///Users/apinan/Developments/haptic/src/config.rs) — Thread-safe Atomic Configuration
- [`src/menu.rs`](file:///Users/apinan/Developments/haptic/src/menu.rs) — Native NSStatusItem & NSMenu Handler
- [`Info.plist`](file:///Users/apinan/Developments/haptic/Info.plist) & [`bundle.sh`](file:///Users/apinan/Developments/haptic/bundle.sh) — สคริปต์แพ็กเกจเป็น `.app`
