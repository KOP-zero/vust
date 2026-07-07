use anyhow::{Ok, Result};
use std::path::Path;
use tera::{Context, Tera};

use crate::cli::NewArg;
pub fn 模板main文件(arg: &NewArg) -> Result<()> {
    let mut tera = Tera::new();
    //读取模板文件到tera实例，参数二是别名
    tera.add_template_file("src/templates/main.rs.tera", Some("main"))?;
    //创建上下文管理器
    let mut context = Context::new();
    context.insert("project_name", &arg.name);
    context.insert("database", &arg.database);
    //模板渲染替换
    let rendered = tera.render("main", &context)?;
    //写入新的文件
    std::fs::write(
        arg.root_path().join("backend/src").join("main.rs"),
        rendered,
    )?;
    Ok(())
}
pub fn 模板toml文件(arg: &NewArg) -> Result<()> {
    let mut tera = Tera::new();
    tera.add_template_file("src/templates/Cargo.toml.tera", Some("toml"))?;
    let mut context = Context::new();
    context.insert("project_name", &arg.name);
    context.insert("database", &arg.database);
    let rendered = tera.render("toml", &context)?;
    std::fs::write(arg.root_path().join("backend").join("Cargo.toml"), rendered)?;
    Ok(())
}
pub fn 模板json文件(arg: &NewArg) -> Result<()> {
    let mut tera = Tera::new();
    tera.add_template_file("src/templates/package.json.tera", Some("json"))?;
    let mut context = Context::new();
    context.insert("project_name", &arg.name);
    context.insert("typescript", &arg.typescript);
    let rendered = tera.render("json", &context)?;
    std::fs::write(
        arg.root_path().join("frontend").join("package.json"),
        rendered,
    )?;
    Ok(())
}
pub fn 模板vite文件(arg: &NewArg) -> Result<()> {
    let mut tera = Tera::new();
    tera.add_template_file("src/templates/vite.config.ts.tera", Some("vite"))?;
    let mut context = Context::new();
    context.insert("backend_port", &arg.port);
    let rendered = tera.render("vite", &context)?;
    let name = if arg.typescript {
        "vite.config.ts"
    } else {
        "vite.config.js"
    };
    std::fs::write(arg.root_path().join("frontend").join(name), rendered)?;
    Ok(())
}
pub fn 模板app文件(arg: &NewArg) -> Result<()> {
    let mut tera = Tera::new();
    tera.add_template_file("src/templates/App.vue.tera", Some("app"))?;
    let mut context = Context::new();
    context.insert("project_name", &arg.name);
    context.insert("typescript", &arg.typescript);
    let rendered = tera.render("app", &context)?;
    std::fs::write(
        arg.root_path().join("frontend/src").join("App.vue"),
        rendered,
    )?;
    Ok(())
}
pub fn 模板env文件(arg: &NewArg) -> Result<()> {
    let mut tera = Tera::new();
    tera.add_template_file("src/templates/.env.tera", Some("env"))?;
    let mut context = Context::new();
    context.insert("backend_port", &arg.port);
    context.insert("database", &arg.database);
    let rendered = tera.render("env", &context)?;
    std::fs::write(arg.root_path().join(".env"), rendered)?;
    Ok(())
}
pub fn 创建gitignore(path: impl AsRef<Path>) -> Result<()> {
    let gitignore_content = r#"# ===== 后端 =====
backend/target/
backend/Cargo.lock
backend/.env

# ===== 前端 =====
frontend/node_modules/
frontend/dist/
frontend/.vite/
frontend/.env

# ===== 通用 =====
.vscode/
.idea/
.DS_Store
*.log
output/
"#;

    std::fs::write(path.as_ref().join(".gitignore"), gitignore_content)?;
    Ok(())
}
pub fn 创建vscode配置(root_path: impl AsRef<Path>) -> Result<()> {
    let vscode_path = root_path.as_ref().join(".vscode");
    std::fs::create_dir(&vscode_path)?;

    let settings = r#"{
  "rust-analyzer.linkedProjects": [
    "backend/Cargo.toml"
  ]
}
"#;
    std::fs::write(vscode_path.join("settings.json"), settings)?;
    Ok(())
}
pub fn 模板vust文件(arg: &NewArg) -> Result<()> {
    let mut tera = Tera::new();
    tera.add_template_file("src/templates/Vust.toml.tera", Some("vust"))?;
    let mut context = Context::new();
    context.insert("project_name", &arg.name);
    context.insert("backend_port", &arg.port);
    context.insert("database", &arg.database);
    context.insert("typescript", &arg.typescript);
    context.insert("root_path", &arg.root_path());
    let rendered = tera.render("vust", &context)?;
    std::fs::write(arg.root_path().join("Vust.toml"), rendered)?;
    Ok(())
}
