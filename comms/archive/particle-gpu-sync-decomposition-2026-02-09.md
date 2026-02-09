## Decomposition: Synchronize Particle component data with GPU instance attributes

### Context
**Project:** GENESIS - Real-time Big Bang and Cosmological Evolution Simulator (Rust + Bevy 0.15+)

**Target TODO Item:** feature: Synchronize Particle component data with GPU instance attributes

**Problem Statement:** The current particle rendering system has a mismatch between CPU-side Particle component data and GPU-side instance attributes. The shader expects per-instance attributes at locations 1 (size) and 2 (color), and the Particle component contains individual color and size data per entity. However, the PointMesh is initialized with single default values shared by all instances, and no mechanism exists to sync Particle component data to GPU instance attributes. Consequently, when update_particle_energy_colors() modifies Particle.color, these changes never reach the GPU.

**Relevant Files:**
- /workspace/genesis-render/src/particle/mod.rs (533 lines) - Main particle module with Particle component, PointMesh, init_point_mesh, spawn_particles, update_particles, update_particle_energy_colors
- /workspace/assets/point_sprite.wgsl (66 lines) - WGSL shader expecting instance_size @location(1) and instance_color @location(2)
- /workspace/genesis-core/src/physics/mod.rs (30 lines) - Physics Particle data structure (different type from render Particle)

**Technical Details:**
- Particle component: `pub struct Particle { pub position: Vec3, pub color: Color, pub size: f32 }`
- Custom vertex attributes: ATTRIBUTE_INSTANCE_SIZE (Float32, location 1), ATTRIBUTE_INSTANCE_COLOR (Float32x4, location 2)
- Current init_point_mesh() initializes instance attributes with single-element vectors: vec![1.0f32] and vec![white_as_rgba()]
- Particles spawned as separate entities sharing same Mesh3d and MeshMaterial3d handles
- update_particle_energy_colors() modifies Particle.color based on distance thermal gradient

### Subtask Decomposition

Based on analysis, this feature requires 4 atomic subtasks:

#### Subtask 1 of 4: Add ParticleCount resource
**Objective:** Track the total number of particle entities for buffer sizing.

**Instructions:**
1. In /workspace/genesis-render/src/particle/mod.rs, create a new resource type:
   ```rust
   #[derive(Resource, Default)]
   pub struct ParticleCount(pub usize);
   ```
2. Initialize ParticleCount(0) in the ParticlePlugin::build() method using app.init_resource::<ParticleCount>()
3. In spawn_particles(), update the resource: commands.insert_resource(ParticleCount(count)) after spawning particles

**Acceptance Criteria:**
- [ ] ParticleCount resource is defined and exported from genesis-render::particle module
- [ ] app.init_resource::<ParticleCount>() is called in ParticlePlugin::build()
- [ ] spawn_particles() updates ParticleCount with the correct count
- [ ] Code compiles: `cargo check --package genesis-render`

**Do NOT:**
- Modify the WGSL shader
- Create the sync system (that's subtask 2)
- Change the PointMesh initialization yet (that's subtask 3)

---

#### Subtask 2 of 4: Create sync_particle_instance_attributes system
**Objective:** Implement a system that synchronizes Particle component data to GPU instance attributes.

**Instructions:**
1. In /workspace/genesis-render/src/particle/mod.rs, create a new system function:
   ```rust
   fn sync_particle_instance_attributes(
       particles: Query<(&Particle, &Mesh3d)>,
       mut meshes: ResMut<Assets<Mesh>>,
       particle_count: Res<ParticleCount>,
   )
   ```
2. Query all entities with both Particle and Mesh3d components
3. Collect size and color data from each particle into separate vectors:
   - sizes: Vec<f32> containing each particle's size
   - colors: Vec<[f32; 4]> containing each particle's color (use Color.as_rgba_f32() to get [r,g,b,a])
4. Get the shared mesh using Mesh3d handle and mutate its instance attributes:
   - Set attribute ATTRIBUTE_INSTANCE_SIZE to the collected sizes vector
   - Set attribute ATTRIBUTE_INSTANCE_COLOR to the collected colors vector
5. Handle edge cases: if particle_count is 0 or no particles found, early return

**Acceptance Criteria:**
- [ ] sync_particle_instance_attributes system is defined and compiles
- [ ] System is added to ParticlePlugin::build() with appropriate run condition
- [ ] System queries Particle and Mesh3d components correctly
- [ ] System mutates Mesh assets to update instance attributes
- [ ] Code compiles: `cargo check --package genesis-render`

**Do NOT:**
- Modify the init_point_mesh() function (that's subtask 3)
- Change the shader or attribute definitions

---

#### Subtask 3 of 4: Update init_point_mesh to initialize with proper buffer capacity
**Objective:** Initialize PointMesh instance attributes with empty buffers sized for the expected particle count.

**Instructions:**
1. Modify /workspace/genesis-render/src/particle/mod.rs init_point_mesh() function
2. Instead of initializing with single-element vectors (vec![1.0f32], vec![white_as_rgba()]), initialize with empty vectors that can grow:
   - For ATTRIBUTE_INSTANCE_SIZE: create an empty Vec<f32> that will be populated by the sync system
   - For ATTRIBUTE_INSTANCE_COLOR: create an empty Vec<[f32; 4]> that will be populated by the sync system
3. The mesh should still define the vertex attribute structure, but initial values should be empty/default

**Acceptance Criteria:**
- [ ] init_point_mesh() initializes instance attributes with empty vectors, not single default values
- [ ] Code compiles: `cargo check --package genesis-render`
- [ ] PointMesh is still created and registered as a resource

**Do NOT:**
- Remove the instance attribute definitions
- Change the shader
- Modify the sync system

---

#### Subtask 4 of 4: Test and verify particle color/size updates affect rendering
**Objective:** Verify that changes to Particle.color and Particle.size are reflected in GPU rendering.

**Instructions:**
1. Build the application: `cargo build --release`
2. Run the application: `cargo run --release`
3. Observe particle colors during the thermal gradient animation (update_particle_energy_colors)
4. Verify that:
   - Particles at the center (distance < 10) appear white-hot (from energy_to_color(1.0))
   - Particles at intermediate distances (10 ≤ distance < 20) appear yellow-orange
   - Particles at outer distances (distance ≥ 20) appear red
   - The color changes are visible in real-time as particles move outward

**Acceptance Criteria:**
- [ ] Application builds successfully
- [ ] Application runs without errors
- [ ] Particle colors change based on distance from center (thermal gradient visible)
- [ ] Color changes are synchronized with GPU rendering (visible in output)

**Do NOT:**
- Modify any source code (this is a verification-only subtask)
- Create test files or automated tests (visual verification is sufficient)

### Execution Order
Subtasks must be executed in order: 1 → 2 → 3 → 4. Each subtask depends on the previous one.

### Verification Protocol
After each subtask completes, verify all acceptance criteria pass before proceeding to the next subtask.

---
