//! Deterministic median-cut quantizer.

use std::collections::HashMap;

use crate::color::Rgb8;

#[derive(Debug, Clone)]
struct ColorCount {
    c: Rgb8,
    n: u32,
}

#[derive(Debug, Clone)]
struct Bucket {
    colors: Vec<ColorCount>,
    total: u32,
}

pub fn quantize_16(pixels: &[Rgb8]) -> [Rgb8; 16] {
    let mut v = quantize(pixels, 16);
    // Ensure exact length.
    while v.len() < 16 {
        v.push(*v.last().unwrap_or(&Rgb8::new(0, 0, 0)));
    }
    let mut out = [Rgb8::new(0, 0, 0); 16];
    out.copy_from_slice(&v[..16]);
    out
}

pub fn quantize(pixels: &[Rgb8], k: usize) -> Vec<Rgb8> {
    assert!(k > 0);
    if pixels.is_empty() {
        return vec![Rgb8::new(0, 0, 0); k];
    }

    let mut counts: HashMap<Rgb8, u32> = HashMap::new();
    for &p in pixels {
        *counts.entry(p).or_insert(0) += 1;
    }

    let mut uniq: Vec<ColorCount> = counts
        .into_iter()
        .map(|(c, n)| ColorCount { c, n })
        .collect();

    // Deterministic ordering.
    uniq.sort_by_key(|cc| (cc.c.r, cc.c.g, cc.c.b));

    let total: u32 = uniq.iter().map(|cc| cc.n).sum();
    let mut buckets = vec![Bucket {
        colors: uniq,
        total,
    }];

    while buckets.len() < k {
        // Pick the bucket with the largest range.
        let (idx, axis) = match buckets
            .iter()
            .enumerate()
            .map(|(i, b)| (i, bucket_range(b)))
            .max_by(|a, b| a.1 .0.cmp(&b.1 .0).then_with(|| a.0.cmp(&b.0)))
        {
            Some((i, (_r, axis))) => (i, axis),
            None => break,
        };

        if buckets[idx].colors.len() <= 1 {
            break;
        }

        let b = buckets.swap_remove(idx);
        let (left, right) = split_bucket(b, axis);
        buckets.push(left);
        buckets.push(right);
    }

    // Representative colors: weighted average of each bucket.
    let mut reps: Vec<Rgb8> = buckets.iter().map(weighted_average).collect();

    // Deterministic output order: by luminance then RGB.
    reps.sort_by(|a, b| {
        a.rel_luminance()
            .total_cmp(&b.rel_luminance())
            .then_with(|| (a.r, a.g, a.b).cmp(&(b.r, b.g, b.b)))
    });

    reps.truncate(k);
    reps
}

fn bucket_range(b: &Bucket) -> (u32, Axis) {
    let mut rmin = 255u8;
    let mut rmax = 0u8;
    let mut gmin = 255u8;
    let mut gmax = 0u8;
    let mut bmin = 255u8;
    let mut bmax = 0u8;
    for cc in &b.colors {
        let c = cc.c;
        rmin = rmin.min(c.r);
        rmax = rmax.max(c.r);
        gmin = gmin.min(c.g);
        gmax = gmax.max(c.g);
        bmin = bmin.min(c.b);
        bmax = bmax.max(c.b);
    }
    let rr = (rmax - rmin) as u32;
    let gr = (gmax - gmin) as u32;
    let br = (bmax - bmin) as u32;

    if rr >= gr && rr >= br {
        (rr, Axis::R)
    } else if gr >= rr && gr >= br {
        (gr, Axis::G)
    } else {
        (br, Axis::B)
    }
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    R,
    G,
    B,
}

fn split_bucket(mut b: Bucket, axis: Axis) -> (Bucket, Bucket) {
    b.colors.sort_by_key(|cc| match axis {
        Axis::R => cc.c.r,
        Axis::G => cc.c.g,
        Axis::B => cc.c.b,
    });

    let half = b.total / 2;
    let mut acc = 0u32;
    let mut split = 0usize;
    for (i, cc) in b.colors.iter().enumerate() {
        acc += cc.n;
        if acc >= half {
            split = i + 1;
            break;
        }
    }
    split = split.clamp(1, b.colors.len() - 1);

    let right_colors = b.colors.split_off(split);
    let left_colors = b.colors;

    let left_total: u32 = left_colors.iter().map(|cc| cc.n).sum();
    let right_total: u32 = right_colors.iter().map(|cc| cc.n).sum();

    (
        Bucket {
            colors: left_colors,
            total: left_total,
        },
        Bucket {
            colors: right_colors,
            total: right_total,
        },
    )
}

fn weighted_average(b: &Bucket) -> Rgb8 {
    if b.total == 0 {
        return Rgb8::new(0, 0, 0);
    }
    let mut r = 0u64;
    let mut g = 0u64;
    let mut bl = 0u64;
    for cc in &b.colors {
        r += cc.c.r as u64 * cc.n as u64;
        g += cc.c.g as u64 * cc.n as u64;
        bl += cc.c.b as u64 * cc.n as u64;
    }
    let t = b.total as u64;
    Rgb8::new((r / t) as u8, (g / t) as u8, (bl / t) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantize_is_deterministic() {
        let px = vec![
            Rgb8::new(10, 20, 30),
            Rgb8::new(10, 20, 30),
            Rgb8::new(200, 210, 220),
            Rgb8::new(201, 211, 221),
            Rgb8::new(50, 60, 70),
        ];
        let a = quantize(&px, 4);
        let b = quantize(&px, 4);
        assert_eq!(a, b);
    }

    #[test]
    fn quantize_empty_returns_black() {
        let v = quantize(&[], 3);
        assert_eq!(v, vec![Rgb8::new(0, 0, 0); 3]);
    }
}
