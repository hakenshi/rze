//! CLI output formatting: one-line errors by default; verbose with RZE_DEBUG=1.

pub fn debug_enabled() -> bool {
    std::env::var_os("RZE_DEBUG").is_some_and(|v| !v.is_empty())
}

pub fn print_error(err: &anyhow::Error) {
    if debug_enabled() {
        eprintln!("{err:#}");
    } else {
        eprintln!("{err}");
    }
}
