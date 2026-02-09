# Time Acceleration Starting Value Clarification

**Date:** 2026-02-09  
**Related TODO:** fix: Update genesis.toml time.initial_time_acceleration to match PRD Phase 1 starting range

## Context

PRD Phase 1 (line 115) specifies "adjustable acceleration (1x to 10¹²x)" as the configurable range.

Current genesis.toml values:
```toml
[time]
time_acceleration_min = 1.0
time_acceleration_max = 1000000000000.0
initial_time_acceleration = 1.0
```

The min/max values correctly match the PRD range. However, the `initial_time_acceleration = 1.0` starts at the minimum end (1x real-time).

## Question

What should the `initial_time_acceleration` value be for Phase 1?

## Options

1. **Keep current value (1.0)**
   - Pros: Technically compliant with PRD, starts slowest, user can speed up
   - Cons: Demo may appear static at launch

2. **Set to midpoint (1.0×10⁹ = 1000000000.0)**
   - Pros: Faster initial demo experience, shows particle evolution immediately
   - Cons: May be too fast for some users

3. **Set to quarter range (1.0×10¹¹ = 100000000000.0)**
   - Pros: More aggressive demo experience
   - Cons: Very fast, may overwhelm users

4. **Other specified value**: _______________

## Impact

This affects the initial demo experience when the application launches. A higher value provides immediate visual feedback of particle evolution, while a lower value starts slower.

## Recommendation Needed

Please select an option or provide a specific value for `initial_time_acceleration`.

---

# Response

**Date:** 2026-02-09  
**Status:** Response Received

## Selected Option
**Set to midpoint (1.0×10⁹ = 1000000000.0)**
   - Pros: Faster initial demo experience, shows particle evolution immediately
   - Cons: May be too fast for some users
