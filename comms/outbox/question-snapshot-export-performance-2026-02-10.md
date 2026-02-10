# Question: Snapshot Export Performance Targets

**Date:** 2026-02-10
**Source:** PRD Review

## Ambiguity Identified

The PRD specifies a Snapshot Export performance target of "<2s for 10M particles" for Real-Time Mode. This target may be technically infeasible depending on:

1. **Storage media type** (SSD vs NVMe vs HDD) - I/O bandwidth varies dramatically
2. **Sync vs async export** - Synchronous export blocks the UI; async doesn't
3. **What attributes are included** - Position only vs full state (position, velocity, mass, composition, etc.)
4. **Export target type** - Binary format vs text/JSON vs custom format
5. **Different operating modes** - Real-Time vs High-Fidelity mode requirements

The PRD also specifies "<30s for 100M particles" for High-Fidelity Mode, but lacks clarification on whether these are the same export mechanism with different data volumes, or fundamentally different approaches.

## PRD References

### Performance Targets (Section 8)

| Metric | Real-Time Mode | High-Fidelity Mode |
|--------|---------------|-------------------|
| Particle Count | 1M – 10M | 50M – 100M |
| Snapshot Export | <2s for 10M particles | <30s for 100M particles |

### Technical Feasibility Analysis

**For 10M particles:**
- Position data (3 × float32) = 120 MB
- Full state (pos + vel + mass + composition + flags) ≈ 500 MB
- **<2s target requires 60-250 MB/s sustained write speed**
- HDD: ~100 MB/s (sequential), SSD: ~500 MB/s, NVMe: ~2,000-5,000 MB/s
- Real-time serialization overhead also impacts timing

**For 100M particles:**
- Position data = 1.2 GB
- Full state ≈ 5 GB
- **<30s target requires 40-170 MB/s sustained write speed**
- More achievable but still HDD-constrained for full state

## Questions for Product Owner

1. **What storage medium should the performance target assume?**
   - SSD ( SATA, ~500 MB/s)?
   - NVMe (PCIe 3.0/4.0, ~2,000-5,000 MB/s)?
   - HDD (SATA, ~100 MB/s)?
   - Or should targets vary by medium?

2. **Should snapshot export be synchronous or asynchronous?**
   - Synchronous: Blocks UI until export completes (user waits)
   - Asynchronous: UI remains responsive, export runs in background
   - If async, what user feedback is expected (progress bar, notification)?

3. **What particle attributes are included in the snapshot?**
   - Position only (3 floats)?
   - Position + velocity (6 floats)?
   - Full physics state (position, velocity, mass, composition, flags)?
   - User-configurable subset?

4. **What export file format is expected?**
   - Binary custom format (fastest)?
   - Standard format (HDF5, Parquet)?
   - Human-readable (JSON, CSV - slower)?
   - Compressed (adds CPU overhead)?

5. **Are the Real-Time and High-Fidelity export mechanisms the same?**
   - Same format with different data volumes?
   - Different formats (e.g., Real-Time uses cached buffer, High-Fidelity writes full state)?
   - Or different approaches entirely (e.g., Real-Time exports current frame, High-Fidelity exports simulation state)?

6. **Should there be different export modes?**
   - Quick export (position only, minimal metadata)?
   - Full export (all attributes)?
   - Analysis export (additional derived quantities)?
   - User-selectable in UI?

7. **What is the acceptable fallback if <2s cannot be achieved?**
   - Increase particle count threshold (e.g., <2s for 5M particles)?
   - Increase time budget (e.g., <5s for 10M particles)?
   - Warn user if target hardware will be slow?
   - Provide quality/speed tradeoff options?

8. **How should snapshot export interact with the simulation?**
   - Pause simulation during export?
   - Continue simulation (export is a snapshot in time)?
   - Lock particle buffers to ensure consistency?
   - Allow concurrent simulation updates?

## Impact

This decision affects:
1. File I/O subsystem architecture
2. Serialization/deserialization code
3. UI responsiveness during export operations
4. User documentation and expectations
5. Validation test cases
6. Performance optimization priorities
7. Storage requirements for user machines
