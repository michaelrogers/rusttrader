# Palm OS Resource Extraction Guide

This guide explains how to extract the original bitmap assets from the Palm OS Space Trader game.

## Prerequisites

You'll need:
1. The original resource files from https://github.com/videogamepreservation/spacetrader
2. Palm OS development tools
3. Image conversion utilities

## Method 1: Using pilrc (Palm Resource Compiler)

### Install pilrc

On macOS:
```bash
brew install pilrc
```

Or download from: http://pilrc.sourceforge.net/

### Decompile Resource Files

```bash
# Clone the original repository
git clone https://github.com/videogamepreservation/spacetrader.git
cd spacetrader/Rsc

# Decompile color resources
pilrc -d MerchantColor.rsrc -o MerchantColor

# This creates a .rcp file and bitmap files
```

### Convert Bitmaps to PNG

The decompilation creates bitmap files. Convert them:

```bash
# Install ImageMagick if needed
brew install imagemagick

# Convert all bitmaps to PNG
for file in *.bmp; do
    convert "$file" "${file%.bmp}.png"
done
```

## Method 2: Using Palm OS Emulator

1. **Get Palm OS Emulator (POSE)**
   - Download from: https://sourceforge.net/projects/pose/
   - Or use CloudpilotEMU (browser-based): https://cloudpilot-emu.github.io/

2. **Install Space Trader**
   - Download SpaceTrader.prc from releases
   - Install into emulator
   - Run the game

3. **Extract Screenshots**
   - Use emulator's screenshot feature
   - Or use screen capture tools

4. **Extract Individual Bitmaps**
   - Use Palm Resource Editor (ResEdit)
   - Or PRC-Tools to dump resources

## Method 3: Using palm-db-tools

```bash
# Install palm-db-tools
git clone https://github.com/jichu4n/palm-db-tools.git
cd palm-db-tools
npm install

# Extract resources
node index.js extract ../spacetrader/Rsc/MerchantColor.rsrc output/

# Find bitmap resources
find output/ -name "*.bmp" -o -name "*.png"
```

## Resource IDs from MerchantGraphics.h

The original game uses these Bitmap Family IDs:

### Ships (ID ranges)
- Flea: 2200-2300
- Gnat: 2600-2700
- Firefly: 3000-3300
- Mosquito: 3400-3700
- Bumblebee: 3800-4100
- Beetle: 4200-4500
- Hornet: 4600-4900
- Grasshopper: 5000-5300
- Termite: 5400-5700
- Wasp: 5800-6100
- Monster: 6200-6300
- Dragonfly: 6600-6900
- Mantis: 7000-7300
- Scarab: 7400-7500
- Bottle: 7800-7900

Each ship has 4 variants:
- Base ID: Normal
- +100: Damaged
- +200: Shielded (if applicable)
- +300: Shielded + Damaged

### Icons
- Pirate: 9500
- Police: 9600
- Trader: 9700
- Alien: 9800
- Special: 9900
- Attack: 11000
- Attack2: 11100

### UI Elements
- System: 1200
- Current System: 1300
- Short Range System: 1400
- Visited System: 1900
- Current Visited System: 2000
- Visited Short Range System: 2100
- Wormhole: 1600
- Small Wormhole: 1700

## Organize Extracted Files

After extraction, organize files:

```bash
assets/
├── ships/
│   ├── flea.png
│   ├── flea_damaged.png
│   ├── gnat.png
│   ├── gnat_damaged.png
│   ├── firefly.png
│   ├── firefly_damaged.png
│   ├── firefly_shielded.png
│   ├── firefly_shielded_damaged.png
│   └── ...
├── icons/
│   ├── pirate.png
│   ├── police.png
│   ├── trader.png
│   ├── alien.png
│   └── special.png
└── ui/
    ├── system.png
    ├── current_system.png
    ├── visited_system.png
    ├── wormhole.png
    └── ...
```

## Automated Extraction Script

Create `extract_resources.sh`:

```bash
#!/bin/bash
set -e

echo "Extracting Space Trader resources..."

# Clone if needed
if [ ! -d "spacetrader" ]; then
    git clone https://github.com/videogamepreservation/spacetrader.git
fi

cd spacetrader/Rsc

# Extract using pilrc
pilrc -d MerchantColor.rsrc -o MerchantColor_out

# Create output directory
mkdir -p ../../assets/{ships,icons,ui}

# TODO: Map resource IDs to filenames and organize
# This requires parsing the .rcp file and resource IDs

echo "Resources extracted. Please verify and organize files."
```

## Important Notes

1. **Copyright**: The original artwork is copyrighted by Alexander Lawrence
2. **Permission**: Ensure you have permission before using the original artwork
3. **Color Depth**: The original has B&W, grayscale, and color versions
4. **Size**: Original bitmaps are small (typically 32x32 or less for Palm OS)
5. **Transparency**: Palm OS uses transparency masks, ensure proper conversion

## Alternative: Use Placeholders

Until original assets are extracted, use the placeholder generator:

```bash
cd tools
python3 generate_placeholder_assets.py
```

This creates simple geometric placeholders that can be replaced later.
