# Review: Ignored Tests in resource_binding_tests.rs
Date: 2026-02-10

## Task
Review 8 ignored tests in genesis-render/tests/resource_binding_tests.rs

## Analysis Summary

### Tests Found (8 total)
- test_materials_initialized_before_rendering (line 402)
- test_system_ordering_point_mesh_before_spawn (line 501)
- test_resources_created_at_startup (line 561)
- test_resources_accessible_during_update (line 616)
- test_pipeline_cache_no_index_out_of_bounds (line 701)
- test_resource_reference_counting (line 1023)
- test_complete_particle_rendering_setup (line 1161)
- test_extract_system_transfers_data (line 1238)

### Recommendations

| Action | Tests | Count |
|--------|-------|-------|
| Refactor | test_materials_initialized_before_rendering, test_system_ordering_point_mesh_before_spawn, test_resources_created_at_startup, test_resources_accessible_during_update, test_resource_reference_counting | 5 |
| Keep Ignored | test_pipeline_cache_no_index_out_of_bounds, test_complete_particle_rendering_setup, test_extract_system_transfers_data | 3 |

### Key Findings
- 62.5% (5/8) of ignored tests can be refactored to remove GPU dependency
- Refactoring involves replacing Assets<PointSpriteMaterial> with direct struct tests, removing spawn_particles dependency, or using custom mock systems
- 3 tests are inherently integration tests requiring GPU resources and should remain ignored
- Blocker documented in BLOCKERS.md remains valid for the 3 integration tests

### Action Items (for future sessions)
- Refactor 5 tests to remove GPU dependency
- Update BLOCKERS.md to note which tests can be refactored
- Consider adding GPU CI for re-enabling integration tests
