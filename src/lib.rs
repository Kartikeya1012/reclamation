pub mod config;
pub mod classify;
pub mod quarantine;
pub mod ai;

use std::path::PathBuf;
use std::fs;
use self::classify::{Classification, classify};
use self::quarantine::{Ops, Manifest};

pub fn triage_folder(path: &PathBuf) -> std::io::Result<(Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>)> {
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

pub fn clean_folder(path: &PathBuf) -> std::io::Result<Manifest> {
    let config = self::config::Config::new()?;
    let (auto_safe, _, _) = triage_folder(path)?;
    let ops = Ops::new(crate::config::Config {
        quarantine_dir: config.quarantine_dir,
        manifests_dir: config.manifests_dir,
    });
    ops.quarantine(&auto_safe)
}

pub fn list_manifests() -> std::io::Result<Vec<String>> {
    let config = self::config::Config::new()?;
    let ops = Ops::new(crate::config::Config {
        quarantine_dir: config.quarantine_dir,
        manifests_dir: config.manifests_dir,
    });
    ops.list()
}

pub fn restore_manifest(id: &str) -> std::io::Result<()> {
    let config = self::config::Config::new()?;
    let ops = Ops::new(crate::config::Config {
        quarantine_dir: config.quarantine_dir,
        manifests_dir: config.manifests_dir,
    });
    let manifest = ops.load(id)?;
    ops.restore(&manifest)
}

pub async fn summarize_triage(path: &PathBuf) -> Result<String, String> {
    let (_, needs_review, _) = triage_folder(path)
        .map_err(|e| e.to_string())?;
    ai::summarize_files(&needs_review).await
}
