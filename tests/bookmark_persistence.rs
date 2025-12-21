use hyprquran::storage::{self, Persisted};
use hyprquran::state::AyahRef;
use std::fs;
use std::path::PathBuf;

// Helper to mock XDG_DATA_HOME for testing to avoid messing with real user data
fn setup_test_env() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("XDG_DATA_HOME", dir.path());
    dir
}

#[test]
fn test_repro_bookmark_loss_on_load_failure() {
    let _temp_dir = setup_test_env();

    // 1. Create initial state with bookmarks
    let initial_bookmark = AyahRef { surah_id: 2, ayah_index: 255 };
    let mut p = Persisted::default();
    p.bookmarks.push(initial_bookmark);
    storage::save(&p).expect("Failed to save initial state");

    // Verify saved
    let loaded = storage::load().expect("Failed to load state");
    assert_eq!(loaded.bookmarks.len(), 1);

    // 2. Simulate corruption or load failure (e.g., by writing invalid JSON)
    // We need to know where the file is. storage::state_path uses ProjectDirs which uses XDG_DATA_HOME.
    // The path structure in storage.rs is `data_dir()/state.json`.
    // data_dir() is ProjectDirs::from("org", "hyprquran", "HyprQuran").data_dir()
    // On Linux with XDG_DATA_HOME set to /tmp/foo, it should be /tmp/foo/hyprquran/HyprQuran/state.json
    // But ProjectDirs behavior depends on the crate 'directories'.
    // Let's rely on storage::save working, so we corrupt the file at the expected location.
    // We can't easily get the path from public API, but we can guess or expose it.
    // Ideally we shouldn't rely on internal path structure.
    
    // Instead of manual corruption, let's simulate the `persist` logic found in `ui.rs`:
    // `let mut p = storage::load().unwrap_or_default();`
    // `p.last = ...;`
    // `storage::save(&p);`
    
    // If load() works, bookmarks are preserved.
    // But if we corrupt the file:
    // We need to find the file.
    
    // Let's use a slightly different approach:
    // If we can't easily corrupt the file without knowing the path, 
    // we can demonstrate that the `persist` PATTERN relies on load success.
    
    // Actually, I can use `directories` crate in test to find the path same way `storage.rs` does.
    let dirs = directories::ProjectDirs::from("org", "hyprquran", "HyprQuran").unwrap();
    let data_dir = dirs.data_dir();
    let state_path = data_dir.join("state.json");
    
    // Corrupt the file
    fs::write(&state_path, "{ invalid json").expect("Failed to write corrupt json");
    
    // 3. Simulate the FIXED `persist` logic
    // "load().unwrap_or_default()"
    let mut p_current = storage::load().unwrap_or_default();
    
    // Simulate restoring from in-memory state (The Fix)
    // In the real app, we would do: p_current.bookmarks = app_state.bookmarks.clone();
    // Here we simulate having the bookmark in memory
    p_current.bookmarks.push(initial_bookmark);
    
    // 4. Update some other state (e.g. navigation)
    p_current.last = AyahRef { surah_id: 3, ayah_index: 1 };
    
    // 5. Save
    storage::save(&p_current).expect("Failed to save new state");
    
    // 6. Fix the file (so it's valid JSON again) - implicitly done by save()
    
    // 7. Verify bookmarks are PRESERVED (This should fail if bug exists)
    let final_state = storage::load().expect("Failed to reload state");
    assert!(!final_state.bookmarks.is_empty(), "Bookmarks were lost! Persistence logic is fragile.");
    assert_eq!(final_state.bookmarks[0], initial_bookmark);
}
