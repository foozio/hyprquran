use anyhow::Result;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

fn bundled_fonts_dir() -> PathBuf {
    crate::data::assets_dir().join("fonts")
}

#[cfg(all(feature = "gui", target_os = "linux"))]
pub fn register_bundled_fonts() -> Result<()> {
    let src = bundled_fonts_dir();
    if !src.exists() { return Ok(()); }
    let dirs = ProjectDirs::from("org", "hyprquran", "HyprQuran").ok_or_else(|| anyhow::anyhow!("dirs"))?;
    let dst = dirs.data_dir().join("fonts");
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(&src)? {
        let e = entry?;
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("ttf")).unwrap_or(false) {
            let fname = p.file_name().unwrap();
            fs::copy(&p, dst.join(fname))?;
        }
    }
    let _ = std::process::Command::new("fc-cache").arg("-f").arg(&dst).status();
    Ok(())
}

#[cfg(not(all(feature = "gui", target_os = "linux")))]
pub fn register_bundled_fonts() -> Result<()> { Ok(()) }

pub fn prefer_font_family() -> String {
    // Prefer Amiri Quran when bundled
    let dir = bundled_fonts_dir();
    let amiri = dir.join("AmiriQuran.ttf");
    if amiri.exists() { return "Amiri Quran".to_string(); }
    "Amiri Quran".to_string()
}
