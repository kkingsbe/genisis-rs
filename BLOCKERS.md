# Blockers

**No active blockers.**

---

## Resolved Blockers

### [2026-02-09] - Time Acceleration Starting Value Uncertainty

**Status:** Resolved

**Resolution:** Set to midpoint value of 1000000000.0 (1.0×10⁹) per RFI response. genesis.toml updated on 2026-02-09.

### [2026-02-09] - Point Sprite Shader Path Not Found

**Status:** Resolved - See ARCHITECTURE.md "Architectural Decisions Log" (2026-02-09)

**Resolution:** Architectural decision made to recreate `assets/` directory and copy shader file to standard Bevy location. Implementation task added to TODO.md (Sprint 1, Sprint QA section).

### [2026-02-09] - Point Sprite Shader Compilation Error

**Status:** Resolved - See ARCHITECTURE.md "Architectural Decisions Log" (2026-02-09)

**Resolution:** Solution documented in ARCHITECTURE.md. Task added to TODO.md as priority fix: "fix: Resolve ViewUniform shader compilation error". The ViewUniform struct must be defined in the shader file.

### [2026-02-10] - Failing integration tests require GPU access

**Status:** Resolved - See ARCHITECTURE.md "Architectural Decisions Log" (2026-02-10)

**Resolution:** Architectural decision made to use dummy handles (`Handle::default()`) for tests that can be validated without GPU resources, and mark GPU-dependent tests with `#[ignore]`. Tests have been modified accordingly. When GPU access becomes available in CI, the ignored tests can be re-enabled.

### [2026-02-10] - Build Error: bind_group_layout trait method in ParticleMaterial

**Status:** Resolved - No longer present

**Resolution:** This compilation error no longer exists. The build completes successfully (verified 2026-02-10). The PointSpriteMaterial implementation correctly implements the Material trait without the bind_group_layout method.

## Format for New Blockers

When reporting a blocker, use the following format:

```markdown
### [Date] - Blocker Title

**Severity:** High/Medium/Low

**Description:**
[Detailed description of the blocker]

**Impact:**
[What tasks/features are blocked by this issue]

**Possible Approaches:**
- [ ] Approach 1
- [ ] Approach 2
- [ ] Approach 3

**Status:** Open/In Review/Resolved
```
