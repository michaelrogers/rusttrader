# Status

Canonical live status for active implementation.

## Now

- **Focus:** Rendering cleanup and window resize handling
- **Current:** Fixed both chart screens for proper viewport clipping on window resize

## Next

1. Test both charts with various window sizes to confirm clipping works
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
- **Fixed short-range chart rendering:** 
  - Full-screen camera for consistent coordinates and text legibility
  - Explicit bounds checking for circles, lines, and crosshair
  - Text labels render in screen space at full resolution
- **Fixed galactic chart rendering:**
  - Added viewport setup for proper clipping on window resize
  - Range circle and system squares only render if visible
  - Text labels render in screen space to prevent corruption

## Drift Notes

Legacy docs may contain historical or stale progress statements. For active work tracking, use this file and `docs/ROADMAP.md`.

## Update Rule

When starting or finishing roadmap work, update this file in the same change.
