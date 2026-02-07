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
