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
        args::Command::Img { .. }
        | args::Command::Apply { .. }
        | args::Command::Init { .. }
        | args::Command::Wallhaven { .. } => Err(anyhow::anyhow!(
            "not implemented yet (docs-first; structure is ready)"
        )),
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
