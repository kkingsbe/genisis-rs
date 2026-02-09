# Question: Camera Interpolation and Epoch Transitions

## Ambiguity Identified
The PRD mentions camera transitions in multiple phases without a unified specification:

- **Phase 1**: "free-flight camera (WASD + mouse) and orbit camera (click-drag) with smooth interpolation"
- **Phase 4**: "Smooth camera transition: as recombination completes, the 'fog lifts' and the camera pulls back to reveal the CMB sphere"
- **Phase 7**: "Cinematic mode: pre-authored camera paths with keyframes and easing curves"

The PRD lacks clarity on:
1. When smooth interpolation is used (user actions vs. automatic transitions)
2. How to switch between camera modes (free-flight, orbit, cinematic)
3. What "smooth interpolation" means for camera transitions
4. Implementation details for keyframe-based cinematic paths

## Why This Is a Problem

1. **Inconsistent Requirements**: Phase 1 expects user-controlled cameras with interpolation (for what?), while Phase 4 expects automatic transitions, and Phase 7 expects keyframe paths.

2. **Mode Confusion**: How does the system know when to use which camera?
   - Free-flight/orbit: User controlled?
   - Automatic transitions: Epoch boundaries?
   - Cinematic: User-triggered or automatic?

3. **Implementation Overlap**: Are these separate systems or one unified camera controller with multiple modes?

4. **User Experience**: How does the user switch between camera modes? Is it manual or automatic?

5. **Timeline Scrubbing**: When scrubbing the timeline, how does the camera behave? Does it stay in current position, move to saved positions, or follow cinematic paths?

## Suggested Approaches

1. **Unified Camera Controller with Three Modes (Recommended)**
   - Implement single camera system with modes: Interactive, Automatic, Cinematic
   - **Interactive mode**: Free-flight (WASD + mouse) and orbit (click-drag), no automatic interpolation
   - **Automatic mode**: Camera follows pre-defined path during epoch transitions (Phase 4 fog lift)
   - **Cinematic mode**: Full keyframe-based playback for Phase 7
   - User can switch between modes via UI hotkeys
   - Pros: Single system, clear separation of concerns, user control, consistent behavior
   - Cons: More complex camera controller, requires mode transition logic

2. **Separate Camera Systems**
   - Free-flight camera: User-controlled, no interpolation
   - Orbit camera: User-controlled, no interpolation
   - Transition camera: Automatic, used at epoch boundaries
   - Cinematic camera: Keyframe paths for Phase 7
   - Each is a separate system active at different times
   - Pros: Each system optimized for its purpose, simpler individual implementations
   - Cons: Code duplication, harder to maintain, inconsistent UX

3. **Camera Tween System + User Control**
   - Implement "smooth interpolation" as tween animations for user-initiated camera moves
   - When user moves camera, it interpolates to new position over time
   - Epoch transitions trigger automatic tweens
   - Cinematic mode uses keyframe sequences of tweens
   - Pros: Consistent "smooth" feel, unified tween system
   - Cons: Tween on every user move may feel sluggish, not intuitive for navigation

4. **Camera Position Timeline**
   - Camera position is a timeline dimension alongside particle simulation
   - Scrubbing timeline changes both particles AND camera position
   - Cinematic mode defines camera positions at each timeline point
   - User can override camera at any point (saved as new keyframe)
   - Pros: Unifies camera with timeline, intuitive for "history" navigation
   - Cons: Complex camera timeline system, massive storage for camera history

5. **No Interpolation for User Control**
   - Free-flight and orbit cameras are instant (no interpolation for direct control)
   - "Smooth interpolation" in Phase 1 refers to mode switching (orbit ↔ free-flight) only
   - Automatic epoch transitions have interpolation
   - Cinematic mode uses keyframes
   - Pros: Responsive user control, simple implementation
   - Cons: May not match "smooth interpolation" expectation in Phase 1

## Additional Questions

- **Mode switching**: How does the user switch camera modes?
  - Keyboard shortcuts (F1: free-flight, F2: orbit, F3: cinematic)?
  - UI panel dropdown?
  - Automatic based on context?

- **Cinematic mode**: Is cinematic mode:
  - User-triggered (press button to watch pre-authored sequence)?
  - Automatic after reaching end of timeline?
  - Both options?

- **Epoch transition timing**: How long should automatic transitions last?
  - Fixed duration (e.g., 3 seconds)?
  - Proportional to epoch length?
  - User-configurable?

- **Timeline scrubbing**: When scrubbing, should camera:
  - Stay at current user position?
  - Move to saved camera positions at each timeline point?
  - Follow cinematic path if in cinematic mode?

## Question for Product Owner
How should camera interpolation work across the different modes?

Should we use a unified camera controller with three modes (Approach 1), separate systems (Approach 2), a tween-based approach (Approach 3), or instant control for navigation with interpolation only for transitions (Approach 5)?

Additionally, for user interaction:
- How should users switch between free-flight, orbit, and cinematic modes?
- Should cinematic mode be user-triggered or automatic?
- How should the camera behave during timeline scrubbing?
