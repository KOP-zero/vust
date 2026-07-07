use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
#[derive(Debug, Deserialize, Serialize)]
pub struct VustConfig {
    project: Project,
    paths: Paths,
    frontend: FrontendConfig,
    backend: BackendConfig,
    status: StatusConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct Project {
    name: String,
    version: String,
    description: String,
    language: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Paths {
    root: String,
    frontend: String,
    backend: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FrontendConfig {
    framework: String,
    version: String,
    build_command: String,
    dev_command: String,
    port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct BackendConfig {
    framework: String,
    version: String,
    port: u16,
    database: Option<String>,
    database_url: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
struct StatusConfig {
    dependencies_installed: bool,
    last_build: String,
}
impl VustConfig {
    pub fn init(path: impl AsRef<Path>) -> Result<Self> {
        if !path.as_ref().join("Vust.toml").exists() {
            return Err(anyhow!(
                "没有读取到'Vust.toml'文件，请检查是否在全栈项目根目录"
            ));
        }
        let content = fs::read_to_string(path.as_ref().join("Vust.toml"))?;
        let config: VustConfig = toml::from_str(&content)?;
        Ok(config)
    }
    pub fn get_backend_port(&self) -> u16 {
        self.backend.port
    }
    pub fn get_rootpath(&self) -> PathBuf {
        PathBuf::from(&self.paths.root)
    }
    pub fn get_frontend_path(&self) -> PathBuf {
        PathBuf::from(&self.paths.frontend)
    }

    pub fn get_backend_path(&self) -> PathBuf {
        PathBuf::from(&self.paths.backend)
    }
    pub fn is_depen_install(&self) -> bool {
        self.status.dependencies_installed
    }
    pub fn depen_install(&mut self) {
        self.status.dependencies_installed = true
    }
    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(&self)?;
        fs::write(self.get_rootpath().join("Vust.toml"), content)?;
        Ok(())
    }
}
