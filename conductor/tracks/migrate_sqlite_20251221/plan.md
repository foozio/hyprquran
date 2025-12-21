# Track Plan: Migrate to SQLite and Import Full Qur'an Data

This plan outlines the steps to migrate to SQLite and import the full Qur'an.

---

## Phase 1: Data Acquisition and Import

- [x] Task: Download and prepare full Qur'an text (Uthmani) using `download_and_convert.py`
- [x] Task: Execute `tanzil_import` to populate the SQLite database with the full text
- [ ] Task: Conductor - User Manual Verification 'Data Acquisition and Import' (Protocol in workflow.md)

---

## Phase 2: Application Integration

- [ ] Task: Update `Cargo.toml` to enable `sqlite` feature by default
- [ ] Task: Verify `src/data.rs` and `src/ui.rs` correctly load data from SQLite
- [ ] Task: Conductor - User Manual Verification 'Application Integration' (Protocol in workflow.md)
