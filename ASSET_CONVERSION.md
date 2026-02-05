# Space Trader Asset Conversion Guide

This document explains the asset conversion process from the original Palm OS Space Trader to the Rust port.

## Overview

The original Space Trader game for Palm OS contains bitmap graphics created by **Alexander Lawrence** (al_virtual@yahoo.com). These graphics are stored in Palm OS resource files (`.rsrc`) in a binary format specific to Palm OS.

## Copyright & Licensing

- **Original Artwork**: © Alexander Lawrence
- **License**: GNU GPL v2+ (as per the original Space Trader license)
- **Permission**: Permission obtained to use original artwork in this port

## Asset Types

### 1. Ship Sprites (54 total)

15 different ship types, each with up to 4 variants:

| Ship Name | Has Shields | Variants |
|-----------|-------------|----------|
| Flea | No | Normal, Damaged |
| Gnat | No | Normal, Damaged |
| Firefly | Yes | Normal, Damaged, Shielded, Shielded+Damaged |
| Mosquito | Yes | All 4 variants |
| Bumblebee | Yes | All 4 variants |
| Beetle | Yes | All 4 variants |
| Hornet | Yes | All 4 variants |
| Grasshopper | Yes | All 4 variants |
| Termite | Yes | All 4 variants |
| Wasp | Yes | All 4 variants |
| Space Monster | No | Normal, Damaged |
| Dragonfly | Yes | All 4 variants |
| Mantis | Yes | All 4 variants |
| Scarab | No | Normal, Damaged |
| Bottle | No | Normal, Damaged |

**Naming Convention:**
- `{ship_name}.png` - Normal
- `{ship_name}_damaged.png` - Damaged
- `{ship_name}_shielded.png` - With shields
- `{ship_name}_shielded_damaged.png` - Shields + damage

### 2. Encounter Icons (5 total)

- `pirate.png` - Pirate encounter icon
- `police.png` - Police encounter icon
- `trader.png` - Trader encounter icon
- `alien.png` - Alien encounter icon
- `special.png` - Special encounter icon

### 3. UI Elements (6 total)

- `system.png` - Solar system marker
- `system_current.png` - Current system marker
- `system_short_range.png` - Short range chart system
- `wormhole.png` - Wormhole indicator
- `wormhole_small.png` - Small wormhole
- `system_visited.png` - Visited system marker

## Resource ID Mapping

From `Rsc/MerchantGraphics.h` in the original repository:

### Ships (BitmapFamily IDs)

```c
FleaBitmapFamily           = 2200
FleaDamagedBitmapFamily    = 2300
GnatBitmapFamily           = 2600
GnatDamagedBitmapFamily    = 2700
FireflyBitmapFamily        = 3000
FireflyDamagedBitmapFamily = 3100
FireflyShieldedBitmapFamily = 3200
FireflyShDamBitmapFamily   = 3300
// ... continues in increments of 400 for each ship
```

The pattern increments by 400 for each ship type.

### Icons (BitmapFamily IDs)

```c
PirateBitmapFamily  = 9500
PoliceBitmapFamily  = 9600
TraderBitmapFamily  = 9700
AlienBitmapFamily   = 9800
SpecialBitmapFamily = 9900
```

Icons increment by 100.

### UI Elements (BitmapFamily IDs)

```c
SystemBitmapFamily              = 1200
CurrentSystemBitmapFamily       = 1300
ShortRangeSystemBitmapFamily    = 1400
WormholeBitmapFamily            = 1600
SmallWormholeBitmapFamily       = 1700
VisitedSystemBitmapFamily       = 1900
```

## Conversion Methods

### Option 1: Automated Extraction (Recommended)

Use the provided Python script:

```bash
python3 tools/extract_palm_resources.py ~/projects/spacetrader
```

**Requirements:**
- `pilrc` (Palm Resource Compiler): `brew install pilrc`
- Python 3 with Pillow: `pip3 install Pillow`

**Process:**
1. Decompiles `Rsc/MerchantColor.rsrc` using pilrc
2. Extracts individual bitmaps by resource ID
3. Converts from BMP to PNG using Pillow
4. Organizes into proper directory structure

### Option 2: Manual Extraction

See `tools/EXTRACTION_GUIDE.md` for detailed manual extraction instructions using:
- Constructor for Palm OS
- Palm OS Emulator
- HackMaster
- pilrc command-line tools

### Option 3: Placeholder Assets

For quick testing without original artwork:

```bash
python3 tools/generate_placeholder_assets.py
```

Creates simple geometric colored sprites matching the naming scheme.

## Asset Loading in Rust

The `src/assets.rs` module handles asset loading:

```rust
// Load assets at startup
let assets = GameAssets::load().await.ok();

// Draw ship sprite
if let Some(ref assets) = assets {
    draw_ship(assets, "flea", x, y, damaged, shielded, scale);
}
```

**Features:**
- Async loading with macroquad
- HashMap-based texture cache
- Fallback to geometric rendering if assets missing
- Support for sprite variants (damaged/shielded)

## File Formats

### Input (Palm OS)

- **Format**: Palm OS BitmapFamily resources
- **File**: `Rsc/MerchantColor.rsrc` (color), `MerchantGray.rsrc` (grayscale), `MerchantBW.rsrc` (B&W)
- **Color Depth**: 16-bit color (preferred), 8-bit grayscale, or 1-bit B&W
- **Resolution**: Variable (typically 48×48 for ships, 24×24 for icons, 16×16 for UI)

### Output (Rust Port)

- **Format**: PNG with transparency
- **Directory Structure**:
  ```
  assets/
  ├── ships/
  │   ├── flea.png
  │   ├── flea_damaged.png
  │   └── ... (54 files total)
  ├── icons/
  │   ├── pirate.png
  │   └── ... (5 files total)
  └── ui/
      ├── system.png
      └── ... (6 files total)
  ```

## Color Palette

The original Palm OS game supports:
- **16-bit color** (5-6-5 RGB) - Best quality
- **8-bit grayscale** - Monochrome Palm devices
- **1-bit B&W** - Original Palm Pilot

The extraction script targets the 16-bit color version from `MerchantColor.rsrc`.

## Technical Details

### Palm OS Resource Format

- Resources stored in `BitmapFamily` structures
- Multiple resolutions/color depths per family
- System selects appropriate version at runtime
- IDs must be consistent across color/gray/BW files

### Conversion Challenges

1. **Binary Format**: Palm OS uses proprietary binary resource format
2. **Bitmap Families**: Multiple bitmaps per resource (different densities)
3. **Color Encoding**: Palm OS 5-6-5 RGB vs standard 8-8-8 RGB
4. **Transparency**: Palm uses color key, PNG uses alpha channel

### Loading Performance

- All assets loaded at startup (~65 PNG files)
- Total size: ~200-500 KB (depending on original bitmap size)
- Async loading with macroquad (non-blocking)
- Cached in memory for immediate access

## Troubleshooting

### "pilrc not found"

```bash
brew install pilrc  # macOS
apt-get install pilrc  # Linux
```

### "Resource ID not found"

The resource IDs must match exactly. Check `MerchantGraphics.h` in the original repo.

### "Pillow not installed"

```bash
pip3 install Pillow
```

### Missing texture at runtime

Check console output for asset loading errors. Ensure:
1. Assets exist in `assets/` directory
2. File permissions are correct
3. PNG files are valid format

## References

- **Original Repository**: https://github.com/videogamepreservation/spacetrader
- **Palm OS SDK**: Available from Palm's archive
- **pilrc**: https://github.com/jichu4n/pilrc
- **Resource IDs**: `Rsc/MerchantGraphics.h` in original repo

## Future Improvements

Potential enhancements:
- [ ] HD texture pack (2x or 4x original resolution)
- [ ] Alternative art styles
- [ ] Animated sprites
- [ ] Particle effects
- [ ] Sound effects extraction (if available in Palm resources)

## Contact

For questions about asset conversion or copyright:
- Original artwork: Alexander Lawrence (al_virtual@yahoo.com)
- Original game: Pieter Spronck (space_trader@hotmail.com)
- Rust port: See GitHub repository
