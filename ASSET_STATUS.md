# Asset Status

## Current Status

✅ **Original Artwork Extracted**
- 71 bitmap resources extracted from `SpaceTrader.prc` (Palm OS binary)
- 50 ship sprites (15 types × 2–4 variants)
- 5 encounter icons (pirate, police, trader, alien, special)
- 16 UI elements (system markers, screens, indicators)
- All saved as RGBA PNGs with correct transparency

✅ **Extraction Pipeline Working**
- `tools/extract_prc_bitmaps.py` — PRC parser + RLE decompressor + palette mapper
- `tools/sprite_gallery.py` — visual HTML gallery for verification
- Canonical Palm OS 256-color system palette (`PalmPalette8bpp` from pilrc)
- Full technical reference: `docs/PALM_BITMAP_FORMAT.md`

✅ **Asset Loading System Complete**
- `src/assets.rs` implemented with async loading
- HashMap-based texture caching
- Helper functions for drawing ships, icons, and UI elements
- Integrated into main game loop

## Re-extracting Assets

If you need to re-extract from the PRC:

```bash
pip install Pillow
python3 tools/extract_prc_bitmaps.py
```

See `tools/ASSET_EXTRACTION.md` for download links and options.

## Implementation Notes

### Asset System Design

The asset system works with either original Palm OS bitmaps (extracted) or
placeholder geometric sprites, with fallback:

```rust
let assets = GameAssets::load().await.ok();
```

### Documentation

- `docs/PALM_BITMAP_FORMAT.md` — Full technical reference for bitmap structure,
  RLE decompression, and the 256-color palette
- `tools/ASSET_EXTRACTION.md` — Quick-start extraction guide
- `ASSET_CONVERSION.md` — Asset types, naming conventions, and copyright info
- `assets/README.md` — Asset directory information

## Copyright

- **Original artwork**: © Alexander Lawrence (al_virtual@yahoo.com)
- **License**: GNU GPL v2+
- **Attribution**: Preserved in README.md, assets/README.md, and source comments
