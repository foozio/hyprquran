use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AyahRef {
    pub surah_id: u16,
    pub ayah_index: u16,
}

#[derive(Clone, Debug)]
pub struct Surah {
    pub id: u16,
    pub name_ar: String,
    pub name_en: String,
    pub ayah_count: u16,
}

#[derive(Default, Clone)]
pub struct AppState {
    pub surahs: Vec<Surah>,
    pub current: AyahRef,
    pub translation_lang: Option<String>,
    pub translations: HashMap<(u16, u16, String), String>,
    pub current_ayat: Vec<String>,
    pub search_query: String,
    pub search_results: Vec<u16>,
    pub bookmarks: Vec<AyahRef>,
    pub prefer_dark: bool,
}

impl AppState {
    pub fn new() -> Self {
        let surahs = vec![Surah {
            id: 1,
            name_ar: "الفاتحة".to_string(),
            name_en: "Al-Fatiha".to_string(),
            ayah_count: 7,
        }];
        Self {
            surahs,
            current: AyahRef { surah_id: 1, ayah_index: 1 },
            translation_lang: None,
            translations: HashMap::new(),
            current_ayat: Vec::new(),
            search_query: String::new(),
            search_results: Vec::new(),
            bookmarks: Vec::new(),
            prefer_dark: false,
        }
    }

    pub fn set_ayat(&mut self, ayat: Vec<String>) {
        self.current_ayat = ayat;
    }

    pub fn run_search(&mut self, query: &str) {
        self.search_query = query.to_string();
        if query.is_empty() {
            self.search_results.clear();
            return;
        }
        let q = query;
        let mut results = Vec::new();
        for (i, a) in self.current_ayat.iter().enumerate() {
            if a.contains(q) {
                results.push((i + 1) as u16);
            }
        }
        self.search_results = results;
    }

    pub fn add_bookmark(&mut self, b: AyahRef) {
        if !self.bookmarks.iter().any(|x| x == &b) {
            self.bookmarks.push(b);
        }
    }
}
