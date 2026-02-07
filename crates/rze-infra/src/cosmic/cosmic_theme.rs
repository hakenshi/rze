//! COSMIC theme integration.
//!
//! v0 strategy:
//! - write a theme file to `$XDG_DATA_HOME/cosmic-themes/*.ron` (so COSMIC can list it)
//! - apply immediately by writing into cosmic-config builder keys
//!   (`~/.config/cosmic/com.system76.CosmicTheme.*.Builder/v1/*`).

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};

use rze_core::color::{contrast_ratio, lerp_rgb_linear, Rgb8};
use rze_core::palette::Palette16;

use crate::atomic_write::atomic_write;

pub fn install_and_apply(theme_base_name: &str, pal: &Palette16) -> anyhow::Result<()> {
    let accent = pick_accent(pal);

    let dark = build_cosmic_palette_dark(theme_base_name, pal, accent);
    let light = build_cosmic_palette_light(theme_base_name, pal, accent);

    install_theme_files(theme_base_name, &dark, &light).context("install theme files")?;
    apply_builder(theme_base_name, &dark, &light, accent).context("apply cosmic-config builder")?;
    Ok(())
}

fn xdg_data_home() -> anyhow::Result<PathBuf> {
    if let Some(v) = std::env::var_os("XDG_DATA_HOME") {
        return Ok(PathBuf::from(v));
    }
    let home = std::env::var_os("HOME").ok_or_else(|| anyhow!("HOME is not set"))?;
    Ok(PathBuf::from(home).join(".local/share"))
}

fn xdg_config_home() -> anyhow::Result<PathBuf> {
    if let Some(v) = std::env::var_os("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(v));
    }
    let home = std::env::var_os("HOME").ok_or_else(|| anyhow!("HOME is not set"))?;
    Ok(PathBuf::from(home).join(".config"))
}

fn install_theme_files(
    theme_base_name: &str,
    dark: &CosmicPalette,
    light: &CosmicPalette,
) -> anyhow::Result<()> {
    let dir = xdg_data_home()?.join("cosmic-themes");
    let dark_path = dir.join(format!("{theme_base_name}-dark.ron"));
    let light_path = dir.join(format!("{theme_base_name}-light.ron"));

    let dark_ron = cosmic_theme_file("Dark", dark);
    let light_ron = cosmic_theme_file("Light", light);

    atomic_write(&dark_path, dark_ron.as_bytes())
        .with_context(|| format!("write {}", dark_path.display()))?;
    atomic_write(&light_path, light_ron.as_bytes())
        .with_context(|| format!("write {}", light_path.display()))?;
    Ok(())
}

fn apply_builder(
    theme_base_name: &str,
    dark: &CosmicPalette,
    light: &CosmicPalette,
    accent: Rgb8,
) -> anyhow::Result<()> {
    let base = xdg_config_home()?.join("cosmic");
    let dark_dir = base.join("com.system76.CosmicTheme.Dark.Builder/v1");
    let light_dir = base.join("com.system76.CosmicTheme.Light.Builder/v1");

    // Runtime theme keys (what COSMIC uses live).
    let dark_theme_dir = base.join("com.system76.CosmicTheme.Dark/v1");
    let light_theme_dir = base.join("com.system76.CosmicTheme.Light/v1");

    // Builder keys: keep it small.
    write_key(
        &dark_dir.join("palette"),
        &format!("{}\n", cosmic_palette_ron("Dark", dark)),
    )?;
    write_key(
        &light_dir.join("palette"),
        &format!("{}\n", cosmic_palette_ron("Light", light)),
    )?;

    // Also update the runtime theme palettes (these are structs, not the Dark/Light enum wrapper).
    write_key(
        &dark_theme_dir.join("palette"),
        &format!("{}\n", cosmic_palette_struct_ron(dark)),
    )?;
    write_key(
        &light_theme_dir.join("palette"),
        &format!("{}\n", cosmic_palette_struct_ron(light)),
    )?;

    // Accent as optional tint.
    write_key(
        &dark_dir.join("accent"),
        &format!("{}\n", ron_some_rgb(accent)),
    )?;
    write_key(
        &light_dir.join("accent"),
        &format!("{}\n", ron_some_rgb(accent)),
    )?;

    // Update runtime accent tokens so the system reflects changes immediately.
    // COSMIC's appearance UI and widgets typically rely on these keys.
    let accent_tok = ron_simple_component(accent);
    write_key(&dark_theme_dir.join("accent"), &format!("{accent_tok}\n"))?;
    write_key(&light_theme_dir.join("accent"), &format!("{accent_tok}\n"))?;
    write_key(
        &dark_theme_dir.join("accent_button"),
        &format!("{accent_tok}\n"),
    )?;
    write_key(
        &light_theme_dir.join("accent_button"),
        &format!("{accent_tok}\n"),
    )?;

    // Update palette names (theme list / debug).
    // If COSMIC expects a quoted string, builder uses the palette struct name field.
    // Still, we also try to update the full theme `name` keys for clarity.
    let dark_name = base.join("com.system76.CosmicTheme.Dark/v1/name");
    let light_name = base.join("com.system76.CosmicTheme.Light/v1/name");
    let dn = format!("\"{theme_base_name}-dark\"\n");
    let ln = format!("\"{theme_base_name}-light\"\n");
    let _ = write_key(&dark_name, &dn);
    let _ = write_key(&light_name, &ln);

    Ok(())
}

fn ron_simple_component(base: Rgb8) -> String {
    // Format matches `~/.config/cosmic/com.system76.CosmicTheme.*/v1/accent`.
    // We synthesize a reasonable ramp from a single base color.
    let black = Rgb8::new(0, 0, 0);
    let white = Rgb8::new(255, 255, 255);
    let on = if contrast_ratio(base, black) >= contrast_ratio(base, white) {
        black
    } else {
        white
    };

    let hover = lerp_rgb_linear(base, on, 0.15);
    let pressed = lerp_rgb_linear(base, on, 0.45);
    let selected = hover;
    let selected_text = base;
    let focus = base;
    let divider = on;
    let disabled = base;
    let on_disabled = lerp_rgb_linear(on, base, 0.5);
    let border = base;
    let disabled_border = base;

    let mut s = String::new();
    s.push_str("(\n");
    s.push_str(&format!("    base: {},\n", ron_rgba(base, 1.0)));
    s.push_str(&format!("    hover: {},\n", ron_rgba(hover, 1.0)));
    s.push_str(&format!("    pressed: {},\n", ron_rgba(pressed, 1.0)));
    s.push_str(&format!("    selected: {},\n", ron_rgba(selected, 1.0)));
    s.push_str(&format!(
        "    selected_text: {},\n",
        ron_rgba(selected_text, 1.0)
    ));
    s.push_str(&format!("    focus: {},\n", ron_rgba(focus, 1.0)));
    s.push_str(&format!("    divider: {},\n", ron_rgba(divider, 1.0)));
    s.push_str(&format!("    on: {},\n", ron_rgba(on, 1.0)));
    s.push_str(&format!("    disabled: {},\n", ron_rgba(disabled, 1.0)));
    s.push_str(&format!(
        "    on_disabled: {},\n",
        ron_rgba(on_disabled, 1.0)
    ));
    s.push_str(&format!("    border: {},\n", ron_rgba(border, 1.0)));
    s.push_str(&format!(
        "    disabled_border: {},\n",
        ron_rgba(disabled_border, 0.5)
    ));
    s.push_str(")");
    s
}

fn write_key(path: &Path, contents: &str) -> anyhow::Result<()> {
    atomic_write(path, contents.as_bytes())
}

#[derive(Debug, Clone)]
struct CosmicPalette {
    name: String,
    // Base colors.
    bright_red: Rgb8,
    bright_green: Rgb8,
    bright_orange: Rgb8,
    gray_1: Rgb8,
    gray_2: Rgb8,
    neutrals: [Rgb8; 11],
    // Accent set.
    accent_blue: Rgb8,
    accent_indigo: Rgb8,
    accent_purple: Rgb8,
    accent_pink: Rgb8,
    accent_red: Rgb8,
    accent_orange: Rgb8,
    accent_yellow: Rgb8,
    accent_green: Rgb8,
    accent_warm_grey: Rgb8,
    // Extended accents.
    ext_warm_grey: Rgb8,
    ext_orange: Rgb8,
    ext_yellow: Rgb8,
    ext_blue: Rgb8,
    ext_purple: Rgb8,
    ext_pink: Rgb8,
    ext_indigo: Rgb8,
}

fn build_cosmic_palette_dark(
    theme_base_name: &str,
    pal: &Palette16,
    accent: Rgb8,
) -> CosmicPalette {
    let bg = pal.bg;
    let fg = pal.fg;
    let neutrals = neutrals(bg, fg);
    let warm_grey = neutrals[8];

    let reds = pick_or(accent, pal, 0.0);
    let oranges = pick_or(accent, pal, 30.0);
    let yellows = pick_or(accent, pal, 55.0);
    let greens = pick_or(accent, pal, 120.0);
    let blues = pick_or(accent, pal, 200.0);
    let indigos = pick_or(accent, pal, 250.0);
    let purples = pick_or(accent, pal, 290.0);
    let pinks = pick_or(accent, pal, 330.0);

    CosmicPalette {
        name: format!("{theme_base_name}-dark"),
        bright_red: reds,
        bright_green: greens,
        bright_orange: oranges,
        gray_1: neutrals[2],
        gray_2: neutrals[3],
        neutrals,

        accent_blue: blues,
        accent_indigo: indigos,
        accent_purple: purples,
        accent_pink: pinks,
        accent_red: reds,
        accent_orange: oranges,
        accent_yellow: yellows,
        accent_green: greens,
        accent_warm_grey: warm_grey,

        // v0: ext = same base accents.
        ext_warm_grey: warm_grey,
        ext_orange: oranges,
        ext_yellow: yellows,
        ext_blue: blues,
        ext_purple: purples,
        ext_pink: pinks,
        ext_indigo: indigos,
    }
}

fn build_cosmic_palette_light(
    theme_base_name: &str,
    pal: &Palette16,
    accent: Rgb8,
) -> CosmicPalette {
    // For light, invert the neutral ramp (light bg, dark fg).
    let mut sorted = pal.colors;
    sorted.sort_by(|a, b| a.rel_luminance().total_cmp(&b.rel_luminance()));
    let fg = sorted[0];
    let bg = sorted[15];
    let neutrals = neutrals(bg, fg);
    let warm_grey = neutrals[2];

    let reds = pick_or(accent, pal, 0.0);
    let oranges = pick_or(accent, pal, 30.0);
    let yellows = pick_or(accent, pal, 55.0);
    let greens = pick_or(accent, pal, 120.0);
    let blues = pick_or(accent, pal, 200.0);
    let indigos = pick_or(accent, pal, 250.0);
    let purples = pick_or(accent, pal, 290.0);
    let pinks = pick_or(accent, pal, 330.0);

    CosmicPalette {
        name: format!("{theme_base_name}-light"),
        bright_red: reds,
        bright_green: greens,
        bright_orange: oranges,
        gray_1: neutrals[8],
        gray_2: neutrals[7],
        neutrals,

        accent_blue: blues,
        accent_indigo: indigos,
        accent_purple: purples,
        accent_pink: pinks,
        accent_red: reds,
        accent_orange: oranges,
        accent_yellow: yellows,
        accent_green: greens,
        accent_warm_grey: warm_grey,

        ext_warm_grey: warm_grey,
        ext_orange: oranges,
        ext_yellow: yellows,
        ext_blue: blues,
        ext_purple: purples,
        ext_pink: pinks,
        ext_indigo: indigos,
    }
}

fn neutrals(bg: Rgb8, fg: Rgb8) -> [Rgb8; 11] {
    let mut out = [Rgb8::new(0, 0, 0); 11];
    for i in 0..11 {
        let t = i as f32 / 10.0;
        out[i] = lerp_rgb_linear(bg, fg, t);
    }
    out
}

fn pick_accent(pal: &Palette16) -> Rgb8 {
    // Prefer a saturated mid-tone.
    let mut best = pal.cursor;
    let mut best_score = -1.0f32;
    for &c in &pal.colors {
        let hsl = c.to_hsl();
        // Ignore near-grays.
        if hsl.s < 0.20 {
            continue;
        }
        if hsl.l < 0.20 || hsl.l > 0.85 {
            continue;
        }
        let score = hsl.s;
        if score > best_score {
            best = c;
            best_score = score;
        }
    }
    best
}

fn pick_or(fallback: Rgb8, pal: &Palette16, target_h: f32) -> Rgb8 {
    let mut best: Option<(Rgb8, f32, f32)> = None; // (color, dist, -sat)
    for &c in &pal.colors {
        let hsl = c.to_hsl();
        if hsl.s < 0.20 {
            continue;
        }
        if hsl.l < 0.10 || hsl.l > 0.90 {
            continue;
        }
        let dist = hue_distance(hsl.h, target_h);
        let key = (dist, -hsl.s);
        match best {
            None => best = Some((c, key.0, key.1)),
            Some((_bc, bd, bs)) => {
                if (key.0, key.1) < (bd, bs) {
                    best = Some((c, key.0, key.1));
                }
            }
        }
    }
    best.map(|(c, _, _)| c).unwrap_or(fallback)
}

fn hue_distance(a: f32, b: f32) -> f32 {
    let d = (a - b).abs();
    d.min(360.0 - d)
}

fn cosmic_palette_ron(kind: &str, p: &CosmicPalette) -> String {
    // kind is "Dark" or "Light".
    let mut s = String::new();
    s.push_str(kind);
    s.push_str("((\n");
    s.push_str(&format!("    name: \"{}\",\n", p.name));
    s.push_str(&format!(
        "    bright_red: {},\n",
        ron_rgba(p.bright_red, 1.0)
    ));
    s.push_str(&format!(
        "    bright_green: {},\n",
        ron_rgba(p.bright_green, 1.0)
    ));
    s.push_str(&format!(
        "    bright_orange: {},\n",
        ron_rgba(p.bright_orange, 1.0)
    ));
    s.push_str(&format!("    gray_1: {},\n", ron_rgba(p.gray_1, 1.0)));
    s.push_str(&format!("    gray_2: {},\n", ron_rgba(p.gray_2, 1.0)));

    for (i, c) in p.neutrals.iter().enumerate() {
        s.push_str(&format!("    neutral_{i}: {},\n", ron_rgba(*c, 1.0)));
    }

    s.push_str(&format!(
        "    accent_blue: {},\n",
        ron_rgba(p.accent_blue, 1.0)
    ));
    s.push_str(&format!(
        "    accent_indigo: {},\n",
        ron_rgba(p.accent_indigo, 1.0)
    ));
    s.push_str(&format!(
        "    accent_purple: {},\n",
        ron_rgba(p.accent_purple, 1.0)
    ));
    s.push_str(&format!(
        "    accent_pink: {},\n",
        ron_rgba(p.accent_pink, 1.0)
    ));
    s.push_str(&format!(
        "    accent_red: {},\n",
        ron_rgba(p.accent_red, 1.0)
    ));
    s.push_str(&format!(
        "    accent_orange: {},\n",
        ron_rgba(p.accent_orange, 1.0)
    ));
    s.push_str(&format!(
        "    accent_yellow: {},\n",
        ron_rgba(p.accent_yellow, 1.0)
    ));
    s.push_str(&format!(
        "    accent_green: {},\n",
        ron_rgba(p.accent_green, 1.0)
    ));
    s.push_str(&format!(
        "    accent_warm_grey: {},\n",
        ron_rgba(p.accent_warm_grey, 1.0)
    ));

    s.push_str(&format!(
        "    ext_warm_grey: {},\n",
        ron_rgba(p.ext_warm_grey, 1.0)
    ));
    s.push_str(&format!(
        "    ext_orange: {},\n",
        ron_rgba(p.ext_orange, 1.0)
    ));
    s.push_str(&format!(
        "    ext_yellow: {},\n",
        ron_rgba(p.ext_yellow, 1.0)
    ));
    s.push_str(&format!("    ext_blue: {},\n", ron_rgba(p.ext_blue, 1.0)));
    s.push_str(&format!(
        "    ext_purple: {},\n",
        ron_rgba(p.ext_purple, 1.0)
    ));
    s.push_str(&format!("    ext_pink: {},\n", ron_rgba(p.ext_pink, 1.0)));
    s.push_str(&format!(
        "    ext_indigo: {},\n",
        ron_rgba(p.ext_indigo, 1.0)
    ));
    s.push_str("))");
    s
}

fn cosmic_palette_struct_ron(p: &CosmicPalette) -> String {
    // Matches `~/.config/cosmic/com.system76.CosmicTheme.{Dark,Light}/v1/palette`.
    let mut s = String::new();
    s.push_str("(\n");
    s.push_str(&format!("    name: \"{}\",\n", p.name));
    s.push_str(&format!(
        "    bright_red: {},\n",
        ron_rgba(p.bright_red, 1.0)
    ));
    s.push_str(&format!(
        "    bright_green: {},\n",
        ron_rgba(p.bright_green, 1.0)
    ));
    s.push_str(&format!(
        "    bright_orange: {},\n",
        ron_rgba(p.bright_orange, 1.0)
    ));
    s.push_str(&format!("    gray_1: {},\n", ron_rgba(p.gray_1, 1.0)));
    s.push_str(&format!("    gray_2: {},\n", ron_rgba(p.gray_2, 1.0)));

    for (i, c) in p.neutrals.iter().enumerate() {
        s.push_str(&format!("    neutral_{i}: {},\n", ron_rgba(*c, 1.0)));
    }

    s.push_str(&format!(
        "    accent_blue: {},\n",
        ron_rgba(p.accent_blue, 1.0)
    ));
    s.push_str(&format!(
        "    accent_indigo: {},\n",
        ron_rgba(p.accent_indigo, 1.0)
    ));
    s.push_str(&format!(
        "    accent_purple: {},\n",
        ron_rgba(p.accent_purple, 1.0)
    ));
    s.push_str(&format!(
        "    accent_pink: {},\n",
        ron_rgba(p.accent_pink, 1.0)
    ));
    s.push_str(&format!(
        "    accent_red: {},\n",
        ron_rgba(p.accent_red, 1.0)
    ));
    s.push_str(&format!(
        "    accent_orange: {},\n",
        ron_rgba(p.accent_orange, 1.0)
    ));
    s.push_str(&format!(
        "    accent_yellow: {},\n",
        ron_rgba(p.accent_yellow, 1.0)
    ));
    s.push_str(&format!(
        "    accent_green: {},\n",
        ron_rgba(p.accent_green, 1.0)
    ));
    s.push_str(&format!(
        "    accent_warm_grey: {},\n",
        ron_rgba(p.accent_warm_grey, 1.0)
    ));

    s.push_str(&format!(
        "    ext_warm_grey: {},\n",
        ron_rgba(p.ext_warm_grey, 1.0)
    ));
    s.push_str(&format!(
        "    ext_orange: {},\n",
        ron_rgba(p.ext_orange, 1.0)
    ));
    s.push_str(&format!(
        "    ext_yellow: {},\n",
        ron_rgba(p.ext_yellow, 1.0)
    ));
    s.push_str(&format!("    ext_blue: {},\n", ron_rgba(p.ext_blue, 1.0)));
    s.push_str(&format!(
        "    ext_purple: {},\n",
        ron_rgba(p.ext_purple, 1.0)
    ));
    s.push_str(&format!("    ext_pink: {},\n", ron_rgba(p.ext_pink, 1.0)));
    s.push_str(&format!(
        "    ext_indigo: {},\n",
        ron_rgba(p.ext_indigo, 1.0)
    ));
    s.push_str(")");
    s
}

fn cosmic_theme_file(kind: &str, p: &CosmicPalette) -> String {
    // Match the shape of `/usr/share/cosmic-themes/*.ron`.
    // Keep spacing/radii defaults stable; only palette changes.
    let palette = cosmic_palette_ron(kind, p);
    format!(
        "(\n    palette: {palette},\n    spacing: (\n        space_none: 0,\n        space_xxxs: 4,\n        space_xxs: 8,\n        space_xs: 12,\n        space_s: 16,\n        space_m: 24,\n        space_l: 32,\n        space_xl: 48,\n        space_xxl: 64,\n        space_xxxl: 128,\n    ),\n    corner_radii: (\n        radius_0: (0.0, 0.0, 0.0, 0.0),\n        radius_xs: (4.0, 4.0, 4.0, 4.0),\n        radius_s: (8.0, 8.0, 8.0, 8.0),\n        radius_m: (16.0, 16.0, 16.0, 16.0),\n        radius_l: (32.0, 32.0, 32.0, 32.0),\n        radius_xl: (160.0, 160.0, 160.0, 160.0),\n    ),\n    neutral_tint: None,\n    bg_color: None,\n    primary_container_bg: None,\n    secondary_container_bg: None,\n    text_tint: None,\n    accent: None,\n    success: None,\n    warning: None,\n    destructive: None,\n    is_frosted: false,\n    gaps: (0, 8),\n    active_hint: 3,\n    window_hint: None,\n)\n"
    )
}

fn ron_rgba(c: Rgb8, a: f32) -> String {
    let (r, g, b) = c.to_f32();
    format!(
        "(\n        red: {r},\n        green: {g},\n        blue: {b},\n        alpha: {a},\n    )",
        r = trim_float(r),
        g = trim_float(g),
        b = trim_float(b),
        a = trim_float(a)
    )
}

fn ron_some_rgb(c: Rgb8) -> String {
    let (r, g, b) = c.to_f32();
    format!(
        "Some((\n    red: {r},\n    green: {g},\n    blue: {b},\n))",
        r = trim_float(r),
        g = trim_float(g),
        b = trim_float(b)
    )
}

fn trim_float(x: f32) -> String {
    // COSMIC config files generally store decimals; keep short but stable.
    // 0.0/1.0 remain compact.
    let s = format!("{x:.7}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s.is_empty() {
        "0".to_string()
    } else {
        s.to_string()
    }
}
