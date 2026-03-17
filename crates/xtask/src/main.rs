use anyhow::{Context, Result};
use axum::Router;
use clap::{Parser, Subcommand};
use notify::{Event, RecursiveMode, Watcher};
use std::process::Command;
use std::time::Duration;
use tower_http::{services::ServeDir, compression::CompressionLayer};
use tower_livereload::LiveReloadLayer;

#[derive(Parser)]
#[command(name = "xtask", about = "Custom tasks for vs-map workspace")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build WASM
    Build {
        /// Build in dev mode instead of release
        #[arg(short, long)]
        dev: bool,
    },
    /// Start development server and auto-rebuild on changes
    Dev,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { dev } => {
            build_wasm(dev)?;
        }
        Commands::Dev => {
            run_dev_server().await?;
        }
    }

    Ok(())
}

fn build_wasm(dev: bool) -> Result<()> {
    println!("Building WASM application (dev: {})...", dev);

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("-p")
        .arg("vsmap-web")
        .arg("--target")
        .arg("wasm32-unknown-unknown");

    if !dev {
        //cmd.arg("--release");
        cmd.arg("--profile")
            .arg("wasm-release");
    }

    let status = cmd.status().context("Failed to run cargo build")?;
    if !status.success() {
        anyhow::bail!("Cargo build failed");
    }

    let workspace_root = std::env::current_dir()?;
    let build_dir = workspace_root.join("build");
    std::fs::create_dir_all(&build_dir)?;

    let html_src = workspace_root.join("crates/web/html/index.html");
    let html_dest = build_dir.join("index.html");
    std::fs::copy(&html_src, &html_dest).context("Failed to copy index.html")?;

    let target_dir = workspace_root.join("target").join("wasm32-unknown-unknown");
    let profile = if dev { "debug" } else { "wasm-release" };
    let wasm_src = target_dir.join(profile).join("vsmap-web.wasm");
    let wasm_dest = build_dir.join("vsmap-web.wasm");

    let mut opt_cmd = Command::new("wasm-opt");
    opt_cmd.arg(wasm_src)
        .arg("-o")
        .arg(wasm_dest)
        .arg("-Oz");

    let opt_status = opt_cmd.status().context("Failed to run wasm-opt")?;
    if !opt_status.success() {
        anyhow::bail!("wasm-opt failed");
    }

    //std::fs::copy(&wasm_src, &wasm_dest).context("Failed to copy wasm file")?;

    println!("Build successful! Artifacts copied to build/");
    Ok(())
}

async fn run_dev_server() -> Result<()> {
    // Initial build
    if let Err(e) = build_wasm(false) {
        eprintln!("Initial build failed: {}. Continuing to watch...", e);
    }

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let app = Router::new()
        .fallback_service(ServeDir::new("build"))
        .layer(livereload)
        .layer(CompressionLayer::new());

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            if event.kind.is_modify() || event.kind.is_create() || event.kind.is_remove() {
                // Ignore the build and target directories to prevent loops
                let should_rebuild = event.paths.iter().any(|path| {
                    let path_str = path.to_string_lossy();
                    !path_str.contains("/build/") && !path_str.contains("/target/")
                });

                if should_rebuild {
                    let _ = tx.blocking_send(());
                }
            }
        }
    })?;

    let workspace_root = std::env::current_dir()?;
    watcher.watch(&workspace_root.join("crates/web/src"), RecursiveMode::Recursive)?;
    watcher.watch(&workspace_root.join("crates/web/html"), RecursiveMode::Recursive)?;
    watcher.watch(&workspace_root.join("crates/lib/src"), RecursiveMode::Recursive)?;
    
    // Spawn a task to handle rebuilds
    tokio::spawn(async move {
        loop {
            if rx.recv().await.is_none() {
                break;
            }

            // Debounce
            tokio::time::sleep(Duration::from_millis(200)).await;

            // Drain extra events
            while let Ok(_) = rx.try_recv() {}

            println!("\nChanges detected, rebuilding...");
            match build_wasm(false) {
                Ok(_) => {
                    println!("Rebuild successful, reloading browser!");
                    reloader.reload();
                }
                Err(e) => {
                    eprintln!("Rebuild failed: {}", e);
                }
            }
        }
    });

    let addr = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Development server listening on http://127.0.0.1:8080");
    
    axum::serve(addr, app).await?;

    Ok(())
}
