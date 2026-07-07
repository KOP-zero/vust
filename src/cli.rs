use anyhow::Result;
pub use clap::{Parser, Subcommand};
use dialoguer::Confirm;
use std::env;
use std::path::PathBuf;
#[derive(Parser)]
#[command(name = "全栈脚手架")]
#[command(about = "rust后端加上vue前端脚手架", version = "0.1.0")]
pub struct Arg {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(clap::Args)]
#[command(
    about = "创建一个新的全栈项目",
    after_help = "参数说明：\n  -p, --port 设置后端端口\n  -d, --database 是否使用数据库\n  -t, --typescript 是否使用 TypeScript\n  -g, --git 是否初始化 Git 仓库\n  -c, --vscode 是否生成 VSCode 配置\n  -i, --interactive 启动交互式配置模式"
)]
pub struct NewArg {
    #[arg(value_name = "项目名称", default_value = "rust_vue_project")]
    pub name: String,
    #[arg(short, long, default_value = "8080", help = "后端端口")]
    pub port: i32,
    #[arg(short, long, default_value = "false", help = "使用基础数据库")]
    pub database: bool,
    #[arg(short, long, default_value = "true", help = "使用typescript")]
    pub typescript: bool,
    #[arg(short, long, default_value = "true", help = "初始化git仓库")]
    pub git: bool,
    #[arg(short, long, default_value = "true", help = "是否使用vscode")]
    pub vscode: bool,
    #[arg(short, long, default_value = "false", help = "启动交互式配置")]
    pub interactive: bool,
}
impl NewArg {
    pub fn interactive(&mut self) -> Result<()> {
        self.database = Confirm::new()
            .with_prompt("使用数据库?")
            .default(self.database)
            .interact()?;
        self.typescript = Confirm::new()
            .with_prompt("使用 TypeScript?")
            .default(self.typescript)
            .interact()?;
        self.git = Confirm::new()
            .with_prompt("创建git仓库?")
            .default(self.git)
            .interact()?;
        self.vscode = Confirm::new()
            .with_prompt("是否使用vscode?")
            .default(self.vscode)
            .interact()?;
        Ok(())
    }
    pub fn root_path(&self) -> PathBuf {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        current_dir.join(&self.name)
    }
}
#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "创建新的全栈项目")]
    New(NewArg),
    #[command(about = "启动前后端开发服务器")]
    Dev {
        #[arg(default_value = ".", help = "项目路径")]
        path: String,
    },
    #[command(about = "构建前后端生产版本")]
    Build {
        #[arg(default_value = ".", help = "项目路径")]
        path: String,

        #[arg(default_value = "../output", help = "输出目录")]
        output: String,
    },
}
