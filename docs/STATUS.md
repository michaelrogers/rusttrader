# Status

Canonical live status for active implementation.

## Now

- **Focus:** Bug fixes and rendering cleanup
- **Current:** Finalized warp screen - explicit bounds checking for geometry, text at full resolution

## Next

1. Test warp screen with various resolutions to confirm no regressions  
2. Evaluate P1 (Save/Load Path Consistency) or P2 (Asset Naming + Doc Consistency)
3. Optional: Extract remaining main-screen/ship-shop rendering

## Blocked

- No active blockers recorded.

## Recently Done

- Added canonical planning/coordination docs under `docs/`.
- Extracted trading/shipyard/repair render functions into `src/ui/game_screen.rs`.
- Extracted warp/chart rendering helpers and hit-tests into `src/ui/game_screen.rs`.
- Extracted system-info and encounter presentation renderers into `src/ui/game_screen.rs`.
- Reduced draw-stage branching in `src/main.rs` using `match` and restored explicit `SystemInfo` screen rendering.
- Documented current UI module ownership boundaries in `docs/DECISIONS.md`.
- Consolidated shared drawing utilities: `draw_text_with_limits` and `draw_panel` now public in ui module.
- Removed duplicate code and reduced UI formatting duplication across modules.
- **Fixed warp screen rendering:** 
  - Full-screen camera for consistent coordinates and text legibility
  - Explicit bounds checking for circles, lines, and crosshair to prevent overflow
  - Text labels render in screen space at full resolution

## Drift Notes

Legacy docs may contain historical or stale progress statements. For active work tracking, use this file and `docs/ROADMAP.md`.

## Update Rule

When starting or finishing roadmap work, update this file in the same change.
