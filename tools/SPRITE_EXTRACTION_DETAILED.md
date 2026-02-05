# Extracting Sprites from CloudpilotEMU - Detailed Steps

After you have Space Trader running in CloudpilotEMU, here's how to extract individual sprite images.

## Prerequisites

- Space Trader running in CloudpilotEMU: https://cloudpilot-emu.github.io/
- Image editor (free options: GIMP, Preview.app on Mac, or online tools)
- Screenshot capability (built into all OSs)

## Step 1: Screenshot Each Sprite Type

### Ship Sprites (Most Important - 15 types, 4 variants each)

The game cycles through all ships in several menus:

1. **During Game Start**
   - New Game screen shows Flea
   - Screenshot: normal view of your ship

2. **Shipyard/Equipment Screen** (if available in menu)
   - Browse through available ships
   - Screenshot each unique ship type
   - Get: Flea, Gnat, Firefly, Mosquito, Bumblebee, Beetle, Hornet, Grasshopper, Termite, Wasp
   - Special ships: Monster, Dragonfly, Mantis, Scarab, Bottle

3. **Ship Status Variants**
   - Normal (just fly normally)
   - Damaged (need to take damage from encounter or cheat)
   - Shielded (activate shields if available)
   - Damaged + Shielded (both active)

### Encounter Icons (5 types)

Look in the Encounters screen:
- Pirate icon (red, menacing)
- Police icon (official looking)
- Trader icon (merchant ship)
- Alien icon (strange/otherworldly)
- Special icon (unique encounter)

### UI Elements (6 types)

Check the galaxy map screen:
- Current system marker (highlighted star)
- Visited system marker (previously visited)
- Wormhole marker (special travel point)
- Small wormhole
- Attack indicator (if shown during combat)
- System designation markers

## Step 2: Browser Screenshots (Easiest Method)

### On Mac:
```bash
# Press Cmd+Shift+4 to get crosshair
# Drag to select the sprite area
# Auto-saves to Desktop as PNG
```

### On Windows:
```bash
# Press Windows+Shift+S to open Snip & Sketch
# Drag to select sprite area
# Save as PNG
```

### On Linux:
```bash
# Use gnome-screenshot or flameshot
gnome-screenshot -a    # Interactive selection
```

## Step 3: Crop Sprites to Size

Each sprite in Space Trader is approximately:
- Ships: 48×48 or 64×64 pixels
- Icons: 32×32 or 48×48 pixels
- UI elements: varies

### Using Preview.app (Mac - Easiest):

1. Open screenshot in Preview
2. Select → All or use rectangular selection tool
3. Crop tool from toolbar
4. Adjust to isolate sprite
5. File → Export as PNG
6. Name it appropriately

Example filename: `flea.png`, `flea_damaged.png`, `flea_shielded.png`, `flea_shielded_damaged.png`

### Using GIMP (Cross-platform):

1. Open screenshot: File → Open
2. Rectangle Select Tool
3. Draw rectangle around sprite
4. Image → Crop to Selection
5. Image → Scale Image (if needed to standard size)
6. File → Export As
7. Choose PNG format
8. Name appropriately

### Using Online (No install needed):

https://www.iloveimg.com/crop-image or similar
1. Upload screenshot
2. Drag to crop around sprite
3. Download as PNG

## Step 4: Organize Files

Create folder structure:

```
extracted_sprites/
├── ships/
│   ├── flea.png
│   ├── flea_damaged.png
│   ├── flea_shielded.png
│   ├── flea_shielded_damaged.png
│   ├── gnat.png
│   ├── gnat_damaged.png
│   ... (54 files total)
├── icons/
│   ├── pirate.png
│   ├── police.png
│   ├── trader.png
│   ├── alien.png
│   ├── special.png
└── ui/
    ├── system.png
    ├── system_current.png
    ├── system_visited.png
    ├── wormhole.png
    ├── wormhole_small.png
    └── attack.png
```

## Step 5: Import into Rust Trader

```bash
# Copy all extracted sprites to assets directory
cp -r extracted_sprites/* rusttrader/assets/

# Or manually:
# - Copy ship PNGs to rusttrader/assets/ships/
# - Copy icon PNGs to rusttrader/assets/icons/
# - Copy UI PNGs to rusttrader/assets/ui/
```

## Step 6: Test in Game

```bash
cd rusttrader
cargo run
```

The game will automatically use any PNG files in the assets directory.

## Required File Names

The game expects these exact names (based on code):

### Ships (54 total - 15 types × 4 variants, some with fewer):

```
flea.png, flea_damaged.png, flea_shielded.png, flea_shielded_damaged.png
gnat.png, gnat_damaged.png (no shields)
firefly.png, firefly_damaged.png, firefly_shielded.png, firefly_shielded_damaged.png
mosquito.png, mosquito_damaged.png, mosquito_shielded.png, mosquito_shielded_damaged.png
bumblebee.png, bumblebee_damaged.png, bumblebee_shielded.png, bumblebee_shielded_damaged.png
beetle.png, beetle_damaged.png, beetle_shielded.png, beetle_shielded_damaged.png
hornet.png, hornet_damaged.png, hornet_shielded.png, hornet_shielded_damaged.png
grasshopper.png, grasshopper_damaged.png, grasshopper_shielded.png, grasshopper_shielded_damaged.png
termite.png, termite_damaged.png, termite_shielded.png, termite_shielded_damaged.png
wasp.png, wasp_damaged.png, wasp_shielded.png, wasp_shielded_damaged.png
monster.png, monster_damaged.png (no shields)
dragonfly.png, dragonfly_damaged.png, dragonfly_shielded.png, dragonfly_shielded_damaged.png
mantis.png, mantis_damaged.png, mantis_shielded.png, mantis_shielded_damaged.png
scarab.png, scarab_damaged.png (no shields)
bottle.png, bottle_damaged.png (no shields)
```

### Icons (5 total):

```
alien.png
pirate.png
police.png
special.png
trader.png
```

### UI (6 total):

```
attack.png
current_system.png
system.png
visited_system.png
wormhole.png
wormhole_small.png
```

## Pro Tips

1. **Use Zoom**: Make CloudpilotEMU window larger (use browser zoom) before screenshots
   - Cmd/Ctrl + Plus to zoom in
   - Makes sprites larger, easier to extract

2. **Batch Naming**: Number files while extracting
   - `01_flea.png`, `02_flea_damaged.png`, etc.
   - Easier to organize later

3. **Quality Check**: After importing, run game and verify
   - Missing sprites fall back to geometric placeholders
   - Check console: `cargo run 2>&1 | grep -i asset`

4. **Partial Extract**: Don't need all sprites at once
   - Start with 10-15 ships
   - Game works with mix of real and placeholder sprites
   - Add more as you extract them

5. **Screenshot Regions**: Focus on the game viewport only
   - Ignore UI chrome and buttons
   - Just the actual game graphics area

## Troubleshooting

### Sprites not showing in game
- Check exact filename spelling (case-sensitive on Linux)
- Verify PNG files are valid (try opening in image viewer)
- Check cargo output: `cargo run 2>&1 | grep -i "asset\|texture"`

### Sprites look wrong (colors off, stretched)
- May need to adjust crop area
- Original Palm sprites might be small; game scales them
- If distorted, try re-extracting with better crop

### Can't get game in right state to screenshot
- Restart CloudpilotEMU fresh
- Use cheat codes if available in original game
- Take multiple attempts to get clean shots

### Scaling issues
- Original sprites may be very small (32×32)
- Game's asset system will scale to fit
- Don't worry about exact pixel dimensions

## Time Estimate

- **Ships only (priority)**: 1-2 hours
- **Ships + Icons**: 2-3 hours  
- **Complete (all 65 sprites)**: 3-4 hours

**Recommended**: Start with 10 ships as proof of concept (30 mins), then decide if worth continuing.

## Alternative: Batch Extract via DevTools

If CloudpilotEMU has accessible graphics:

1. Open DevTools (F12)
2. Go to Storage → IndexedDB or LocalStorage
3. Look for graphics/bitmap data
4. Right-click → Save As → PNG

This may work if CloudpilotEMU stores graphics in accessible format.

---

**CloudpilotEMU**: https://cloudpilot-emu.github.io/
**Image Editors**: GIMP (free), Preview.app (Mac), online tools
**Batch Converter**: `tools/batch_convert_bmp_to_png.py` (if you extract BMP files)
