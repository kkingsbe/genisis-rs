# Blockers

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
