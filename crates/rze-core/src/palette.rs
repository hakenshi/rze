//! Palette model (16 colors) and helpers.

use crate::color::{contrast_ratio, Rgb8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette16 {
    pub colors: [Rgb8; 16],
    pub bg: Rgb8,
    pub fg: Rgb8,
    pub cursor: Rgb8,
}

impl Palette16 {
    pub fn from_quantized(mut colors: [Rgb8; 16]) -> Self {
        // Deterministic ordering: by luminance.
        colors.sort_by(|a, b| a.rel_luminance().total_cmp(&b.rel_luminance()));
        let bg = colors[0];
        let fg = colors[15];

        // Cursor: choose the color with best contrast against bg.
        let mut cursor = fg;
        let mut best = 0.0;
        for &c in &colors {
            let cr = contrast_ratio(bg, c);
            if cr > best {
                best = cr;
                cursor = c;
            }
        }

        Self {
            colors,
            bg,
            fg,
            cursor,
        }
    }
}
