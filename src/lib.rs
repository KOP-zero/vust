pub mod build;
pub mod cli;
pub mod config;
pub mod dev;
pub mod new;
pub mod templates;
use crate::{cli::NewArg, config::VustConfig};

use std::path::Path;
use tokio::runtime::Builder;
pub fn new_project(arg: &NewArg) {
    std::fs::create_dir(&arg.name).expect("目录已存在");
    if arg.git {
        templates::创建gitignore(&arg.name).unwrap();
        new::创建git(&arg.name).unwrap();
    }
    if arg.vscode {
        templates::创建vscode配置(&arg.name).unwrap();
    }
    templates::模板vust文件(arg).unwrap();
    new::创建rust项目(arg).unwrap();
    new::创建vue项目(arg).unwrap();
}
pub fn dev_start(path: impl AsRef<Path>) {
    let mut config = VustConfig::init(path).unwrap();
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(async { dev::dev(&mut config).await.unwrap() })
}
pub fn build_start(path: impl AsRef<Path>, output: impl AsRef<Path>) {
    let config = VustConfig::init(path).unwrap();
    build::构建rust(&config, &output).unwrap();
    build::构建vue(&config, &output).unwrap();
}
