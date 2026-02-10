# Task #1: Per-Instance Particle Attributes - Decomposition

## Context

This task implements proper synchronization of Particle component data with GPU instance attributes to enable per-instance color and size variation for 10K-50K particle rendering.

### Current Problem

The existing `sync_particle_instance_attributes()` function in [`genesis-render/src/particle/mod.rs`](genesis-render/src/particle/mod.rs:390-429) stores per-instance data in **shared mesh attributes**:

```rust
// Lines 423-428 in sync_particle_instance_attributes()
let mesh_handle = first_particle.1;
if let Some(mesh) = meshes.get_mut(mesh_handle) {
    mesh.insert_attribute(ATTRIBUTE_INSTANCE_SIZE, sizes);
    mesh.insert_attribute(ATTRIBUTE_INSTANCE_COLOR, colors);
}
```

**Why This Doesn't Work:**

1. All particle entities share the same mesh handle via `PointMesh` resource (line 111)
2. In Bevy 0.15's automatic GPU instancing, mesh attributes are **shared across all instances**
3. When multiple entities use the same mesh with `Mesh3d(point_mesh.0.clone())` (line 308), they cannot have different per-instance attribute values
4. The shader's `instance_size` and `instance_color` attributes (locations 1 and 2) would receive the same values for every particle

### Required Solution

Implement a proper per-instance data transfer system where:
1. Each particle entity has its own instance data buffer
2. The sync system populates these buffers from Particle component data
3. The shader reads per-instance data correctly using Bevy 0.15's instancing API
4. Size attenuation uses the `instance_size` attribute for distance-based scaling

### Key Files

- **Primary Implementation**: [`genesis-render/src/particle/mod.rs`](genesis-render/src/particle/mod.rs)
- **Shader**: [`genesis-render/src/particle/point_sprite.wgsl`](genesis-render/src/particle/point_sprite.wgsl)
- **Config**: [`genesis-core/src/config.rs`](genesis-core/src/config.rs) (for ParticleConfig)

### Dependencies

- Bevy 0.15 with automatic GPU instancing
- Existing [`Particle`](genesis-render/src/particle/mod.rs:119-126) component with `position`, `color`, `size` fields
- Existing [`PointSpriteMaterial`](genesis-render/src/particle/mod.rs:76-88) with uniforms
- Existing [`update_particle_energy_colors()`](genesis-render/src/particle/mod.rs:366-380) system that updates Particle.color

---

## Subtasks

### Subtask 1 of 4: Research Bevy 0.15 instancing API and design per-instance buffer infrastructure

**Context:**
- Current implementation incorrectly uses shared mesh attributes for per-instance data
- Need to understand Bevy 0.15's correct API for per-instance attributes with automatic instancing
- Must identify the proper data structures and systems to manage per-instance buffers
- This subtask is pure research and documentation - no code changes yet

**Instructions:**

1. **Research Bevy 0.15 Instancing Documentation**
   - Search Bevy 0.15 documentation for "instancing", "per-instance attributes", "instance buffer"
   - Look for examples of dynamic per-instance data with automatic instancing
   - Check if Bevy 0.15 provides `InstanceBuffer`, `InstanceMeshData`, or similar types
   - Research how `Mesh3d` + `MeshMaterial3d` with automatic batching handles per-instance attributes

2. **Identify Implementation Options**
   Document at least 2 viable approaches:
   - **Option A**: Custom instance buffer component added to each particle entity
   - **Option B**: Bevy's built-in instancing API (if available in 0.15)
   - **Option C**: Custom render node with per-instance attribute binding

3. **Create Design Document**
   Create a new file [`genesis-render/src/particle/DESIGN.md`](genesis-render/src/particle/DESIGN.md) containing:
   - **Selected Approach**: Which implementation option to use (with justification)
   - **Data Structures**: New types/components needed (e.g., `ParticleInstanceData`)
   - **Sync System Design**: How sync_particle_instance_attributes will be restructured
   - **Shader Integration**: How the shader will bind and read per-instance data
   - **Performance Considerations**: Buffer updates, memory layout, GPU transfer

4. **Update mod.rs Documentation**
   Update the documentation comment block starting at line 9 in [`genesis-render/src/particle/mod.rs`](genesis-render/src/particle/mod.rs:9-29) to reflect the new architecture:
   - Replace the "TODO" section (lines 25-29) with a clear description of the chosen approach
   - Update the "Current Behavior" section to reflect the problem and solution

**Acceptance Criteria:**
- [ ] `genesis-render/src/particle/DESIGN.md` exists with complete design specification
- [ ] Design document includes: selected approach, data structures, sync system design, shader integration, performance considerations
- [ ] Documentation in `mod.rs` (lines 9-29) updated to reference the new design
- [ ] Design document is verified by testing: `cargo doc --open --no-deps` and inspecting the design doc
- [ ] No code changes made to existing functions (documentation updates only)

**Do NOT:**
- Do NOT modify any existing function implementations
- Do NOT create any new Rust source files (only the DESIGN.md documentation file)
- Do NOT write any implementation code
- Do NOT modify the shader file
- Do NOT run any tests or build commands

---

### Subtask 2 of 4: Implement per-instance data structures and sync system

**Context:**
- Design from Subtask 1 is available in [`genesis-render/src/particle/DESIGN.md`](genesis-render/src/particle/DESIGN.md)
- Need to replace the broken `sync_particle_instance_attributes()` implementation
- Must create proper per-instance data structures
- Should follow the approach defined in the design document

**Instructions:**

1. **Create Per-Instance Data Component**
   Add a new component to [`genesis-render/src/particle/mod.rs`](genesis-render/src/particle/mod.rs) after the `Particle` struct (around line 127):
   ```rust
   #[derive(Component, Clone)]
   pub struct ParticleInstanceData {
       /// Per-instance size for GPU rendering
       pub instance_size: f32,
       /// Per-instance color for GPU rendering (RGBA in Linear space)
       pub instance_color: [f32; 4],
   }
   ```

2. **Update spawn_particles() to Initialize Instance Data**
   Modify [`spawn_particles()`](genesis-render/src/particle/mod.rs:252-321) to add `ParticleInstanceData` to each spawned entity:
   - Around line 315, add the new component after `Particle`:
   ```rust
   ParticleInstanceData {
       instance_size: size,
       instance_color: [linear.red, linear.green, linear.blue, linear.alpha],
   },
   ```

3. **Rewrite sync_particle_instance_attributes()**
   Completely replace the function at lines 390-429 with proper implementation:
   - Remove the code that modifies shared mesh attributes (lines 407-428)
   - New implementation should update `ParticleInstanceData` components
   - Sync `Particle.size` → `ParticleInstanceData.instance_size`
   - Sync `Particle.color` (converted to Linear RGBA) → `ParticleInstanceData.instance_color`
   - Keep the early return logic for empty particle count

4. **Register Query in Plugin**
   Ensure [`ParticlePlugin`](genesis-render/src/particle/mod.rs:451-466) queries the correct components in the sync system:
   - The query should access `(&Particle, &mut ParticleInstanceData)`
   - Update the system registration if needed (lines 459-464)

5. **Remove Incorrect Mesh Attribute Code from init_point_mesh()**
   In [`init_point_mesh()`](genesis-render/src/particle/mod.rs:137-163), remove lines 146-158 that add empty `ATTRIBUTE_INSTANCE_SIZE` and `ATTRIBUTE_INSTANCE_COLOR` buffers to the shared mesh. The shader will receive per-instance data through the new component instead.

**Acceptance Criteria:**
- [ ] `ParticleInstanceData` component exists with `instance_size` and `instance_color` fields
- [ ] `spawn_particles()` adds `ParticleInstanceData` to each particle entity with correct initial values
- [ ] `sync_particle_instance_attributes()` is rewritten to update `ParticleInstanceData` components from `Particle` data
- [ ] `init_point_mesh()` no longer adds `ATTRIBUTE_INSTANCE_SIZE` and `ATTRIBUTE_INSTANCE_COLOR` to the shared mesh
- [ ] Code compiles without errors: `cargo build --package genesis-render`
- [ ] Code compiles without errors: `cargo build`
- [ ] Clippy passes: `cargo clippy --all-targets --all-features`

**Do NOT:**
- Do NOT modify the shader file (`point_sprite.wgsl`)
- Do NOT change the `Particle` component structure
- Do NOT modify `update_particle_energy_colors()` system
- Do NOT add new dependencies to `Cargo.toml`
- Do NOT change the `PointSpriteMaterial` structure

---

### Subtask 3 of 4: Update shader to use per-instance data with size attenuation

**Context:**
- `ParticleInstanceData` component is now available and populated by sync system
- Shader currently reads `instance_size` and `instance_color` from mesh attributes (locations 1 and 2)
- Need to update shader to read from the proper per-instance data binding
- Must implement size attenuation based on distance from camera

**Instructions:**

1. **Update Shader Vertex Input**
   Modify [`genesis-render/src/particle/point_sprite.wgsl`](genesis-render/src/particle/point_sprite.wgsl):
   - In `VertexInput` struct (lines 5-9), ensure `instance_size` at `@location(1)` and `instance_color` at `@location(2)` are preserved
   - If the design from Subtask 1 requires a different binding scheme (e.g., storage buffer), update the struct accordingly

2. **Implement Size Attenuation in Vertex Shader**
   Update the `vertex()` function (lines 42-58):
   - Calculate distance from camera to particle using `view.world_position` and `world_pos.xyz`
   - Apply size attenuation formula: `attenuated_size = instance_size / (1.0 + distance * material.attenuation_factor)`
   - Use the attenuated size for point sprite rendering (Bevy's `@builtin(point_size)` or manual calculation if needed)

3. **Ensure Color Pass-Through**
   Verify that `output.color` in the vertex shader correctly passes `input.instance_color` to the fragment shader (line 55)

4. **Test Shader Compilation**
   Compile the shader by running the application:
   ```bash
   cargo run
   ```
   - The shader will be compiled at startup by Bevy's asset system
   - Check for WGSL compilation errors in the console

**Acceptance Criteria:**
- [ ] Shader's `VertexInput` struct has `instance_size` and `instance_color` at locations 1 and 2
- [ ] Vertex shader calculates distance from camera and applies size attenuation
- [ ] Size attenuation formula uses `material.attenuation_factor` uniform
- [ ] Shader compiles without errors when running `cargo run`
- [ ] No WGSL compilation errors in console output
- [ ] Clippy passes: `cargo clippy --all-targets --all-features`

**Do NOT:**
- Do NOT modify the fragment shader's color output logic
- Do NOT change the material uniform bindings (`PointSpriteMaterial`)
- Do NOT remove or rename the `view` or `model` uniform bindings
- Do NOT change the `@builtin(position)` clip position calculation

---

### Subtask 4 of 4: Test per-instance rendering with varied colors and sizes

**Context:**
- All implementation is complete
- Need to verify that per-instance attributes work correctly
- Must ensure color and size variations are visible in rendering
- Should confirm that `update_particle_energy_colors()` changes affect rendering

**Instructions:**

1. **Verify Particle Spawning with Varied Attributes**
   Run the application and verify:
   ```bash
   cargo run
   ```
   - Particles spawn successfully with initial varied sizes (0.5 to 2.0 range from line 303)
   - All particles start with white-hot color (energy = 1.0)

2. **Test Color Updates from update_particle_energy_colors()**
   - Allow the simulation to run for several seconds (particles move outward)
   - Observe color changes from white-hot → yellow → orange → red → dark-red based on distance from origin
   - Verify that particles closer to origin appear brighter/whiter
   - Verify that particles further away appear darker/redder
   - This confirms that `update_particle_energy_colors()` → `sync_particle_instance_attributes()` → GPU rendering pipeline is working

3. **Test Size Attenuation**
   - Use camera controls (FreeFlight or Orbit mode) to move closer and farther from the particle cloud
   - Observe that particles appear larger when camera is close and smaller when camera is far
   - Verify that the attenuation is smooth and proportional to distance
   - Test with different `attenuation_factor` values in [`PointSpriteMaterial`](genesis-render/src/particle/mod.rs:76-88) (try 0.005, 0.01, 0.02)

4. **Performance Verification**
   Run with varying particle counts to ensure performance is acceptable:
   ```bash
   # Test with default configuration (check genesis.toml for initial_count)
   cargo run

   # If needed, test with higher particle counts by modifying genesis.toml
   ```
   - Verify that particle count from `ParticleCount` resource matches spawned entities
   - Check that sync system updates all particles each frame (no particles skipped)
   - Ensure frame rate remains ≥60 FPS for 10K particles (adjust initial_count as needed)

5. **Manual Test Code (Optional)**
   If visual verification is insufficient, add temporary test code:
   - Add `println!()` statements in `sync_particle_instance_attributes()` to log size/color values for a few particles
   - Verify that values change as expected when particles move
   - Remove temporary debug code after verification

**Acceptance Criteria:**
- [ ] Application runs without crashes: `cargo run`
- [ ] Particles display varied colors (white → yellow → orange → red → dark-red) based on distance from origin
- [ ] Particles display varied sizes (size attenuation works with camera distance)
- [ ] Color changes from `update_particle_energy_colors()` are visible in rendering
- [ ] All spawned particles are rendered (no missing particles)
- [ ] Frame rate is ≥60 FPS with 10K particles (verify with `cargo run` and visual inspection)
- [ ] No errors or warnings in console output

**Do NOT:**
- Do NOT modify any implementation code for this subtask
- Do NOT change the particle count in genesis.toml unless explicitly testing performance
- Do NOT commit temporary debug code
- Do NOT proceed to other tasks until this task is complete and verified

---

## Overall Task Acceptance Criteria

- [ ] All 4 subtasks completed in order
- [ ] Each subtask has its acceptance criteria met
- [ ] Per-instance particle rendering works with varied colors and sizes
- [ ] Size attenuation is applied correctly based on camera distance
- [ ] `update_particle_energy_colors()` changes are visible in GPU rendering
- [ ] Application compiles without errors: `cargo build`
- [ ] Application runs without crashes: `cargo run`
- [ ] Performance is ≥60 FPS with 10K particles
- [ ] Clippy passes: `cargo clippy --all-targets --all-features`
- [ ] TODO.md item "Synchronize Particle component data with GPU instance attributes" can be marked complete

## Notes for Orchestrator

1. **Subtask Dependency Order**: Subtasks must be completed in order (1 → 2 → 3 → 4)
2. **Verification Required**: Each subtask must be verified before proceeding to the next
3. **No Code Changes in Subtask 1**: Subtask 1 is research-only; no implementation code should be written
4. **Critical Path**: This task is on the critical path for 10K-50K particle scaling (enables per-instance variation without individual draw calls)
5. **Risk Assessment**: If Bevy 0.15's instancing API doesn't support the desired approach, Subtask 1 should document alternative implementations and may require iteration

## Related TODO.md Items

- Line 14: "feature: Synchronize Particle component data with GPU instance attributes (Sprint 1)"
- Line 50: "feature: Scale particle system to 10K-50K particles (configurable) (Sprint 1)" - depends on this task
