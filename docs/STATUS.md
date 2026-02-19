# Status

Canonical live status for active implementation.

## Now

- **Focus:** UI architecture cleanup (extracting warp/chart rendering from `src/main.rs` into `src/ui/*`).
- **Constraint:** maintain behavior parity during extraction.

## Next

1. Extract warp/chart rendering helpers.
2. Extract encounter/system presentation renderers.
3. Document resulting module boundaries.

## Blocked

- No active blockers recorded.

## Recently Done

- Added canonical planning/coordination docs under `docs/`.
- Extracted trading/shipyard/repair render functions into `src/ui/game_screen.rs`.

## Drift Notes

Legacy docs may contain historical or stale progress statements. For active work tracking, use this file and `docs/ROADMAP.md`.

## Update Rule

When starting or finishing roadmap work, update this file in the same change.
