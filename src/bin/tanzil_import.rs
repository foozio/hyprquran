#![cfg(feature = "sqlite")]
use anyhow::{anyhow, Result};
use rusqlite::params;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(line: &str) -> Option<(u16, u16, String)> {
    let l = line.trim_start_matches('\u{feff}');
    if l.is_empty() { return None; }
    if let Some((a, b)) = l.split_once('|') {
        let mut parts = a.split('|');
        let s = parts.next()?.trim().parse::<u16>().ok()?;
        let y = parts.next()?.trim().parse::<u16>().ok()?;
        return Some((s, y, b.trim().to_string()));
    }
    let parts: Vec<&str> = l.split('\t').collect();
    if parts.len() >= 3 {
        let s = parts[0].trim().parse::<u16>().ok()?;
        let y = parts[1].trim().parse::<u16>().ok()?;
        let t = parts[2..].join("\t");
        return Some((s, y, t.trim().to_string()));
    }
    None
}

fn default_surahs() -> Vec<(u16, &'static str, &'static str)> {
    let names_en = [
        "Al-Fatiha","Al-Baqarah","Ali 'Imran","An-Nisa","Al-Ma'idah","Al-An'am","Al-A'raf","Al-Anfal","At-Tawbah","Yunus","Hud","Yusuf","Ar-Ra'd","Ibrahim","Al-Hijr","An-Nahl","Al-Isra","Al-Kahf","Maryam","Ta-Ha","Al-Anbiya","Al-Hajj","Al-Mu'minun","An-Nur","Al-Furqan","Ash-Shu'ara","An-Naml","Al-Qasas","Al-'Ankabut","Ar-Rum","Luqman","As-Sajdah","Al-Ahzab","Saba'","Fatir","Ya-Sin","As-Saffat","Sad","Az-Zumar","Ghafir","Fussilat","Ash-Shura","Az-Zukhruf","Ad-Dukhan","Al-Jathiyah","Al-Ahqaf","Muhammad","Al-Fath","Al-Hujurat","Qaf","Adh-Dhariyat","At-Tur","An-Najm","Al-Qamar","Ar-Rahman","Al-Waqi'ah","Al-Hadid","Al-Mujadila","Al-Hashr","Al-Mumtahanah","As-Saff","Al-Jumu'ah","Al-Munafiqun","At-Taghabun","At-Talaq","At-Tahrim","Al-Mulk","Al-Qalam","Al-Haqqah","Al-Ma'arij","Nuh","Al-Jinn","Al-Muzzammil","Al-Muddathir","Al-Qiyamah","Al-Insan","Al-Mursalat","An-Naba","An-Nazi'at","Abasa","At-Takwir","Al-Infitar","Al-Mutaffifin","Al-Inshiqaq","Al-Buruj","At-Tariq","Al-A'la","Al-Ghashiyah","Al-Fajr","Al-Balad","Ash-Shams","Al-Layl","Ad-Duha","Ash-Sharh","At-Tin","Al-'Alaq","Al-Qadr","Al-Bayyinah","Az-Zalzalah","Al-'Adiyat","Al-Qari'ah","At-Takathur","Al-'Asr","Al-Humazah","Al-Fil","Quraysh","Al-Ma'un","Al-Kawthar","Al-Kafirun","An-Nasr","Al-Masad","Al-Ikhlas","Al-Falaq","An-Nas"
    ];
    let names_ar = [
        "الفاتحة","البقرة","آل عمران","النساء","المائدة","الأنعام","الأعراف","الأنفال","التوبة","يونس","هود","يوسف","الرعد","إبراهيم","الحجر","النحل","الإسراء","الكهف","مريم","طه","الأنبياء","الحج","المؤمنون","النور","الفرقان","الشعراء","النمل","القصص","العنكبوت","الروم","لقمان","السجدة","الأحزاب","سبإ","فاطر","يس","الصافات","ص","الزمر","غافر","فصلت","الشورى","الزخرف","الدخان","الجاثية","الأحقاف","محمد","الفتح","الحجرات","ق","الذاريات","الطور","النجم","القمر","الرحمن","الواقعة","الحديد","المجادلة","الحشر","الممتحنة","الصف","الجمعة","المنافقون","التغابن","الطلاق","التحريم","الملك","القلم","الحاقة","المعارج","نوح","الجن","المزمل","المدثر","القيامة","الإنسان","المرسلات","النبإ","النازعات","عبس","التكوير","الإنفطار","المطففين","الإنشقاق","البروج","الطارق","الأعلى","الغاشية","الفجر","البلد","الشمس","الليل","الضحى","الشرح","التين","العلق","القدر","البينة","الزلزلة","العاديات","القارعة","التكاثر","العصر","الهمزة","الفيل","قريش","الماعون","الكوثر","الكافرون","النصر","المسد","الإخلاص","الفلق","الناس"
    ];
    (1..=114).map(|i| (i, names_ar[(i-1) as usize], names_en[(i-1) as usize])).collect()
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let mut text_path: Option<String> = None;
    let mut translations: Vec<(String, String, String)> = Vec::new();
    while let Some(a) = args.next() {
        match a.as_str() {
            "--text" => { text_path = args.next(); }
            "--translation" => {
                let lang = args.next().ok_or_else(|| anyhow!("lang"))?;
                let name = args.next().ok_or_else(|| anyhow!("name"))?;
                let path = args.next().ok_or_else(|| anyhow!("path"))?;
                translations.push((lang, name, path));
            }
            _ => {}
        }
    }

    let conn = hyprquran::db::open()?;
    hyprquran::db::init_schema(&conn)?;
    for (id, ar, en) in default_surahs() {
        hyprquran::db::upsert_surah(&conn, id, ar, en)?;
    }
    if let Some(p) = text_path {
        let f = File::open(p)?;
        let rdr = BufReader::new(f);
        for line in rdr.lines() {
            let l = line?;
            if let Some((s, y, t)) = parse_line(&l) {
                let _ = hyprquran::db::upsert_ayah(&conn, s, y, &t)?;
            }
        }
    }
    for (lang, name, path) in translations {
        conn.execute("INSERT INTO translation(name,language) VALUES(?,?)", params![name, lang])?;
        let trans_id = conn.last_insert_rowid();
        let f = File::open(path)?;
        let rdr = BufReader::new(f);
        for line in rdr.lines() {
            let l = line?;
            if let Some((s, y, t)) = parse_line(&l) {
                let ayah_id: i64 = (s as i64) * 1000 + (y as i64);
                conn.execute(
                    "INSERT INTO translated_ayah(ayah_id,trans_id,text) VALUES(?,?,?) ON CONFLICT(ayah_id,trans_id) DO UPDATE SET text=excluded.text",
                    params![ayah_id, trans_id, t],
                )?;
            }
        }
    }
    Ok(())
}