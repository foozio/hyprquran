use rusqlite::Connection;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    let db_path = format!("{}/.local/share/hyprquran/quran.db", home);
    println!("Checking database at: {}", db_path);
    
    let conn = Connection::open(db_path)?;
    
    // Check surah count
    let count: i64 = conn.query_row("SELECT count(*) FROM surah", [], |row| row.get(0))?;
    println!("Surahs count: {}", count);
    
    if count > 0 {
        // Check first few surahs
        let mut stmt = conn.prepare("SELECT surah_id, name_english FROM surah LIMIT 5")?;
        let rows = stmt.query_map([], |row| {
             Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            println!("Surah: {:?}", row?);
        }
    }

    // Check ayah count
    let ayah_count: i64 = conn.query_row("SELECT count(*) FROM ayah", [], |row| row.get(0))?;
    println!("Ayahs count: {}", ayah_count);
    
    if ayah_count > 0 {
         // Check a few ayahs from Surah 2 (Al-Baqarah)
        let mut stmt = conn.prepare("SELECT ayah_number, substr(text_uthmani, 1, 50) FROM ayah WHERE surah_id = 2 LIMIT 3")?;
        let rows = stmt.query_map([], |row| {
             Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            println!("Ayah (Al-Baqarah): {:?}", row?);
        }
    }

    Ok(())
}
