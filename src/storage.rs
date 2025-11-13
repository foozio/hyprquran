use crate::state::AyahRef;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Persisted {
    pub last: AyahRef,
    pub bookmarks: Vec<AyahRef>,
    pub translation_lang: Option<String>,
    pub prefer_dark: bool,
}

fn data_dir() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("org", "hyprquran", "HyprQuran").ok_or_else(|| anyhow::anyhow!("dirs"))?;
    Ok(dirs.data_dir().to_path_buf())
}

fn state_path() -> Result<PathBuf> {
    Ok(data_dir()?.join("state.json"))
}

pub fn load() -> Option<Persisted> {
    let path = state_path().ok()?;
    let s = fs::read_to_string(path).ok()?;
    serde_json::from_str(&s).ok()
}

pub fn save(p: &Persisted) -> Result<()> {
    let dir = data_dir()?;
    fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    let path = dir.join("state.json");
    let s = serde_json::to_string_pretty(p)?;
    fs::write(&path, s).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

pub fn add_bookmark(b: AyahRef) -> Result<()> {
    let mut p = load().unwrap_or_default();
    if !p.bookmarks.iter().any(|x| x == &b) {
        p.bookmarks.push(b);
    }
    save(&p)
}
