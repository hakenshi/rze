//! Wallpaper backend that calls `nayu set <path>`.

use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Context};

pub fn set(image_abs: &Path) -> anyhow::Result<()> {
    let status = Command::new("nayu")
        .arg("set")
        .arg(image_abs)
        .status()
        .context("run nayu set")?;
    if !status.success() {
        return Err(anyhow!("nayu set failed"));
    }
    Ok(())
}
