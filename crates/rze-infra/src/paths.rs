//! XDG path resolution for cache/config directories.

use std::path::PathBuf;

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Paths {
    pub cache_root: PathBuf,
    pub config_root: PathBuf,
    pub out_root: PathBuf,
    pub state_json: PathBuf,
}

impl Paths {
    pub fn compute() -> anyhow::Result<Self> {
        let home = home_dir()?;

        let cache_base = std::env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".cache"));
        let config_base = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"));

        let cache_root = cache_base.join("rze");
        let config_root = config_base.join("rze");

        Ok(Self {
            out_root: cache_root.join("out"),
            state_json: cache_root.join("state.json"),
            cache_root,
            config_root,
        })
    }
}

fn home_dir() -> anyhow::Result<PathBuf> {
    if let Some(home) = std::env::var_os("HOME") {
        return Ok(PathBuf::from(home));
    }
    Err(anyhow!("HOME is not set"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_env<F: FnOnce()>(f: F) {
        let _g = ENV_LOCK.lock().unwrap();

        let old_home = std::env::var_os("HOME");
        let old_cache = std::env::var_os("XDG_CACHE_HOME");
        let old_config = std::env::var_os("XDG_CONFIG_HOME");

        unsafe {
            std::env::remove_var("HOME");
            std::env::remove_var("XDG_CACHE_HOME");
            std::env::remove_var("XDG_CONFIG_HOME");
        }

        f();

        unsafe {
            match old_home {
                Some(v) => std::env::set_var("HOME", v),
                None => std::env::remove_var("HOME"),
            }
            match old_cache {
                Some(v) => std::env::set_var("XDG_CACHE_HOME", v),
                None => std::env::remove_var("XDG_CACHE_HOME"),
            }
            match old_config {
                Some(v) => std::env::set_var("XDG_CONFIG_HOME", v),
                None => std::env::remove_var("XDG_CONFIG_HOME"),
            }
        }
    }

    #[test]
    fn compute_uses_home_fallbacks() {
        with_env(|| {
            unsafe {
                std::env::set_var("HOME", "/home/testuser");
            }
            let p = Paths::compute().unwrap();
            assert_eq!(p.cache_root, PathBuf::from("/home/testuser/.cache/rze"));
            assert_eq!(p.config_root, PathBuf::from("/home/testuser/.config/rze"));
            assert_eq!(p.out_root, PathBuf::from("/home/testuser/.cache/rze/out"));
            assert_eq!(
                p.state_json,
                PathBuf::from("/home/testuser/.cache/rze/state.json")
            );
        });
    }

    #[test]
    fn compute_uses_xdg_overrides() {
        with_env(|| {
            unsafe {
                std::env::set_var("HOME", "/home/testuser");
                std::env::set_var("XDG_CACHE_HOME", "/tmp/cache");
                std::env::set_var("XDG_CONFIG_HOME", "/tmp/config");
            }
            let p = Paths::compute().unwrap();
            assert_eq!(p.cache_root, PathBuf::from("/tmp/cache/rze"));
            assert_eq!(p.config_root, PathBuf::from("/tmp/config/rze"));
        });
    }

    #[test]
    fn compute_errors_without_home() {
        with_env(|| {
            let err = Paths::compute().unwrap_err();
            assert!(format!("{err}").contains("HOME is not set"));
        });
    }
}
