# Haptic for macOS

เพิ่มสัมผัส Haptic feedback (แรงสั่น/คลิกเบาๆ) ให้กับ Trackpad และ **เสียงพิมพ์ Mechanical Keyboard** บน Mac เวลาเลื่อนเมาส์, scroll, ทำ multi-touch gesture หรือพิมพ์งาน

เขียนด้วย **Rust** แบบ lightweight ทำงานเงียบๆ อยู่บน Menu Bar ไม่กินแรม ไม่หน่วงเครื่อง

---

## ทำไมถึงทำตัวนี้?

เวลาใช้ MacBook หรือ Magic Trackpad เรามักจะ scroll หน้ายาวๆ แบบลื่นไหล แต่บางทีก็คิดถึงความรู้สึกตอนหมุน scroll wheel กึกๆ บนเมาส์จริง หรือคิดถึงเสียงพิมพ์คีย์บอร์ด mechanical แบบ thocky แน่นๆ หรือเวลาใช้เมาส์ธรรมดากับ Mac mini / Mac Studio แล้วอยากได้เสียงจำลอง click/tick เหมือนล้อหมุนจริง แอปตัวนี้รวมฟังก์ชันเหล่านี้ไว้ให้ครบในที่เดียว

## ความสามารถ

- **Mouse Movement Haptic** — สั่นเบาๆ ตามระยะทางที่เมาส์เคลื่อนที่
- **Scroll Wheel Haptic** — สั่นเป็นจังหวะตามความเร็วและระยะการ scroll
- **Multi-Touch Gestures** — สั่นตอบสนองเวลา Pinch-to-zoom (ซูมภาพ), หมุน 2 นิ้ว (Rotate) หรือปัดหน้าจอ (Swipe)
- **Haptic Output Mode (เลือกโหมดส่งสัญญาณสัมผัส)**:
  - **Trackpad Vibration Only (Default)**: สั่น Taptic Engine ของ Trackpad
  - **Speaker Audio Tick**: จำลองเสียงติ๊กเบาๆ สไตล์ Apple Watch / Digital Crown ผ่านลำโพงแทน (เหมาะมากเวลาใช้เมาส์ธรรมดาหรือไม่มี Trackpad)
  - **Both (Trackpad + Speaker)**: ทั้งสั่นและมีเสียงติ๊กเบาๆ ควบคู่กัน
- **Mechanical Keyboard Sounds** — มีเสียงคลิกแป้นพิมพ์ mechanical ขณะพิมพ์แบบ low-latency (<2ms):
  - **Cream / Holy Panda (Thocky)**: เสียงลึก ทุ้ม นุ่ม แน่น
  - **Blue Switch (Crisp Click)**: เสียงกริ๊กใส คม ชัดเจน
  - **Vintage Typewriter**: เสียงแป้นพิมพ์ดีดโบราณ
  - ปรับระดับความดังได้ 5 ระดับ: 100%, 70%, 40%, 15%, Mute
- **Menu Bar Controls** — เปิด/ปิด แยกแต่ละฟังก์ชันได้อิสระจากเมนูด้านบนขวา
- **ปรับแรงสั่นได้ 3 ระดับ**: Light (สั่นเบา), Medium (ปานกลาง), Firm (หนักแน่น)
- **ปรับความไว (Sensitivity)**: เลือกระยะ threshold ได้ว่าอยากให้สั่นถี่หรือห่างแค่ไหน
- **รองรับทั้งระบบ** — ทำงานได้กับทุกแอป ไม่ว่าจะอยู่หน้าต่างไหน

## วิธีติดตั้งและใช้งาน

1. โหลดไฟล์ `Haptic.dmg` จากหน้า [Releases](https://github.com/iamapinan/haptic/releases)
2. ลาก `Haptic.app` เข้าโฟลเดอร์ Applications
3. เปิดแอป จะมีไอคอนสายฟ้า `⚡️` ขึ้นที่ Menu Bar ด้านบนขวา

> **หมายเหตุสำหรับการเปิดครั้งแรก**:  
> macOS จะขอสิทธิ์ **Accessibility (การช่วยการเข้าถึง)** เพื่อให้แอปดักจับตำแหน่งเมาส์, gesture และการพิมพ์ได้ ให้เข้าไปติ๊กเปิดอนุญาตที่:  
> `System Settings` ➜ `Privacy & Security` ➜ `Accessibility` ➜ เปิดสวิตช์ให้ **Haptic**

---

## Build เองจาก Source Code

ถ้าอยากคอมไพล์เอง ต้องลง Rust ไว้ในเครื่องก่อน:

```bash
git clone git@github.com:iamapinan/haptic.git
cd haptic

# รันตรงๆ
cargo run --release

# หรือจะแพ็กเป็น .app / .dmg
./bundle.sh
./create_dmg.sh
```

## Requirements

- macOS 11.0 ขึ้นไป
- ใช้งานได้กับทั้ง MacBook Force Touch Trackpad, Apple Magic Trackpad หรือเมาส์ทั่วไป (ผ่านโหมด Speaker Audio Tick)

## License

MIT License
