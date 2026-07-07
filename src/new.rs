use crate::cli::NewArg;
use crate::templates;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;
pub fn 创建rust项目(arg: &NewArg) -> Result<()> {
    let path: &Path = arg.name.as_ref();
    let src_path = path.join("backend/src");
    fs::create_dir_all(&src_path)?;
    templates::模板main文件(&arg)?;
    templates::模板toml文件(&arg)?;
    templates::模板env文件(&arg)?;
    println!("Rust 项目初始化成功！");
    Ok(())
}
pub fn 创建vue项目(arg: &NewArg) -> Result<()> {
    let path: &Path = &arg.root_path();
    let front_path = path.join("frontend");
    fs::create_dir(&front_path)?;
    let vue = if arg.typescript { "vue-ts" } else { "vue" };
    Command::new("pnpm")
        .args([
            "create",
            "vite",
            ".",
            "--template",
            vue,
            "--skip-install",
            "--no-interactive",
        ])
        .current_dir(&front_path)
        .status()?;
    templates::模板json文件(&arg)?;
    templates::模板vite文件(&arg)?;
    templates::模板app文件(&arg)?;
    println!("Vue 项目初始化成功！");
    Ok(())
}

pub fn 创建git(root_path: impl AsRef<Path>) -> Result<()> {
    Command::new("git")
        .arg("init")
        .current_dir(root_path.as_ref())
        .status()?;
    println!("git仓库已经初始化");
    Ok(())
}
