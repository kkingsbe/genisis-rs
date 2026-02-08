# Question: Validation Criteria and Technical Feasibility Concerns

**Context:** Task 4 - PRD Ambiguity Check

## Issues Identified

### 1. Ambiguous Validation Criteria

Several success metrics in Section 10.2 use vague language that makes objective assessment difficult:

#### A. CMB Power Spectrum (Line 336)
**Requirement:** "CMB power spectrum shape qualitatively matches Planck data at ℓ < 1000"

**Ambiguity:**
- What does "qualitatively matches" mean?
- Should we match within 10%? 20%? Some other tolerance?
- Is this a visual check or an automated test?
- Which Planck data product should we compare against?

#### B. Helium Abundance (Line 335)
**Requirement:** "Primordial helium abundance within 5% of observed value (Y_p ≈ 0.245)"

**Question:** Is this tolerance acceptable given the approximations (Friedmann equations, simplified nucleosynthesis network)?

#### C. Community Adoption (Line 338)
**Requirement:** "500+ GitHub stars within 6 months of release"

**Concern:** This is a marketing/metric, not a technical requirement. Should this be removed from success metrics?

### 2. Technical Feasibility Concerns

#### A. Memory Budget Constraints (Section 8, Line 300)
**Requirement:** "<4 GB VRAM for Real-Time Mode (1M – 10M particles)"

**Feasibility Concern:**
- 10M particles × (position + velocity + mass + temperature + color) ≈ 10M × 64 bytes minimum = 640 MB just for particle data
- Add spatial index (octree/BVH) for 10M particles: potentially 200–400 MB
- Add frame buffers, textures, compute buffers, UI overhead: ~1–2 GB
- **Total estimated: 3–4 GB, leaving almost no margin**

**Question:** Is the 4 GB VRAM target realistic, or should we:
- Lower the particle count target?
- Allow for higher VRAM usage?
- Implement aggressive disk streaming?

#### B. GPU Compute Portability (Section 9, Line 312)
**Requirement:** "GPU compute shader portability" with wgpu abstraction

**Feasibility Concerns:**
- wgpu support is still evolving; certain features may not work on all platforms
- Apple Silicon (M1/M2/M3) has specific limitations in WebGPU implementations
- Intel integrated graphics may struggle with compute shaders

**Question:** What fallback strategies are acceptable?
- CPU fallback for critical paths (as mentioned in mitigation)?
- Which features can we sacrifice on lower-end hardware?
- Should we establish a "supported platforms" matrix?

#### C. Stiff ODE Solver (Phase 3, Line 156)
**Requirement:** "Implicit Rosenbrock method for 12-species nuclear reaction network"

**Feasibility Concern:**
- Stiff ODE solvers are computationally expensive and complex to implement
- Real-time requirement (≥60 FPS) may conflict with accurate stiff solver integration
- Validation against PArthENoPE/PRIMAT adds complexity

**Question:**
- Can we use a simplified solver for real-time, with offline validation?
- Is real-time nucleosynthesis visualization required, or can it be precomputed?
- What's the acceptable tradeoff between accuracy and performance?

#### D. SPH Implementation (Phase 6, Line 222)
**Requirement:** "Smoothed Particle Hydrodynamics with Wendland C4 kernel for baryonic gas dynamics"

**Feasibility Concerns:**
- SPH is computationally expensive (neighbor search, kernel evaluation)
- At 1M+ particles, SPH may struggle to hit 60 FPS
- Wendland C4 kernel requires more neighbors than simpler kernels

**Question:**
- Can we use a simplified SPH approximation for real-time visualization?
- Should SPH be limited to regions of interest (near halos)?
- What's the acceptable error tolerance for SPH calculations?

### 3. Contradictions with Non-Goals

#### A. Non-Goal vs. Technical Reality
**Non-Goal (Line 59):** "Competing with research-grade codes (Gadget-4, AREPO) for publication-quality results"

**Contradiction:**
- Success metrics require "within 5% of observed helium abundance"
- Success metrics require CMB power spectrum to "qualitatively match Planck data"
- These sound like research-grade validation criteria

**Question:** What is the actual accuracy target?
- Are we building an educational/visualization tool (as Non-Goals suggest)?
- Or a scientifically accurate simulation (as success metrics suggest)?
- How should we prioritize visual fidelity vs. numerical precision?

## Questions for Clarification

### Validation Criteria
1. **For CMB power spectrum:** What is the acceptable deviation from Planck data?
2. **For helium abundance:** Is 5% tolerance achievable with our approximations?
3. **For general validation:** Which metrics should be automated tests vs. manual visual checks?

### Technical Feasibility
4. **Memory budget:** Should we relax the 4 GB VRAM limit or lower particle counts?
5. **GPU portability:** What fallback strategies are acceptable for unsupported hardware?
6. **Stiff ODE solvers:** Can we use simplified/real-time approximations with offline validation?
7. **SPH performance:** Can we use approximations or limit SPH to specific regions?

### Accuracy vs. Fidelity Priority
8. **Core question:** Is this primarily an educational visualization tool or a research-grade simulation?
   - This determines the acceptable tradeoffs for all technical decisions

## Impact

These ambiguities affect:
- Phase 3 (nucleosynthesis ODE solver complexity)
- Phase 5 (N-body + SPH integration, memory budgeting)
- Phase 6 (SPH implementation detail, accuracy targets)
- Phase 7 (validation testing, performance optimization targets)
- Overall architecture decisions (accuracy vs. performance tradeoffs)

## Requested Response

Please clarify:
- The intended use case (educational vs. research-grade accuracy)
- Specific tolerance values for validation metrics
- Acceptable performance/accuracy tradeoffs for each phase
- Whether memory budget targets can be adjusted
- Fallback strategies for unsupported hardware
