# Decisions

Track implementation decisions that affect architecture, workflow, or compatibility.

---

## ADR-0001: Canonical planning docs + UI-first extraction

- **Date:** 2026-02-19
- **Status:** accepted

### Context

Project status and roadmap details are spread across multiple root docs with some drift. UI rendering is heavily concentrated in `src/main.rs`, increasing change risk and merge conflicts.

### Decision

- Use `docs/ROADMAP.md` as the canonical implementation roadmap.
- Use `docs/STATUS.md` as the canonical live progress snapshot.
- Prioritize UI extraction from `src/main.rs` into `src/ui/*` before broader feature expansion.
- Use a lightweight handoff process (`docs/AGENT_HANDOFF.md`) rather than strict process gating.

### Consequences

- Reduced ambiguity for multi-agent collaboration.
- Lower coordination overhead than strict workflow templates.
- Near-term effort favors architecture cleanup over new gameplay features.

---

## ADR-0002: UI module ownership boundaries after Sprint C

- **Date:** 2026-02-19
- **Status:** accepted

### Context

`src/main.rs` previously contained most screen rendering and chart utility code, increasing merge conflicts and making incremental UI work harder to isolate.

### Decision

- `src/ui/game_screen.rs` owns screen renderers for trading, repair, shipyard, warp/chart, encounter, and system-info views.
- `src/ui/game_screen.rs` also owns chart hit-test helpers used by input flow (`short_range_chart_hit_test`, `galactic_chart_hit_test`).
- `src/main.rs` remains responsible for game loop orchestration, input/state transitions, and main-screen rendering that has not yet been extracted.

### Consequences

- Screen UI changes can land with lower conflict risk by targeting `src/ui/game_screen.rs`.
- `src/main.rs` is reduced in rendering responsibility but still contains transition logic and remaining UI branches.
- Future UI extraction should keep behavior parity and avoid changing game rules while moving rendering code.
