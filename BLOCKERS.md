# Blockers

## [2026-02-09-TIME-ACCEL] - Time Acceleration Starting Value Uncertainty

**Related TODO:** `fix: Update genesis.toml time.initial_time_acceleration to match PRD Phase 1 starting range`

**Description:** Need guidance on appropriate initial_time_acceleration value for PRD Phase 1. Current value is 1.0 (minimum of 1x to 10¹²x range). No guidance found in archived questions, Sprint 1 decisions, or gap analysis.

**RFI:** `comms/outbox/2026-02-09_time-acceleration-starting-value.md`

**Status:** Resolved

**Resolution:** Set to midpoint value of 1000000000.0 (1.0×10⁹) per RFI response. genesis.toml updated on 2026-02-09.

**Impact:** Blocks completion of critical fix task in Sprint 1. Without clarification on the appropriate starting value, cannot finalize genesis.toml configuration to match PRD Phase 1 specifications. This impacts the time system initialization and user experience when starting the simulation.

---

**No other active blockers.**

---

## Resolved Blockers

### [2026-02-09] - Point Sprite Shader Path Not Found

**Status:** Resolved - See ARCHITECTURE.md "Architectural Decisions Log" (2026-02-09)

**Resolution:** Architectural decision made to recreate `assets/` directory and copy shader file to standard Bevy location. Implementation task added to TODO.md (Sprint 1, Sprint QA section).

### [2026-02-09] - Point Sprite Shader Compilation Error

**Status:** Resolved - See ARCHITECTURE.md "Architectural Decisions Log" (2026-02-09)

**Resolution:** Solution documented in ARCHITECTURE.md. Task added to TODO.md as priority fix: "fix: Resolve ViewUniform shader compilation error". The ViewUniform struct must be defined in the shader file.

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
