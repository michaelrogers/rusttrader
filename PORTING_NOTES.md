# Space Trader Porting Notes

## Original Source Analysis

The original Space Trader is written in C for Palm OS. Key characteristics:

- **~30 C source files** with clear module separation
- **Event-driven architecture** using Palm OS forms and event handlers
- **Procedural programming** style with global state
- **Manual memory management** with Palm OS memory handles
- **120 solar systems** with procedural generation
- **Complex trading system** with dynamic pricing based on:
  - Tech level
  - Political system
  - Special resources
  - Random events
  - Supply/demand

## Rust Port Strategy

### Completed
- ✅ Basic project structure
- ✅ Core data types (Ship, SolarSystem, Crew, Equipment, Trade)
- ✅ Constants ported from original
- ✅ Galaxy generation stub
- ✅ Basic game state management

### In Progress
- 🚧 Galaxy generation algorithm
- 🚧 Trading system
- 🚧 Pricing algorithm
- 🚧 UI layer

### To Do
- ⬜ Complete pricing algorithm (DeterminePrices, RecalculateBuyPrices)
- ⬜ Encounter system (pirates, police, traders)
- ⬜ Combat system
- ⬜ Quest system (37 special events)
- ⬜ Ship upgrade system
- ⬜ Crew hiring system
- ⬜ Random events
- ⬜ High score system
- ⬜ Complete UI implementation
- ⬜ Sound effects (optional)
- ⬜ Wormhole system
- ⬜ Special systems (Sol, Utopia, etc.)

## Key Differences from Original

### Memory Management
- **Original**: Manual with Palm OS MemHandle
- **Rust**: Automatic with ownership system

### State Management
- **Original**: Global variables
- **Rust**: GameState struct with proper encapsulation

### Error Handling
- **Original**: Error codes and alerts
- **Rust**: Result<T, E> types

### UI
- **Original**: Palm OS forms with event handlers
- **Rust**: macroquad game loop with immediate mode rendering

## Files to Reference from Original

Key original files for porting:

1. `Global.c` - All constants and static data
2. `Traveler.c` - Travel, galaxy generation, core game loop
3. `Merchant.c` - Trading system
4. `Encounter.c` - Combat and encounters
5. `SpecialEvent.c` - Quest system
6. `Shipyard.c` - Ship upgrades
7. `DataTypes.h` - All struct definitions
8. `spacetrader.h` - Constants

## Next Steps

1. **Complete galaxy generation** - Port StartNewGame() from Traveler.c
2. **Implement pricing** - Port DeterminePrices() from Traveler.c
3. **Add basic UI** - Create trading and navigation screens
4. **Test core loop** - Verify buy/sell/warp works
5. **Add encounters** - Port basic pirate/police logic
6. **Implement combat** - Port encounter resolution
7. **Add quests** - Port special event system

## Testing Strategy

1. Generate galaxy - verify 120 systems, proper spacing
2. Test trading - buy/sell at different systems
3. Test travel - fuel consumption, distance calculations
4. Test encounters - random generation, combat resolution
5. Test save/load - game state persistence
6. Playtest - balance and gameplay feel

## Performance Notes

- Original ran on Palm Pilot (~20 MHz, 8 MB RAM)
- Rust version should be instantaneous on modern hardware
- Focus on correctness first, optimize later if needed

## License Compliance

- Original: GPL-2.0-or-later
- This port: Same license
- Must credit Pieter Spronck and original authors
- Must include GPL license text
- Source code must be available
