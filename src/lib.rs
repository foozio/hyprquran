pub mod data;
pub mod state;
pub mod storage;
pub mod logging;
pub mod surah_index;
#[cfg(feature = "gui")]
pub mod app;
#[cfg(feature = "gui")]
pub mod ui;
#[cfg(feature = "sqlite")]
pub mod db;

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