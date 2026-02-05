# Assets Directory

This directory is for game assets like images and sounds.

## Original Space Trader Assets

The original Palm OS game includes copyrighted artwork by Alexander Lawrence.

### Extracting Assets from Original Game

If you have permission to use the original artwork:

1. **Get the resource files** from the [original repository](https://github.com/videogamepreservation/spacetrader):
   - `Rsc/MerchantColor.rsrc` (color version)
   - `Rsc/MerchantGray.rsrc` (grayscale version)  
   - `Rsc/MerchantBW.rsrc` (black & white version)

2. **Extract bitmaps** using Palm OS tools:
   - Use `pilrc` (Palm Resource Compiler) to decompile resources
   - Or use Palm OS Emulator with bitmap export tools
   - Or use modern Palm resource extractors

3. **Convert to PNG**:
   ```bash
   # Example with ImageMagick
   convert ship_bitmap.bmp ship_bitmap.png
   ```

4. **Organize by category**:
   ```
   assets/
   ├── ships/
   │   ├── flea.png
   │   ├── flea_damaged.png
   │   ├── gnat.png
   │   └── ...
   ├── icons/
   │   ├── pirate.png
   │   ├── police.png
   │   └── ...
   └── ui/
       ├── system.png
       ├── wormhole.png
       └── ...
   ```

### Asset List from Original Game

**Ships** (14 types, each with 4 variants):
- Flea, Gnat, Firefly, Mosquito, Bumblebee
- Beetle, Hornet, Grasshopper, Termite, Wasp
- Special: Space Monster, Dragonfly, Mantis, Scarab, Bottle

**Icons** (encounter types):
- Pirate, Police, Trader, Alien, Special
- Attack indicators

**System Markers**:
- System (unvisited)
- Current System
- Visited System
- Wormhole (large and small)
- Short-range chart markers

### Using Placeholder Graphics

For development without original assets, the current implementation uses:
- Macroquad's basic drawing primitives
- Colored rectangles and circles for ships
- Simple text-based UI

This allows the game to be fully playable while respecting the original artwork copyright.

## Creating Your Own Assets

If you want to create replacement artwork:

1. **Sprite dimensions** (approximate, based on original):
   - Ships: 32x32 to 64x64 pixels
   - Icons: 16x16 to 24x24 pixels
   - System markers: 8x8 to 16x16 pixels

2. **Art style considerations**:
   - The original has a retro pixel art style
   - Ships have a top-down view
   - Simple, iconic designs work best

3. **Save as PNG** with transparency where appropriate

4. **Load in game** using macroquad:
   ```rust
   let ship_texture = load_texture("assets/ships/flea.png").await?;
   draw_texture(ship_texture, x, y, WHITE);
   ```

## Sound Effects

The original Palm OS game did not include sound effects (Palm OS limitations), so this is an opportunity to add enhancement:

- **Laser fire**: assets/sounds/laser.wav
- **Explosion**: assets/sounds/explosion.wav
- **Warp**: assets/sounds/warp.wav
- **Purchase**: assets/sounds/purchase.wav

Use royalty-free sound libraries like:
- [OpenGameArt.org](https://opengameart.org/)
- [Freesound.org](https://freesound.org/)
- [Kenney.nl](https://kenney.nl/assets)
