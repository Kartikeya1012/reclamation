use std::env;
use std::path::PathBuf;
use std::fs;

mod config;
mod classify;
mod quarantine;

use classify::{Classification, classify, reason};
use quarantine::{Ops, Manifest};

fn triage_folder(path: &PathBuf) -> std::io::Result<(Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>)> {
    let items: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    
    let (mut auto, mut review, mut skip) = (Vec::new(), Vec::new(), Vec::new());
    for item in items {
        match classify(&item) {
            Classification::AutoSafe => auto.push(item),
            Classification::NeedsReview => review.push(item),
            Classification::DoNotTouch => skip.push(item),
        }
    }
    Ok((auto, review, skip))
}

fn clean_folder(path: &PathBuf) -> std::io::Result<Manifest> {
    let config = config::Config::new()?;
    let (auto_safe, _, _) = triage_folder(path)?;
    let ops = Ops::new(config);
    ops.quarantine(&auto_safe)
}

fn main() -> std::io::Result<()> {
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
            let config = config::Config::new()?;
            let ops = Ops::new(config);
            let id = args.next().or_else(|| ops.list()?.last().cloned())
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "No manifest"))?;
            let manifest = ops.load(&id)?;
            ops.restore(&manifest)?;
            println!("Restored: {}", id);
        }
        _ => eprintln!("Usage: reclamation [triage|clean|restore] [path|id]"),
    }
    Ok(())
}
