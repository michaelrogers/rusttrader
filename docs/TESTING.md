# Testing

Minimal, consistent validation expectations for contributors and agents.

## Fast Checks (required for code changes)

```bash
cargo fmt --check
cargo check
```

## Recommended Checks

```bash
cargo clippy --all-targets --all-features
cargo test
```

## UI Refactor Manual Checks

When changing UI flow/rendering, verify:

- Main menu appears and accepts input.
- Main game screen renders current system/credits/fuel/day.
- Trading screen selection and actions still work.
- Warp screen + chart navigation still work.
- Repair/shipyard/info/encounter screens still render.

## Documentation Changes

For docs-only changes:

- Validate markdown links resolve.
- Ensure `docs/ROADMAP.md` and `docs/STATUS.md` remain aligned.
