//! Color value type and formatting/transforms.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb8 {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_f32(self) -> (f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }

    pub fn rel_luminance(self) -> f32 {
        // WCAG relative luminance in linear sRGB.
        let (r, g, b) = self.to_f32();
        let r = srgb_to_linear(r);
        let g = srgb_to_linear(g);
        let b = srgb_to_linear(b);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    pub fn to_hsl(self) -> Hsl {
        // Standard RGB -> HSL in sRGB space.
        let (r, g, b) = self.to_f32();
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let l = (max + min) * 0.5;
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let mut h = h;
        if h < 0.0 {
            h += 360.0;
        }

        Hsl { h, s, l }
    }

    pub fn hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

pub fn contrast_ratio(a: Rgb8, b: Rgb8) -> f32 {
    let la = a.rel_luminance();
    let lb = b.rel_luminance();
    let (hi, lo) = if la >= lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

pub fn srgb_to_linear(x: f32) -> f32 {
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}

pub fn linear_to_srgb(x: f32) -> f32 {
    if x <= 0.0031308 {
        12.92 * x
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}

pub fn lerp_rgb_linear(a: Rgb8, b: Rgb8, t: f32) -> Rgb8 {
    let (ar, ag, ab) = a.to_f32();
    let (br, bg, bb) = b.to_f32();
    let ar = srgb_to_linear(ar);
    let ag = srgb_to_linear(ag);
    let ab = srgb_to_linear(ab);
    let br = srgb_to_linear(br);
    let bg = srgb_to_linear(bg);
    let bb = srgb_to_linear(bb);

    let r = linear_to_srgb(ar + (br - ar) * t).clamp(0.0, 1.0);
    let g = linear_to_srgb(ag + (bg - ag) * t).clamp(0.0, 1.0);
    let b = linear_to_srgb(ab + (bb - ab) * t).clamp(0.0, 1.0);
    Rgb8::new(
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luminance_black_white() {
        assert!(Rgb8::new(0, 0, 0).rel_luminance() < 0.001);
        assert!(Rgb8::new(255, 255, 255).rel_luminance() > 0.99);
    }

    #[test]
    fn contrast_is_symmetric() {
        let a = Rgb8::new(10, 20, 30);
        let b = Rgb8::new(240, 240, 240);
        let ab = contrast_ratio(a, b);
        let ba = contrast_ratio(b, a);
        assert!((ab - ba).abs() < 1e-6);
    }
}
