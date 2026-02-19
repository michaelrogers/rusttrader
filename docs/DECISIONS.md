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
