# Asset Extraction Guide

## Quick Start (Recommended)

The original Space Trader sprites can be extracted directly from the Palm OS `.prc` binary
using the included Python script. No external tools needed.

### 1. Download the PRC

```bash
curl -L -o tools/SpaceTrader.prc \
  "https://archive.org/download/palm3_SpaceTrader/SpaceTrader.prc"
```

Or download manually from [PalmDB](https://palmdb.net/app/space-trader) (the
`SpaceTrader_1.2.2.zip` contains three PRC files — use the color version).

### 2. Run the Extractor

```bash
# Requires Python 3 + Pillow
pip install Pillow
python3 tools/extract_prc_bitmaps.py
```

This extracts all 71 bitmap resources as PNGs into `assets/ships/`, `assets/icons/`,
and `assets/ui/`, replacing any existing placeholder files.

### Options

```bash
# Extract ALL bitmaps including unmapped ones
python3 tools/extract_prc_bitmaps.py --dump-all

# Upscale for higher-resolution displays
python3 tools/extract_prc_bitmaps.py --scale 3

# Custom PRC file path
python3 tools/extract_prc_bitmaps.py path/to/SpaceTrader.prc
```

## How It Works

The extractor directly parses the Palm OS PRC (Palm Resource Collection) binary format:

1. **PRC container** — 78-byte header + 10-byte record entries listing each resource's
   type, ID, and file offset.

2. **Tbmp resources** — Palm OS `BitmapType` v2 structures with a 16-byte header
   (width, height, rowBytes, flags, pixelSize, version, etc.) followed by pixel data.

3. **8-bit indexed color** — All bitmaps use the canonical Palm OS 256-color system
   palette (`PalmPalette8bpp` from pilrc). No embedded color tables.

4. **RLE compression** — Ship sprites use Palm OS compression type 1: each row is
   stored as `(count, value)` byte pairs where counts sum to rowBytes.

5. **Transparency** — Palette index 0 (white) is the transparency color. The
   extractor converts this to RGBA PNG with alpha=0 for transparent pixels.

For full technical details see [docs/PALM_BITMAP_FORMAT.md](../docs/PALM_BITMAP_FORMAT.md).

## Resource ID Mapping

Resource IDs are derived from `Rsc/MerchantGraphics.h` in the
[original source repository](https://github.com/videogamepreservation/spacetrader).

| Category | Count | ID Range | Notes |
|----------|-------|----------|-------|
| Ships    | 50    | 2200–7900 | 15 types × 2-4 variants |
| Icons    | 5     | 9500–9900 | Pirate, Police, Trader, Alien, Special |
| UI       | 15    | 1000–12000 | System markers, screens, etc. |

### Ship Variants

Each ship type has a base bitmap and a damaged variant. Ships with shields also
have shielded and shielded+damaged variants:

- **No shields**: Flea, Gnat, Monster, Scarab, Bottle (2 variants each)
- **With shields**: Firefly, Mosquito, Bumblebee, Beetle, Hornet, Grasshopper,
  Termite, Wasp, Dragonfly, Mantis (4 variants each)

## Previous Approaches (Superseded)

- **pilrc** — Required the discontinued pilrc tool and `.rsrc` files
  (which are 0-byte Git LFS stubs in the source repo).
- **CloudpilotEMU screenshots** — Manual process of screenshotting each sprite.
- **palm-db-tools** — JavaScript-based, didn't handle bitmap decoding.

## Credits

- **Original artwork**: Alexander Lawrence (al_virtual@yahoo.com)
- **Game**: Pieter Spronck — [GPL v2+](https://www.gnu.org/copyleft/gpl.html)
- **PRC source**: [Internet Archive](https://archive.org/details/palm3_SpaceTrader),
  [PalmDB](https://palmdb.net/app/space-trader)
