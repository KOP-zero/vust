use anyhow::Result;
use tokio::process::Command;
use tokio::select;
use tokio::signal;

use crate::config::VustConfig;
pub async fn dev(config: &mut VustConfig) -> Result<()> {
    //启动后端进程
    let mut back_process = Command::new("cargo")
        .arg("run")
        .current_dir(&config.get_backend_path())
        .kill_on_drop(true)
        .spawn()?;
    if !config.is_depen_install() {
        println!("📦 首次运行，正在安装前端依赖...");
        let status = Command::new("pnpm")
            .arg("install")
            .current_dir(&config.get_frontend_path())
            .status()
            .await?;

        if !status.success() {
            anyhow::bail!("依赖安装失败");
        }
        println!("✅ 依赖安装完成");
        config.depen_install();
        config.save()?;
    }
    //启动前端进程
    let mut front_process = Command::new("pnpm")
        .arg("dev")
        .current_dir(&config.get_frontend_path())
        .kill_on_drop(true)
        .spawn()?;
    println!("🚀 前后端开发服务器已启动！");
    println!("   🔧 后端: http://localhost:{}", config.get_backend_port());
    println!("   🎨 前端: http://localhost:5173");
    println!("   📦 按 Ctrl+C 停止所有服务\n");
    let ctrl_c = signal::ctrl_c();
    tokio::pin!(ctrl_c);
    // 等待任意一个完成
    select! {
        _ = back_process.wait() => {
            let _ = front_process.kill().await;
            let _ = front_process.wait().await;
            println!("❌ 后端进程已退出");
        }
        _ = front_process.wait() => {
            let _ = back_process.kill().await;
            let _ = back_process.wait().await;
            println!("❌ 前端进程已退出");
        }
        _ = &mut ctrl_c => {
            println!("\n🛑 收到停止信号，正在关闭服务...");
            // 主动 kill 进程
            let _ = back_process.kill().await;
            let _ = front_process.kill().await;
            let _ = back_process.wait().await;
            let _ = front_process.wait().await;
            println!("✅ 服务已停止");
        }
    }
    Ok(())
}
