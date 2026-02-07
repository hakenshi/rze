//! Role assignment (bg/fg/cursor) with contrast guardrails.

use crate::color::{contrast_ratio, Rgb8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Roles {
    pub bg: Rgb8,
    pub fg: Rgb8,
    pub cursor: Rgb8,
}

pub fn assign_roles(colors: &[Rgb8]) -> Roles {
    if colors.is_empty() {
        return Roles {
            bg: Rgb8::new(0, 0, 0),
            fg: Rgb8::new(255, 255, 255),
            cursor: Rgb8::new(255, 255, 255),
        };
    }

    let mut sorted: Vec<Rgb8> = colors.to_vec();
    sorted.sort_by(|a, b| a.rel_luminance().total_cmp(&b.rel_luminance()));
    let bg = sorted[0];
    let fg = sorted[sorted.len() - 1];

    let mut cursor = fg;
    let mut best = 0.0;
    for &c in &sorted {
        let cr = contrast_ratio(bg, c);
        if cr > best {
            best = cr;
            cursor = c;
        }
    }

    // Ensure readable fg (prefer >= 4.5:1).
    let mut fg2 = fg;
    if contrast_ratio(bg, fg2) < 4.5 {
        // Pick best contrast.
        let mut best = 0.0;
        for &c in &sorted {
            let cr = contrast_ratio(bg, c);
            if cr > best {
                best = cr;
                fg2 = c;
            }
        }
    }

    Roles {
        bg,
        fg: fg2,
        cursor,
    }
}
