//! CLI output formatting: one-line errors by default; verbose with RZE_DEBUG=1.

pub fn debug_enabled() -> bool {
    std::env::var_os("RZE_DEBUG").is_some_and(|v| !v.is_empty())
}

pub fn print_error(err: &anyhow::Error) {
    if debug_enabled() {
        eprintln!("{err:#}");
    } else {
        // Keep one-line errors, but include context chain.
        let parts: Vec<String> = err.chain().map(|e| e.to_string()).collect();
        eprintln!("{}", parts.join(": "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn debug_enabled_tracks_env() {
        let _g = ENV_LOCK.lock().unwrap();
        let old = std::env::var_os("RZE_DEBUG");

        unsafe {
            std::env::remove_var("RZE_DEBUG");
        }
        assert!(!debug_enabled());

        unsafe {
            std::env::set_var("RZE_DEBUG", "1");
        }
        assert!(debug_enabled());

        unsafe {
            std::env::set_var("RZE_DEBUG", "");
        }
        assert!(!debug_enabled());

        unsafe {
            match old {
                Some(v) => std::env::set_var("RZE_DEBUG", v),
                None => std::env::remove_var("RZE_DEBUG"),
            }
        }
    }
}
