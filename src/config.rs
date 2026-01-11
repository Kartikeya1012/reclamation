use std::path::PathBuf;
use std::fs;

pub struct Config {
    pub quarantine_dir: PathBuf,
    pub manifests_dir: PathBuf,
}

impl Config {
    pub fn new() -> std::io::Result<Self> {
        let base = PathBuf::from(std::env::var("HOME")
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "No HOME"))?)
            .join(".reclamation");
        
        let quarantine = base.join("quarantine");
        let manifests = base.join("manifests");
        
        fs::create_dir_all(&quarantine)?;
        fs::create_dir_all(&manifests)?;
        
        Ok(Self { quarantine_dir: quarantine, manifests_dir: manifests })
    }
}
