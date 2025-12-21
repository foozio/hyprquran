#[cfg(feature = "gui")]
pub mod app;
pub mod data;
#[cfg(feature = "sqlite")]
pub mod db;
pub mod logging;
pub mod state;
pub mod storage;
pub mod surah_index;
#[cfg(feature = "gui")]
pub mod ui;
pub mod fonts;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surah_index() {
        let surahs = surah_index::default_surahs();
        assert!(!surahs.is_empty());
        assert_eq!(surahs.len(), 114);
    }
}