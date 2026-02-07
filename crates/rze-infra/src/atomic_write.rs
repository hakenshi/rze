//! Atomic write helpers (temp + rename).

use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context};

pub fn atomic_write(path: &Path, bytes: &[u8]) -> anyhow::Result<()> {
    let dir = path.parent().ok_or_else(|| anyhow!("invalid path"))?;
    fs::create_dir_all(dir).with_context(|| format!("create dir {}", dir.display()))?;

    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("tmp");
    let tmp = dir.join(format!(".{name}.tmp"));

    fs::write(&tmp, bytes).with_context(|| format!("write {}", tmp.display()))?;
    fs::rename(&tmp, path)
        .with_context(|| format!("rename {} -> {}", tmp.display(), path.display()))?;
    Ok(())
}
