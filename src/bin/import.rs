#![cfg(feature = "sqlite")]
use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
struct SurahMeta { id: u16, name_ar: &'static str, name_en: &'static str }

fn default_surahs() -> Vec<SurahMeta> {
    let names_en = [
        "Al-Fatiha","Al-Baqarah","Ali 'Imran","An-Nisa","Al-Ma'idah","Al-An'am","Al-A'raf","Al-Anfal","At-Tawbah","Yunus","Hud","Yusuf","Ar-Ra'd","Ibrahim","Al-Hijr","An-Nahl","Al-Isra","Al-Kahf","Maryam","Ta-Ha","Al-Anbiya","Al-Hajj","Al-Mu'minun","An-Nur","Al-Furqan","Ash-Shu'ara","An-Naml","Al-Qasas","Al-'Ankabut","Ar-Rum","Luqman","As-Sajdah","Al-Ahzab","Saba'","Fatir","Ya-Sin","As-Saffat","Sad","Az-Zumar","Ghafir","Fussilat","Ash-Shura","Az-Zukhruf","Ad-Dukhan","Al-Jathiyah","Al-Ahqaf","Muhammad","Al-Fath","Al-Hujurat","Qaf","Adh-Dhariyat","At-Tur","An-Najm","Al-Qamar","Ar-Rahman","Al-Waqi'ah","Al-Hadid","Al-Mujadila","Al-Hashr","Al-Mumtahanah","As-Saff","Al-Jumu'ah","Al-Munafiqun","At-Taghabun","At-Talaq","At-Tahrim","Al-Mulk","Al-Qalam","Al-Haqqah","Al-Ma'arij","Nuh","Al-Jinn","Al-Muzzammil","Al-Muddathir","Al-Qiyamah","Al-Insan","Al-Mursalat","An-Naba","An-Nazi'at","Abasa","At-Takwir","Al-Infitar","Al-Mutaffifin","Al-Inshiqaq","Al-Buruj","At-Tariq","Al-A'la","Al-Ghashiyah","Al-Fajr","Al-Balad","Ash-Shams","Al-Layl","Ad-Duha","Ash-Sharh","At-Tin","Al-'Alaq","Al-Qadr","Al-Bayyinah","Az-Zalzalah","Al-'Adiyat","Al-Qari'ah","At-Takathur","Al-'Asr","Al-Humazah","Al-Fil","Quraysh","Al-Ma'un","Al-Kawthar","Al-Kafirun","An-Nasr","Al-Masad","Al-Ikhlas","Al-Falaq","An-Nas"
    ];
    let names_ar = [
        "الفاتحة","البقرة","آل عمران","النساء","المائدة","الأنعام","الأعراف","الأنفال","التوبة","يونس","هود","يوسف","الرعد","إبراهيم","الحجر","النحل","الإسراء","الكهف","مريم","طه","الأنبياء","الحج","المؤمنون","النور","الفرقان","الشعراء","النمل","القصص","العنكبوت","الروم","لقمان","السجدة","الأحزاب","سبإ","فاطر","يس","الصافات","ص","الزمر","غافر","فصلت","الشورى","الزخرف","الدخان","الجاثية","الأحقاف","محمد","الفتح","الحجرات","ق","الذاريات","الطور","النجم","القمر","الرحمن","الواقعة","الحديد","المجادلة","الحشر","الممتحنة","الصف","الجمعة","المنافقون","التغابن","الطلاق","التحريم","الملك","القلم","الحاقة","المعارج","نوح","الجن","المزمل","المدثر","القيامة","الإنسان","المرسلات","النبإ","النازعات","عبس","التكوير","الإنفطار","المطففين","الإنشقاق","البروج","الطارق","الأعلى","الغاشية","الفجر","البلد","الشمس","الليل","الضحى","الشرح","التين","العلق","القدر","البينة","الزلزلة","العاديات","القارعة","التكاثر","العصر","الهمزة","الفيل","قريش","الماعون","الكوثر","الكافرون","النصر","المسد","الإخلاص","الفلق","الناس"
    ];
    (1..=114).map(|i| SurahMeta { id: i, name_ar: names_ar[(i-1) as usize], name_en: names_en[(i-1) as usize] }).collect()
}

#[derive(Deserialize)]
struct SurahTextFile { surah: u16, name_ar: String, name_en: String, ayat: Vec<String> }
#[derive(Deserialize)]
struct TranslationFile { lang: String, entries: Vec<TranslationEntry> }
#[derive(Deserialize)]
struct TranslationEntry { surah: u16, ayah: u16, text: String }

fn assets_dir() -> PathBuf { PathBuf::from("assets") }

fn db_path() -> Result<std::path::PathBuf> {
    let dirs = ProjectDirs::from("org", "hyprquran", "HyprQuran").ok_or_else(|| anyhow::anyhow!("dirs"))?;
    let dir = dirs.data_dir();
    std::fs::create_dir_all(dir)?;
    Ok(dir.join("quran.db"))
}

fn open() -> Result<Connection> { Ok(Connection::open(db_path()?)?) }

fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(r#"
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
    "#)?;
    Ok(())
}

fn main() -> Result<()> {
    let conn = open()?;
    init_schema(&conn)?;

    for s in default_surahs() {
        conn.execute(
            "INSERT INTO surah(surah_id,name_arabic,name_english) VALUES(?,?,?) \
             ON CONFLICT(surah_id) DO UPDATE SET name_arabic=excluded.name_arabic,name_english=excluded.name_english",
            params![s.id as i64, s.name_ar, s.name_en],
        )?;
    }

    let f = fs::read_to_string(assets_dir().join("quran").join("fatiha.json"))?;
    let f: SurahTextFile = serde_json::from_str(&f)?;
    for (i, t) in f.ayat.iter().enumerate() {
        let ayah_id: i64 = (1_i64) * 1000 + ((i + 1) as i64);
        conn.execute(
            "INSERT INTO ayah(ayah_id,surah_id,ayah_number,text_uthmani) VALUES(?,?,?,?) \
             ON CONFLICT(ayah_id) DO UPDATE SET text_uthmani=excluded.text_uthmani",
            params![ayah_id, 1_i64, (i + 1) as i64, t],
        )?;
    }

    conn.execute("INSERT INTO translation(name,language) VALUES(?,?)", params!["Sample EN", "en"])?;
    let en_id = conn.last_insert_rowid();
    let en = fs::read_to_string(assets_dir().join("translations").join("en_fatiha.json"))?;
    let en: TranslationFile = serde_json::from_str(&en)?;
    for e in en.entries {
        let ayah_id: i64 = (e.surah as i64) * 1000 + (e.ayah as i64);
        conn.execute(
            "INSERT INTO translated_ayah(ayah_id,trans_id,text) VALUES(?,?,?) \
             ON CONFLICT(ayah_id,trans_id) DO UPDATE SET text=excluded.text",
            params![ayah_id, en_id, e.text],
        )?;
    }

    conn.execute("INSERT INTO translation(name,language) VALUES(?,?)", params!["Sample ID", "id"])?;
    let id_id = conn.last_insert_rowid();
    let id = fs::read_to_string(assets_dir().join("translations").join("id_fatiha.json"))?;
    let id: TranslationFile = serde_json::from_str(&id)?;
    for e in id.entries {
        let ayah_id: i64 = (e.surah as i64) * 1000 + (e.ayah as i64);
        conn.execute(
            "INSERT INTO translated_ayah(ayah_id,trans_id,text) VALUES(?,?,?) \
             ON CONFLICT(ayah_id,trans_id) DO UPDATE SET text=excluded.text",
            params![ayah_id, id_id, e.text],
        )?;
    }

    println!("Seeded SQLite DB at {:?}", db_path()?);
    Ok(())
}
