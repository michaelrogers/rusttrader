# Status

Canonical live status for active implementation.

## Now

- **Focus:** Bug fixes and UI polish
- **Current:** Fixed warp screen scaling regression - camera now properly scaled to panel bounds

## Next

1. Evaluate P1 (Save/Load Path Consistency) or P2 (Asset Naming + Doc Consistency).
2. Optional: Extract remaining main-screen/ship-shop rendering.
3. Keep `docs/DECISIONS.md` updated for any boundary changes.

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
- **Fixed warp screen scaling regression:** Changed camera rect from full screen to panel bounds, ensuring proper viewport scaling.

## Drift Notes

Legacy docs may contain historical or stale progress statements. For active work tracking, use this file and `docs/ROADMAP.md`.

## Update Rule

When starting or finishing roadmap work, update this file in the same change.
