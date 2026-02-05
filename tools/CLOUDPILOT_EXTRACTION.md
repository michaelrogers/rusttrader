# CloudpilotEMU Asset Extraction Guide

A practical approach to extract Space Trader artwork using the browser-based Palm OS emulator.

## Why This Method Works

- No native tools to install (pilrc is discontinued, ImageMagick complex setup)
- Browser-based, works on any OS
- Can run the original Space Trader game
- Can screenshot and manually extract sprites
- Takes 2-4 hours but fully achievable

## Setup

### 1. Access CloudpilotEMU
Visit: https://cloudpilot-emu.github.io/

### 2. Get Space Trader .prc File
Option A (Preferred):
- Contact Pieter Spronck or check his website
- Look for SpaceTrader_1.2.2_Color.prc
- Or search abandonware sites (legally questionable but functional)

Option B (Build from source):
- Clone original repo: `git clone https://github.com/videogamepreservation/spacetrader`
- Requires CodeWarrior (not practical, discontinued)

### 3. Install in CloudpilotEMU
1. Open CloudpilotEMU in browser
2. Create new device or use existing
3. Upload SpaceTrader.prc file
4. Run the game

## Extraction Process

### Method A: Screenshot + Manual Crop (Tedious but Works)

1. **Take Screenshots**
   - Play through game, screenshot each ship type
   - Screenshot each game state (normal, damaged, shields)
   - Screenshot all encounter icons
   - Screenshot UI elements

2. **Crop in Image Editor**
   - Use free tools: GIMP, Aseprite, or even Preview.app
   - Save individual sprites as PNG

3. **File Structure**
   - `assets/ships/flea.png`
   - `assets/ships/flea_damaged.png`
   - `assets/ships/flea_shielded.png`
   - etc.

### Method B: Browser DevTools Capture (Faster)

1. **Enable DevTools**
   - F12 or Cmd+Option+I in CloudpilotEMU
   - Go to Application/Storage tab

2. **Inspect Emulator State**
   - CloudpilotEMU stores graphics in memory
   - May be accessible via Canvas/WebGL inspection

3. **Export Graphics**
   - Right-click canvas, "Save image as"
   - Process in batch with Python/ImageMagick

### Method C: Network Intercept (Advanced)

1. **Monitor Network**
   - Open DevTools → Network tab
   - CloudpilotEMU may load assets as files
   - Capture and save .bmp or .png files

## Python Helper Script

Once you have .bmp files (from any method), convert them:

```bash
# Convert all bitmaps to PNG
for file in *.bmp; do
    convert "$file" "${file%.bmp}.png"
done
```

Or using Python:
```bash
python3 tools/batch_convert_bmp_to_png.py
```

## Time Estimate

- **Method A (Screenshots)**: 2-4 hours
  - Takes many screenshots
  - Manual cropping each sprite
  - But guaranteed to work

- **Method B (DevTools)**: 1-3 hours
  - Faster if tools work
  - Less manual labor

- **Method C (Network)**: 1-2 hours
  - Fastest if successful
  - May not work depending on emulator

## Expected Output

Each method should produce:

**Ships (54 total)**:
- 15 ship types
- 4 variants each (normal, damaged, shielded, shielded+damaged)
- Some ships have fewer variants

**Icons (5)**:
- Pirate, Police, Trader, Alien, Special

**UI (6)**:
- System marker, current system, visited system
- Wormhole, small wormhole, attack indicator

## Integration

Once you have PNG files:

```bash
# Copy to assets directory
cp extracted_sprites/* rusttrader/assets/

# Verify game loads them
cargo run
```

The game will automatically use PNG files if they exist, falling back to placeholders if missing.

## Recommended Starting Point

1. **Test the approach**: Extract 2-3 ship sprites as proof of concept
2. **Verify in game**: Run `cargo run` and check if images load
3. **If working**: Continue extracting remaining assets
4. **If not**: Try different method

## Troubleshooting

### Assets not loading in game
- Check file names match expected format
- Verify PNG files are valid images
- Check console for error messages: `cargo run 2>&1 | grep -i asset`

### Screenshot quality too low
- Try higher resolution emulator window
- Use zoom/magnification before capture
- Consider hand-drawing if necessary

### Can't find .prc file
- Try original game's official download
- Search OpenRetro or similar ROM sites
- Contact original author Pieter Spronck

## Alternative: Commission Art

If extraction proves too time-consuming:

**Option**: Hire pixel artist to create original artwork
- Cost: $500-$2000 for 60 ship sprites + icons
- Time: 1-4 weeks
- Quality: Brand new, fully customizable
- Consider for Phase 6 (final release)

---

**CloudpilotEMU**: https://cloudpilot-emu.github.io/
**Original Game**: https://www.spronck.net/spacetrader
**Community**: Check Reddit r/retrogaming or Palm OS forums
