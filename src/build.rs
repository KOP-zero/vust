use crate::config::VustConfig;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
pub fn 构建rust(config: &VustConfig, output: impl AsRef<Path>) -> Result<()> {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target-dir")
        .arg(output.as_ref())
        .current_dir(&config.get_backend_path())
        .status()
        .context("构建rust失败")?;
    Ok(())
}
pub fn 构建vue(config: &VustConfig, output: impl AsRef<Path>) -> Result<()> {
    Command::new("pnpm")
        .arg("build")
        .arg("--outDir")
        .arg(output.as_ref())
        .current_dir(&config.get_frontend_path())
        .status()
        .context("构建VUE失败")?;
    Ok(())
}
