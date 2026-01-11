use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classification {
    AutoSafe,
    NeedsReview,
    DoNotTouch,
}

const AUTO_SAFE: &[&str] = &[
    ".tmp",
    ".temp",
    ".log",
    ".DS_Store",
];

const DO_NOT_TOUCH: &[&str] = &[
    ".app",
    "/System/",
    "/Library/Frameworks/",
];

pub fn classify(path: &Path) -> Classification {
    let path_str = path.to_string_lossy();
    
    if DO_NOT_TOUCH.iter().any(|&pattern| path_str.contains(pattern)) {
        return Classification::DoNotTouch;
    }
    
    if AUTO_SAFE.iter().any(|&pattern| path_str.contains(pattern)) {
        return Classification::AutoSafe;
    }
    
    Classification::NeedsReview
}

pub fn reason(path: &Path, class: Classification) -> Option<String> {
    let path_str = path.to_string_lossy();
    match class {
        Classification::AutoSafe => {
            AUTO_SAFE.iter()
                .find(|&pattern| path_str.contains(pattern))
                .map(|p| format!("Contains: {}", p))
        }
        _ => None,
    }
}
