# Session Learnings

This file documents patterns, decisions, and lessons learned while working on the Genesis Universe codebase.

---

## Session Date: 2026-02-10 - Bytemuck Zeroable Import Fix

### Gotchas Encountered:
- The `Zeroable` trait from `bytemuck` crate must be explicitly imported for the `zeroed()` method to work
- `bytemuck::Pod` does not automatically include `Zeroable` - both traits must be imported separately if needed
- The compilation error was specific to the test code at line 315, not the production code

### Patterns That Work in This Codebase:
- Bevy's GPU instancing uses `bytemuck` crate for zero-copy buffer data conversion
- The `zeroed()` method is useful for initializing GPU-compatible buffer data structures
- `Cargo check --package <crate-name>` is the appropriate way to verify compilation errors for a specific crate

### Decisions Made and Why:
- Added the import at line 30 (top-level imports section) rather than inline - follows Rust convention for imports
- Fixed this blocking issue first before addressing the failing tests - following "fix compilation errors before tests" priority

### Key Bevy/wgpu Patterns:
- GPU buffers require traits like `Pod` and `Zeroable` from `bytemuck` for safe memory layout conversion
- Storage buffers in WGSL require properly aligned, repr(C) structs in Rust

---

## Session Date: 2026-02-09 - Test Suite Analysis (Bevy 0.15 Migration)

### Gotchas Encountered:
- Previous `cargo test` output showed "running 0 tests" which was misleading - tests were discovered but failed to compile
- Integration tests in `tests/` directory are automatically discovered by Cargo but won't execute if they have compilation errors
- Test configuration was correct (dev-dependencies in Cargo.toml), but the API migration issues prevented compilation
- The 66 compilation errors span both test files with consistent patterns indicating a single Bevy 0.15 API migration task

### Patterns That Work in This Codebase:
- Test file placement: `genesis-render/tests/` contains integration tests that are auto-discovered by Cargo
- Test structure: Both resource_binding_tests.rs (1487 lines) and shader_tests.rs (906 lines) follow consistent patterns with:
  - `#[test]` annotations on test functions
  - Helper functions for test setup
  - Clear documentation sections explaining what each test validates
- Test organization: Tests are grouped by category (Pipeline Layout, Resource Initialization, Shader Asset Loading, etc.)

### Decisions Made and Why:
- Identified that the test suite IS properly configured - the issue is solely Bevy 0.15 API migration, not missing configuration
- Updated TODO.md to reflect that "Run all tests to verify current state" is complete with documented findings
- Consolidated the duplicate entries for Bevy 0.15 API migration into a single task with complete analysis

### Key Bevy 0.15 API Changes Impacting Tests:
1. **ScheduleRunnerSettings removed**: `ScheduleRunnerPlugin::run_once()` no longer takes arguments
   - Old: `ScheduleRunnerPlugin::run_once(ScheduleRunnerSettings { wait_for_events: false })`
   - New: `ScheduleRunnerPlugin::run_once()`
2. **Camera3d renamed to Camera**: Bevy 0.15 unified camera types
3. **World::add_systems() removed**: System registration now only works on `App`, not `World`
4. **Entities::iter() removed**: Entity iteration pattern changed in Bevy 0.15
5. **Mesh attribute constants renamed**: `Mesh::ATTRIBUTE_COLOR_0` → `Mesh::ATTRIBUTE_COLOR`
6. **Color API reorganization**: `Color::RED` moved to `bevy::color::palettes::css::RED`
7. **AssetPath API changed**: `AssetPath::to_str()` → `AssetPath::to_string()`
8. **Removed methods**: `LinearRgba::is_normalized()` no longer exists
9. **Type inference changes**: `next_power_of_two()` now requires explicit type annotations
10. **Trait imports**: `Zeroable` trait must be explicitly imported for `zeroed()` calls

### Task Decomposition Insights:
- Running `cargo test --package genesis-render -- -v 2>&1` provides the most complete error output for diagnosis
- Test analysis should verify both existence AND compilation status before declaring tests "not running"
- A single API migration (Bevy 0.14 → 0.15) can manifest as many seemingly-unrelated compilation errors
- Documenting specific line numbers where each API change is needed significantly speeds up the migration task

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
