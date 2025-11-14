#![cfg(feature = "sqlite")]
use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::{params, Connection, OptionalExtension};

pub fn db_path() -> Result<std::path::PathBuf> {
    let dirs = ProjectDirs::from("org", "hyprquran", "HyprQuran").ok_or_else(|| anyhow::anyhow!("dirs"))?;
    let dir = dirs.data_dir();
    std::fs::create_dir_all(dir)?;
    Ok(dir.join("quran.db"))
}

pub fn open() -> Result<Connection> {
    let path = db_path()?;
    let conn = Connection::open(path)?;
    Ok(conn)
}

pub fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        PRAGMA foreign_keys=ON;
        CREATE TABLE IF NOT EXISTS surah (
            surah_id INTEGER PRIMARY KEY,
            name_arabic TEXT NOT NULL,
            name_english TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS ayah (
            ayah_id INTEGER PRIMARY KEY,
            surah_id INTEGER NOT NULL REFERENCES surah(surah_id) ON DELETE CASCADE,
            ayah_number INTEGER NOT NULL,
            text_uthmani TEXT NOT NULL,
            UNIQUE(surah_id, ayah_number)
        );
        CREATE TABLE IF NOT EXISTS translation (
            trans_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            language TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS translated_ayah (
            ayah_id INTEGER NOT NULL REFERENCES ayah(ayah_id) ON DELETE CASCADE,
            trans_id INTEGER NOT NULL REFERENCES translation(trans_id) ON DELETE CASCADE,
            text TEXT NOT NULL,
            PRIMARY KEY(ayah_id, trans_id)
        );
        "#,
    )?;
    Ok(())
}

pub fn upsert_surah(conn: &Connection, surah_id: u16, name_ar: &str, name_en: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO surah(surah_id,name_arabic,name_english) VALUES(?,?,?) \
         ON CONFLICT(surah_id) DO UPDATE SET name_arabic=excluded.name_arabic, name_english=excluded.name_english",
        params![surah_id as i64, name_ar, name_en],
    )?;
    Ok(())
}

pub fn upsert_ayah(conn: &Connection, surah_id: u16, ayah_number: u16, text: &str) -> Result<i64> {
    let ayah_id: i64 = (surah_id as i64) * 1000 + (ayah_number as i64);
    conn.execute(
        "INSERT INTO ayah(ayah_id,surah_id,ayah_number,text_uthmani) VALUES(?,?,?,?) \
         ON CONFLICT(ayah_id) DO UPDATE SET text_uthmani=excluded.text_uthmani",
        params![ayah_id, surah_id as i64, ayah_number as i64, text],
    )?;
    Ok(ayah_id)
}

pub fn upsert_translation(conn: &Connection, name: &str, language: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO translation(name,language) VALUES(?,?)",
        params![name, language],
    )?;
    let id = conn.last_insert_rowid();
    Ok(id)
}

pub fn upsert_translated_ayah(conn: &Connection, ayah_id: i64, trans_id: i64, text: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO translated_ayah(ayah_id,trans_id,text) VALUES(?,?,?) \
         ON CONFLICT(ayah_id,trans_id) DO UPDATE SET text=excluded.text",
        params![ayah_id, trans_id, text],
    )?;
    Ok(())
}

pub fn get_surah(conn: &Connection, surah_id: u16) -> Result<Option<(String, String)>> {
    conn.query_row(
        "SELECT name_arabic,name_english FROM surah WHERE surah_id=?",
        params![surah_id as i64],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )
    .optional()
    .map_err(Into::into)
}

pub fn get_surah_list(conn: &Connection) -> Result<Vec<(u16, String, String)>> {
    let mut stmt = conn.prepare("SELECT surah_id,name_arabic,name_english FROM surah ORDER BY surah_id")?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)? as u16, row.get(1)?, row.get(2)?)))?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}

pub fn get_ayat(conn: &Connection, surah_id: u16) -> Result<Vec<(u16, String)>> {
    let mut stmt = conn.prepare("SELECT ayah_number,text_uthmani FROM ayah WHERE surah_id=? ORDER BY ayah_number")?;
    let rows = stmt.query_map(params![surah_id as i64], |row| Ok((row.get::<_, i64>(0)? as u16, row.get(1)?)))?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}

pub fn get_translation_for_ayah(conn: &Connection, surah_id: u16, ayah_number: u16, language: &str) -> Result<Option<String>> {
    let ayah_id: i64 = (surah_id as i64) * 1000 + (ayah_number as i64);
    conn.query_row(
        "SELECT ta.text FROM translated_ayah ta JOIN translation t ON ta.trans_id=t.trans_id WHERE ta.ayah_id=? AND t.language=?",
        params![ayah_id, language],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(Into::into)
}

pub fn search_surah_translation_ayahs(conn: &Connection, surah_id: u16, language: &str, query: &str) -> Result<Vec<u16>> {
    let ayah_prefix: i64 = (surah_id as i64) * 1000;
    let mut stmt = conn.prepare(
        "SELECT ta.ayah_id FROM translated_ayah ta JOIN translation t ON ta.trans_id=t.trans_id WHERE ta.ayah_id BETWEEN ? AND ? AND t.language=? AND ta.text LIKE '%' || ? || '%' ORDER BY ta.ayah_id",
    )?;
    let rows = stmt.query_map(params![ayah_prefix, ayah_prefix + 999, language, query], |row| Ok(row.get::<_, i64>(0)?))?;
    let mut out = Vec::new();
    for r in rows { let id: i64 = r?; out.push((id % 1000) as u16); }
    Ok(out)
}

pub fn get_available_translations(conn: &Connection) -> Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare("SELECT language, name FROM translation ORDER BY language")?;
    let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}
