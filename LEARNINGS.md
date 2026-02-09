# Session Learnings

This file documents patterns, decisions, and lessons learned while working on the Genesis Universe codebase.

---

## Session Date: 2026-02-09 - show_epoch_info Refactoring

### Gotchas Encountered:
- Orchestrator mode has limited file access - cannot directly use `list_files` or `execute_command`, must delegate to code subagents for file operations
- Initial analysis required understanding the full usage pattern before making changes (found 7 different locations across 3 files)

### Patterns That Work in This Codebase:
- Configuration flow: `genesis.toml` → `Config::load()` → `DisplayConfig` → `OverlayState`
- Three-tier configuration: runtime config file (genesis.toml), struct defaults (Default impls), and runtime resource initialization
- Setting disabled features to `false` rather than removing fields is safer - preserves structure and allows easy re-enablement

### Decisions Made and Why:
- Chose to set `show_epoch_info` to `false` in 3 locations rather than removing the field because:
  - No code would break - the conditional check at `genesis-ui/src/overlay/mod.rs:43` handles false values gracefully
  - Feature can be easily re-enabled for Phase 2+ by changing defaults back to true
  - Preserves the conditional UI rendering logic that will be used in future phases
  - Avoids requiring updates to struct definitions, main.rs initialization, and TOML deserialization

### Task Decomposition Insights:
- Even a "simple" configuration change requires 3 atomic subtasks when the codebase uses three-tier configuration
- Analysis subtask before implementation is valuable for discovering all touchpoints and dependencies
- Each subtask was independently verifiable with `cargo check`
