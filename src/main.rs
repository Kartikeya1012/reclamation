use std::env;
use std::path::PathBuf;

mod config;
mod classify;
mod quarantine;
mod web;

#[path = "lib.rs"]
mod lib;

use classify::{Classification, reason};
use lib::{triage_folder, clean_folder, list_manifests, restore_manifest, summarize_triage};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);
    let cmd = args.next().unwrap_or_default();
    
    match cmd.as_str() {
        "triage" => {
            let path = PathBuf::from(args.next()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing path"))?);
            let (auto, review, skip) = triage_folder(&path)?;
            println!("Auto-safe: {} items", auto.len());
            for item in &auto {
                if let Some(r) = reason(item, Classification::AutoSafe) {
                    println!("  • {} - {}", item.display(), r);
                }
            }
            println!("Needs review: {} items", review.len());
            for item in &review {
                println!("  • {}", item.display());
            }
            println!("Do not touch: {} items", skip.len());
        }
        "clean" => {
            let path = PathBuf::from(args.next()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing path"))?);
            let manifest = clean_folder(&path)?;
            println!("Quarantined {} items", manifest.entries.len());
            println!("Manifest: {}", manifest.id);
        }
        "restore" => {
            let id = args.next()
                .or_else(|| list_manifests().ok()?.last().cloned())
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "No manifest"))?;
            restore_manifest(&id)?;
            println!("Restored: {}", id);
        }
        "list" => {
            let manifests = list_manifests()?;
            if manifests.is_empty() {
                println!("No manifests found");
            } else {
                println!("Available manifests:");
                for id in manifests {
                    println!("  {}", id);
                }
            }
        }
        "summarize" => {
            let path = PathBuf::from(args.next()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing path"))?);
            let summary = summarize_triage(&path).await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            println!("{}", summary);
        }
        "web" => {
            use axum::Router;
            use tokio::net::TcpListener;
            
            let app: Router = web::create_router();
            let listener = TcpListener::bind("127.0.0.1:3000").await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            println!("Web UI running at http://127.0.0.1:3000");
            axum::serve(listener, app).await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        }
        _ => eprintln!("Usage: reclamation [triage|clean|restore|list|summarize|web] [path|id]"),
    }
    Ok(())
}
