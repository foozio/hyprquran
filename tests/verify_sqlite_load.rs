use hyprquran::data;

#[test]
fn test_load_surah_baqarah_from_sqlite() {
    // This test assumes the DB is populated (which we did in Phase 1)
    // and that the test environment has access to the user's data dir
    // OR we need to mock XDG_DATA_HOME to point to where we put the DB.
    
    // In Phase 1, we put the DB in ~/.local/share/hyprquran/quran.db (real user path).
    // `cargo test` runs in the same environment.
    
    // We expect this to SUCCEED and return Al-Baqarah.
    let surah = data::load_surah_text(2).expect("Failed to load Surah Al-Baqarah");
    
    assert_eq!(surah.surah, 2);
    assert_eq!(surah.name_en, "Al-Baqarah");
    assert!(surah.ayat.len() > 100); // Al-Baqarah has 286 verses
}
