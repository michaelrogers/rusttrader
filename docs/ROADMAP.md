# Roadmap

Canonical planning document for implementation sequencing.

## Planning Horizons

- **Execution horizon (1-2 sprints):** concrete, mergeable tasks.
- **Direction horizon (quarter):** architecture and stabilization goals.

## Current Priorities (Execution)

### P0: UI Architecture Cleanup

Goal: Reduce UI concentration in `src/main.rs` by extracting rendering + screen-specific logic into `src/ui/*` while preserving behavior.

#### Sprint A (next)

- [x] Extract `draw_trading_screen` into `src/ui/game_screen.rs`
- [x] Extract `draw_shipyard_screen` into `src/ui/game_screen.rs`
- [x] Extract `draw_repair_screen` into `src/ui/game_screen.rs`
- [x] Keep behavior parity (no gameplay changes)
- [ ] Verify with `cargo fmt --check` + `cargo check`

#### Sprint B

- [ ] Extract warp/chart rendering helpers (`draw_short_range_chart`, `draw_galactic_chart`, `draw_warp_screen`)
- [ ] Consolidate shared drawing utilities
- [ ] Reduce duplicate UI formatting logic

#### Sprint C

- [ ] Extract encounter/system presentation (`draw_encounter_screen`, `draw_system_info_screen`)
- [ ] Minimize UI branching in `src/main.rs`
- [ ] Document module boundaries in `docs/DECISIONS.md`

### P1: Save/Load Path Consistency

- [ ] Unify runtime save/load path with `src/save/mod.rs`
- [ ] Remove/replace stub persistence methods in `src/types/mod.rs`
- [ ] Update player-facing docs after implementation is confirmed

### P2: Asset Naming + Doc Consistency

- [ ] Align extraction docs/tool output names with `src/assets.rs` loader expectations
- [ ] Consolidate conflicting status notes into `docs/STATUS.md`

## Quarter Direction

- **UI modularity:** major screen rendering no longer owned by `src/main.rs`.
- **Documentation reliability:** `docs/ROADMAP.md` + `docs/STATUS.md` are the only live planning/status sources.
- **Quality gates:** lightweight but consistent pre-merge checks documented in `docs/TESTING.md`.

## Ownership Model

- Agents should claim one roadmap item at a time in `docs/STATUS.md`.
- Prefer small PR-sized increments with clear handoff notes in `docs/AGENT_HANDOFF.md`.

## Change Control

When priorities change, update this file first, then mirror active work in `docs/STATUS.md`.
