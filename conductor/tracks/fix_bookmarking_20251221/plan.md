# Track Plan: Fix Ayah Bookmarking Persistence

This plan outlines the steps to resolve the bookmarking persistence bug.

---

## Phase 1: Investigation & Infrastructure

- [ ] Task: Audit existing bookmarking logic in `src/state.rs` and `src/storage.rs`
- [ ] Task: Create a failing unit test that reproduces the bookmark persistence failure
- [ ] Task: Ensure XDG paths are correctly initialized for bookmark storage
- [ ] Task: Conductor - User Manual Verification 'Investigation & Infrastructure' (Protocol in workflow.md)

---

## Phase 2: Implementation & Verification

- [ ] Task: Fix bookmark saving logic to trigger on setiap bookmark action
- [ ] Task: Fix bookmark loading logic to populate state on startup
- [ ] Task: Implement a safeguard to prevent state updates from overwriting bookmarks incorrectly
- [ ] Task: Verify fix with unit tests (Green phase)
- [ ] Task: Conductor - User Manual Verification 'Implementation & Verification' (Protocol in workflow.md)
