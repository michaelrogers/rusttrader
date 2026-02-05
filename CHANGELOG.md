# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Core game loop with three screens (Main, Trading, Warp)
- Complete trading system with buy/sell mechanics
- Warp/travel system with fuel consumption
- 120 procedurally generated solar systems
- Dynamic pricing based on tech level
- Ship type system (15 ship types from original)
- Equipment and crew data structures
- Asset loading system with texture caching
- Placeholder asset generation (65 sprites)
- Palm OS asset extraction tools
- Save/load system with JSON
- Main menu (New/Load/Quit)
- Game state persistence

### Technical
- Rust 2021 with macroquad 0.4 game engine
- Modular architecture (types/, game/, ui/, save/)
- Asset fallback rendering system
- Distance-based warp range calculation
- Visited system tracking
- Days counter for progression

## [0.1.0] - 2024-XX-XX

### Initial Release
- First playable version with core trading loop
- Basic gameplay: Trade → Warp → Trade
- Placeholder graphics for development
