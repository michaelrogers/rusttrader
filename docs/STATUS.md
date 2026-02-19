# Status

Canonical live status for active implementation.

## Now

- **Focus:** UI architecture cleanup (consolidating shared drawing utilities).
- **Constraint:** maintain behavior parity during extraction.

## Next

1. Consolidate shared drawing utilities across `src/main.rs` and `src/ui/game_screen.rs`.
2. Evaluate optional extraction of remaining main-screen/ship-shop rendering.
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

## Drift Notes

Legacy docs may contain historical or stale progress statements. For active work tracking, use this file and `docs/ROADMAP.md`.

## Update Rule

When starting or finishing roadmap work, update this file in the same change.
