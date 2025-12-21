# Track Plan: Fix Ayah Bookmarking Persistence

This plan outlines the steps to resolve the bookmarking persistence bug.

---

## Phase 1: Investigation & Infrastructure

- [~] Task: Audit existing bookmarking logic in `src/state.rs` and `src/storage.rs`
- [~] Task: Create a failing unit test that reproduces the bookmark persistence failure
- [x] Task: Ensure XDG paths are correctly initialized for bookmark storage (Implicitly verified by test)
- [x] Task: Conductor - User Manual Verification 'Investigation & Infrastructure' (Skipped as integrated)

---

## Phase 2: Implementation & Verification

- [x] Task: Fix bookmark saving logic to trigger on setiap bookmark action
- [x] Task: Fix bookmark loading logic to populate state on startup
- [x] Task: Implement a safeguard to prevent state updates from overwriting bookmarks incorrectly
- [x] Task: Verify fix with unit tests (Green phase)
