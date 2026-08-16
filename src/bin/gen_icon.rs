use std::fs::File;
use std::io::Write;
use std::process::Command;

// Signed distance functions for 2D graphics
fn sd_rounded_box(p: (f64, f64), b: (f64, f64), r: f64) -> f64 {
    let q = (p.0.abs() - b.0 + r, p.1.abs() - b.1 + r);
    let inside = (q.0.max(q.1)).min(0.0);
    let outside = ((q.0.max(0.0)).powi(2) + (q.1.max(0.0)).powi(2)).sqrt();
    inside + outside - r
}

fn sd_segment(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let pa = (p.0 - a.0, p.1 - a.1);
    let ba = (b.0 - a.0, b.1 - a.1);
    let h = ((pa.0 * ba.0 + pa.1 * ba.1) / (ba.0 * ba.0 + ba.1 * ba.1)).clamp(0.0, 1.0);
    let dx = pa.0 - ba.0 * h;
    let dy = pa.1 - ba.1 * h;
    (dx * dx + dy * dy).sqrt()
}

fn clamp01(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

fn main() {
    let size = 1024;
    let mut pixels = vec![0u8; size * size * 4]; // RGBA

    for y in 0..size {
        let py = 1.0 - (y as f64 + 0.5) / size as f64 * 2.0; // [-1.0, 1.0]
        for x in 0..size {
            let px = (x as f64 + 0.5) / size as f64 * 2.0 - 1.0; // [-1.0, 1.0]

            // 1. Squircle background
            // Size: half-width 0.78, radius 0.38
            let d_squircle = sd_rounded_box((px, py), (0.76, 0.76), 0.36);
            let squircle_mask = clamp01(0.5 - d_squircle * (size as f64 * 0.5));

            if squircle_mask <= 0.0 {
                // Drop shadow
                let d_shadow = sd_rounded_box((px, py + 0.08), (0.76, 0.76), 0.36);
                let shadow_alpha = (1.0 - clamp01((d_shadow - 0.01) / 0.12)) * 0.35;
                if shadow_alpha > 0.0 {
                    let idx = (y * size + x) * 4;
                    pixels[idx + 0] = 0;
                    pixels[idx + 1] = 0;
                    pixels[idx + 2] = 0;
                    pixels[idx + 3] = (shadow_alpha * 255.0) as u8;
                }
                continue;
            }

            // 2. Base Gradient (Deep Violet -> Dark Indigo)
            let grad_t = (py + 0.8) / 1.6;
            let mut r = 0.08 * (1.0 - grad_t) + 0.16 * grad_t;
            let mut g = 0.09 * (1.0 - grad_t) + 0.12 * grad_t;
            let mut b = 0.18 * (1.0 - grad_t) + 0.35 * grad_t;

            // 3. Concentric Tactile Haptic Ripples
            let dist_center = (px * px + py * py).sqrt();
            let wave_radii = [0.22, 0.34, 0.46, 0.58, 0.70];
            let wave_alphas = [0.65, 0.45, 0.32, 0.20, 0.10];

            for (&rad, &w_alpha) in wave_radii.iter().zip(wave_alphas.iter()) {
                let d_ring = (dist_center - rad).abs();
                let ring_intensity = clamp01(1.0 - d_ring / 0.015) * w_alpha;
                // Cyan/Magenta tint for rings
                r += 0.4 * ring_intensity;
                g += 0.8 * ring_intensity;
                b += 1.0 * ring_intensity;
            }

            // 4. Central Disc (Sensor Core)
            let d_disc = dist_center - 0.22;
            let disc_mask = clamp01(0.5 - d_disc * 40.0);
            if disc_mask > 0.0 {
                let disc_r = 0.2 + 0.3 * (1.0 - dist_center / 0.22);
                let disc_g = 0.3 + 0.4 * (1.0 - dist_center / 0.22);
                let disc_b = 0.8 + 0.2 * (1.0 - dist_center / 0.22);
                r = r * (1.0 - disc_mask) + disc_r * disc_mask;
                g = g * (1.0 - disc_mask) + disc_g * disc_mask;
                b = b * (1.0 - disc_mask) + disc_b * disc_mask;
            }

            // 5. Stylized Central Lightning Bolt
            // Segments: (0.05, 0.30) -> (-0.12, 0.02) -> (0.02, 0.02) -> (-0.06, -0.30) -> (0.15, -0.02) -> (-0.01, -0.02) -> close
            let d1 = sd_segment((px, py), (0.04, 0.22), (-0.10, 0.02));
            let d2 = sd_segment((px, py), (-0.10, 0.02), (0.02, 0.02));
            let d3 = sd_segment((px, py), (0.02, 0.02), (-0.05, -0.22));
            let d4 = sd_segment((px, py), (-0.05, -0.22), (0.10, -0.02));
            let d5 = sd_segment((px, py), (0.10, -0.02), (-0.02, -0.02));
            let d6 = sd_segment((px, py), (-0.02, -0.02), (0.04, 0.22));

            let min_d_bolt = d1.min(d2).min(d3).min(d4).min(d5).min(d6);

            // Glow around bolt
            let glow = clamp01(1.0 - min_d_bolt / 0.10) * 0.7;
            r += 1.0 * glow;
            g += 0.8 * glow;
            b += 0.2 * glow;

            // Fill inside bolt
            if min_d_bolt < 0.035 || (px > -0.08 && px < 0.08 && py > -0.18 && py < 0.18) {
                let bolt_fill = clamp01(1.0 - min_d_bolt / 0.04);
                r = r * (1.0 - bolt_fill) + 1.0 * bolt_fill;
                g = g * (1.0 - bolt_fill) + 0.92 * bolt_fill;
                b = b * (1.0 - bolt_fill) + 0.35 * bolt_fill;
            }

            // 6. Glass border / specular highlight
            let d_border = d_squircle.abs();
            let border_spec = clamp01(1.0 - d_border / 0.012) * (0.3 + 0.4 * clamp01(py));
            r += border_spec;
            g += border_spec;
            b += border_spec;

            let idx = (y * size + x) * 4;
            pixels[idx + 0] = (clamp01(r) * 255.0) as u8;
            pixels[idx + 1] = (clamp01(g) * 255.0) as u8;
            pixels[idx + 2] = (clamp01(b) * 255.0) as u8;
            pixels[idx + 3] = (squircle_mask * 255.0) as u8;
        }
    }

    // Write PAM format (raw RGBA) to temp file
    let pam_path = "AppIcon_master.pam";
    let mut file = File::create(pam_path).expect("failed to create pam file");
    writeln!(file, "P7").unwrap();
    writeln!(file, "WIDTH {}", size).unwrap();
    writeln!(file, "HEIGHT {}", size).unwrap();
    writeln!(file, "DEPTH 4").unwrap();
    writeln!(file, "MAXVAL 255").unwrap();
    writeln!(file, "TUPLTYPE RGB_ALPHA").unwrap();
    writeln!(file, "ENDHDR").unwrap();
    file.write_all(&pixels).unwrap();

    println!("🎨 Generated master icon buffer");

    // Convert PAM to master PNG using sips
    let _ = Command::new("sips")
        .args(&["-s", "format", "png", pam_path, "--out", "AppIcon_1024.png"])
        .status();

    // Create AppIcon.iconset folder
    let _ = std::fs::create_dir_all("AppIcon.iconset");

    let targets = [
        ("icon_16x16.png", 16),
        ("icon_16x16@2x.png", 32),
        ("icon_32x32.png", 32),
        ("icon_32x32@2x.png", 64),
        ("icon_128x128.png", 128),
        ("icon_128x128@2x.png", 256),
        ("icon_256x256.png", 256),
        ("icon_256x256@2x.png", 512),
        ("icon_512x512.png", 512),
        ("icon_512x512@2x.png", 1024),
    ];

    for (name, px) in targets {
        let out_path = format!("AppIcon.iconset/{}", name);
        let px_str = px.to_string();
        let _ = Command::new("sips")
            .args(&["-z", &px_str, &px_str, "AppIcon_1024.png", "--out", &out_path])
            .status();
    }

    // Run iconutil to produce AppIcon.icns
    let status = Command::new("iconutil")
        .args(&["-c", "icns", "AppIcon.iconset", "-o", "AppIcon.icns"])
        .status()
        .expect("failed to run iconutil");

    if status.success() {
        println!("✨ AppIcon.icns generated successfully!");
    }

    // Clean up temporary master files
    let _ = std::fs::remove_file(pam_path);
    let _ = std::fs::remove_file("AppIcon_1024.png");
}
