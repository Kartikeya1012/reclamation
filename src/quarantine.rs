use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub id: String,
    pub entries: Vec<(PathBuf, PathBuf)>,
}

pub struct Ops {
    quarantine_dir: PathBuf,
    manifests_dir: PathBuf,
}

impl Ops {
    pub fn new(config: crate::config::Config) -> Self {
        Self {
            quarantine_dir: config.quarantine_dir,
            manifests_dir: config.manifests_dir,
        }
    }

    pub fn quarantine(&self, paths: &[PathBuf]) -> std::io::Result<Manifest> {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        
        let entries: Vec<_> = paths.iter()
            .filter_map(|original| {
                let filename = original.file_name()?;
                let quarantined = self.quarantine_dir.join(filename);
                fs::rename(original, &quarantined).ok()?;
                Some((original.clone(), quarantined))
            })
            .collect();
        
        let manifest = Manifest { id: id.clone(), entries };
        let path = self.manifests_dir.join(format!("{}.json", id));
        fs::write(&path, serde_json::to_string_pretty(&manifest)?)?;
        Ok(manifest)
    }

    pub fn restore(&self, manifest: &Manifest) -> std::io::Result<()> {
        for (original, quarantined) in &manifest.entries {
            if let Some(parent) = original.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::rename(quarantined, original)?;
        }
        let path = self.manifests_dir.join(format!("{}.json", manifest.id));
        fs::remove_file(path)?;
        Ok(())
    }

    pub fn load(&self, id: &str) -> std::io::Result<Manifest> {
        let path = self.manifests_dir.join(format!("{}.json", id));
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn list(&self) -> std::io::Result<Vec<String>> {
        Ok(fs::read_dir(&self.manifests_dir)?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                e.path().file_stem()?.to_str().map(|s| s.to_string())
            })
            .collect())
    }
}
