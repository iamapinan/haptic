# Haptic for macOS

เพิ่มสัมผัส Haptic feedback (แรงสั่น/คลิกเบาๆ) ให้กับ Trackpad บน Mac เวลาเลื่อนเมาส์หรือ scroll หน้าจอ 

เขียนด้วย **Rust** แบบ lightweight ทำงานเงียบๆ อยู่บน Menu Bar ไม่กินแรม ไม่หน่วงเครื่อง

---

## ทำไมถึงทำตัวนี้?

เวลาใช้ Trackpad บน MacBook หรือ Magic Trackpad เรามักจะ scroll หน้ายาวๆ แบบลื่นไหล แต่บางทีก็คิดถึงความรู้สึกตอนหมุน scroll wheel กึกๆ บนเมาส์จริง หรืออยากได้สัมผัส tactile เวลาเคอร์เซอร์ขยับผ่านระยะต่างๆ แอปตัวนี้จะยิงสัญญาณตรงไปที่ Taptic Engine ของ Trackpad เพื่อจำลองจังหวะนั้นขึ้นมา

## ความสามารถ

- **Mouse Movement Haptic** — สั่นเบาๆ ตามระยะทางที่เมาส์เคลื่อนที่
- **Scroll Wheel Haptic** — สั่นเป็นจังหวะตามความเร็วและระยะการ scroll
- **Menu Bar Controls** — เปิด/ปิด แยกแต่ละฟังก์ชันได้อิสระจากเมนูด้านบนขวา
- **ปรับแรงสั่นได้ 3 ระดับ**: Light (สั่นเบา), Medium (ปานกลาง), Firm (หนักแน่น)
- **ปรับความไว (Sensitivity)**: เลือกระยะ threshold ได้ว่าอยากให้สั่นถี่หรือห่างแค่ไหน
- **รองรับทั้งระบบ** — ทำงานได้กับทุกแอป ไม่ว่าจะอยู่หน้าต่างไหน

## วิธีติดตั้งและใช้งาน

1. โหลดไฟล์ `Haptic.dmg` จากหน้า [Releases](https://github.com/iamapinan/haptic/releases)
2. ลาก `Haptic.app` เข้าโฟลเดอร์ Applications
3. เปิดแอป จะมีไอคอนสายฟ้า `⚡️` ขึ้นที่ Menu Bar ด้านบนขวา

> **หมายเหตุสำหรับการเปิดครั้งแรก**:  
> macOS จะขอสิทธิ์ **Accessibility (การช่วยการเข้าถึง)** เพื่อให้แอปดักจับตำแหน่งเมาส์และการ scroll ได้ ให้เข้าไปติ๊กเปิดอนุญาตที่:  
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
- MacBook ที่มี Force Touch Trackpad หรือใช้งานร่วมกับ Apple Magic Trackpad

## License

MIT License
