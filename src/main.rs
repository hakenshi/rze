mod args;
mod output;
mod wiring;

fn main() {
    if let Err(err) = real_main() {
        output::print_error(&err);
        std::process::exit(1);
    }
}

fn real_main() -> anyhow::Result<()> {
    use anyhow::Context as _;
    use clap::Parser as _;

    let cli = args::Cli::parse();

    match cli.cmd {
        args::Command::Env => {
            let p = rze_infra::paths::Paths::compute()?;
            println!("export RZE_CACHE={}", shell_quote(&p.cache_root));
            println!("export RZE_CONFIG={}", shell_quote(&p.config_root));
            println!("export RZE_STATE={}", shell_quote(&p.state_json));
            println!("export RZE_OUT={}", shell_quote(&p.out_root));
            Ok(())
        }
        args::Command::Img { input, no_reset: _ } => {
            // v0: local paths only; URL support is planned.
            if input.starts_with("http://") || input.starts_with("https://") {
                return Err(anyhow::anyhow!("URL inputs not implemented yet"));
            }

            let p = std::path::PathBuf::from(input);
            let abs = if p.is_absolute() {
                p
            } else {
                std::env::current_dir()?.join(p)
            };
            if !abs.exists() {
                return Err(anyhow::anyhow!("image path does not exist"))
                    .with_context(|| format!("{abs:?}"));
            }

            // 1) Wallpaper (delegated to nayu; it will pick COSMIC/GNOME/KDE/Wayland).
            // Non-fatal for now: theme generation is our focus.
            let _ = rze_infra::wallpaper::nayu::set(&abs);

            // 2) Palette.
            let pixels = rze_infra::ffmpeg_decoder::decode_rgb24_cover(&abs, 128, 128)
                .with_context(|| format!("decode {}", abs.display()))?;
            let colors = rze_core::quantize_mediancut::quantize_16(&pixels);
            let pal = rze_core::palette::Palette16::from_quantized(colors);

            // 3) COSMIC theme.
            rze_infra::cosmic::cosmic_theme::install_and_apply("rze", &pal)
                .context("install/apply COSMIC theme")?;

            println!("OK");
            Ok(())
        }
        args::Command::Apply { .. }
        | args::Command::Init { .. }
        | args::Command::Wallhaven { .. } => Err(anyhow::anyhow!("not implemented yet")),
    }
}

fn shell_quote(p: &std::path::Path) -> String {
    // Minimal quoting: wrap in single quotes and escape embedded single quotes.
    let s = p.to_string_lossy();
    if !s.contains('"') && !s.contains(' ') && !s.contains('\t') && !s.contains('\n') {
        return s.into_owned();
    }
    let escaped = s.replace('"', "\\\"");
    format!("\"{escaped}\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_quote_passes_simple_paths() {
        assert_eq!(shell_quote(std::path::Path::new("/tmp/ok")), "/tmp/ok");
        assert_eq!(
            shell_quote(std::path::Path::new("/tmp/ok.txt")),
            "/tmp/ok.txt"
        );
    }

    #[test]
    fn shell_quote_quotes_spaces() {
        assert_eq!(
            shell_quote(std::path::Path::new("/tmp/has space")),
            "\"/tmp/has space\""
        );
    }

    #[test]
    fn shell_quote_escapes_double_quotes() {
        assert_eq!(
            shell_quote(std::path::Path::new("/tmp/a\"b")),
            "\"/tmp/a\\\"b\""
        );
    }
}
