# Contributing to Space Trader Rust Port

Thank you for your interest in contributing! This document provides guidelines and information for contributors.

## How to Contribute

### Reporting Bugs

Before creating a bug report, please check existing issues to avoid duplicates. When creating a bug report, include:

- **Description**: Clear description of the bug
- **Steps to reproduce**: Numbered steps to trigger the bug
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **System info**: OS, Rust version, macroquad version
- **Screenshots**: If applicable

### Suggesting Features

Feature suggestions are welcome! Please include:

- **Use case**: Why this feature would be useful
- **Description**: Detailed explanation of the feature
- **Alternatives**: Other solutions you've considered
- **Original game**: Does this exist in the Palm OS version?

### Code Contributions

1. **Fork** the repository
2. **Create a branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes**: Follow code style guidelines
4. **Test**: Ensure the game runs and your changes work
5. **Commit**: Write clear, descriptive commit messages
6. **Push**: Push to your fork
7. **Pull Request**: Create a PR with description of changes

## Development Setup

### Prerequisites

- Rust 1.70+ (`rustup.rs`)
- Python 3 (for asset tools)
- Git

### Building from Source

```bash
# Clone your fork
git clone https://github.com/yourusername/rusttrader.git
cd rusttrader

# Generate placeholder assets
cd tools
python3 generate_placeholder_assets.py
cd ..

# Build and run
cargo run
```

### Running Tests

```bash
cargo test
```

### Code Style

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

## Code Style Guidelines

### Rust Style

- Follow standard Rust formatting (`cargo fmt`)
- Use descriptive variable names
- Add doc comments for public APIs
- Keep functions focused and small
- Avoid unwrap() where practical errors can occur

### Naming Conventions

- **Types**: `PascalCase` (e.g., `SolarSystem`, `TradeGood`)
- **Functions**: `snake_case` (e.g., `buy_cargo`, `warp_to_system`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_FUEL`, `STARTING_CREDITS`)
- **Modules**: `snake_case` (e.g., `trading`, `solar_system`)

### Module Organization

```
src/
├── main.rs           # Entry point and game loop
├── assets.rs         # Asset loading
├── types/            # Data structures
├── game/             # Game logic
├── ui/               # UI screens
└── save/             # Persistence
```

### Comments

- Add doc comments (`///`) for public functions
- Explain "why" not "what" in implementation comments
- Reference original C code when porting features

Example:
```rust
/// Calculates the maximum quantity of a trade good the player can buy
/// based on available credits and cargo space.
///
/// # Arguments
/// * `price` - Current price per unit
/// * `credits` - Player's available credits
/// * `cargo_free` - Available cargo bay space
pub fn max_buyable(price: i32, credits: i32, cargo_free: i32) -> i32 {
    let can_afford = credits / price;
    can_afford.min(cargo_free)
}
```

## Areas Needing Help

### High Priority

1. **Combat System** - Port the original combat mechanics
2. **Encounter System** - Implement random space encounters
3. **UI Improvements** - Better visual feedback and information display
4. **Testing** - Unit tests for game logic

### Medium Priority

1. **Quest System** - Port special events from original game
2. **Equipment System** - Implement ship upgrades
3. **Crew System** - Hire and manage crew members
4. **Audio** - Add sound effects and music

### Low Priority

1. **Visual Polish** - Animations, particles, effects
2. **Settings Screen** - Game configuration options
3. **Achievements** - Track player milestones
4. **Documentation** - Guides, tutorials, API docs

## Porting from Original

When porting features from the original C code:

1. **Find the original**: Check the [original repo](https://github.com/videogamepreservation/spacetrader)
2. **Understand the logic**: Read the C implementation thoroughly
3. **Adapt to Rust**: Use Rust idioms (Result, Option, iterators)
4. **Test**: Verify behavior matches original
5. **Document**: Note which original file/function you ported

### Original Code Structure

The original Space Trader C code is organized as:
- `Src/Main.c` - Main game loop
- `Src/Traveler.c` - Travel and encounters
- `Src/Cargo.c` - Trading logic
- `Src/Equipment.c` - Ship upgrades
- `Src/Encounter.c` - Combat system
- `Src/Ship.c` - Ship management
- `Src/WarpForm.c` - Warp screen
- `Src/Special.c` - Special events

## Testing

### Manual Testing Checklist

Before submitting a PR:

- [ ] Game starts without errors
- [ ] Assets load correctly
- [ ] Trading: Can buy and sell goods
- [ ] Warp: Can travel between systems
- [ ] Fuel: Consumption works correctly
- [ ] Save/Load: Game state persists
- [ ] UI: All screens accessible and functional
- [ ] No compiler warnings from your changes
- [ ] Code formatted with `cargo fmt`
- [ ] Passes `cargo clippy`

### Adding Tests

When adding new features, include unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_buyable() {
        assert_eq!(max_buyable(10, 100, 20), 10);  // Limited by credits
        assert_eq!(max_buyable(10, 300, 20), 20);  // Limited by space
    }
}
```

## Commit Messages

Write clear, descriptive commit messages:

```
Add fuel purchase mechanics

- Add buy_fuel() function in trading module
- Update main screen to show fuel price
- Add 'F' key to purchase fuel at current system
- Costs 1 credit per fuel unit

Relates to Phase 1 roadmap item.
```

Format:
- **First line**: Brief summary (50 chars max)
- **Body**: Detailed explanation of changes
- **Footer**: Issue/PR references if applicable

## License

By contributing, you agree that your contributions will be licensed under the GNU GPL v2.0 or later, matching the original Space Trader game.

## Questions?

- Check the [README](README.md) for project overview
- Review [docs/ROADMAP.md](docs/ROADMAP.md) for planned features
- Check [docs/STATUS.md](docs/STATUS.md) for current active work
- Read [PORTING_NOTES.md](PORTING_NOTES.md) for porting details
- Open an issue for questions or discussions

## Recognition

Contributors will be recognized in:
- README.md credits section
- CHANGELOG.md for their contributions
- Commit history

Thank you for contributing to Space Trader! 🚀
