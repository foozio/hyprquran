use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationFile {
    pub lang: String,
    pub entries: Vec<TranslationEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationEntry {
    pub surah: u16,
    pub ayah: u16,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurahTextFile {
    pub surah: u16,
    pub name_ar: String,
    pub name_en: String,
    pub ayat: Vec<String>,
}

pub fn assets_dir() -> PathBuf {
    PathBuf::from("assets")
}

pub fn load_surah_text_fatiha() -> Result<SurahTextFile> {
    let path = assets_dir().join("quran").join("fatiha.json");
    let s = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let v: SurahTextFile = serde_json::from_str(&s)?;
    Ok(v)
}

pub fn load_surah_text(id: u16) -> Result<SurahTextFile> {
    let fname = if id == 1 { "fatiha.json".to_string() } else { format!("{}.json", id) };
    let path = assets_dir().join("quran").join(fname);
    let s = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let v: SurahTextFile = serde_json::from_str(&s)?;
    Ok(v)
}

pub fn load_translation(lang: &str, surah: u16) -> Result<TranslationFile> {
    let fname = match (lang, surah) {
        ("en", 1) => "en_fatiha.json",
        ("id", 1) => "id_fatiha.json",
        _ => return Err(anyhow::anyhow!("missing sample translation")),
    };
    let path = assets_dir().join("translations").join(fname);
    let s = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let v: TranslationFile = serde_json::from_str(&s)?;
    Ok(v)
}
