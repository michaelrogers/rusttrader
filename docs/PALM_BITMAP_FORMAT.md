# Palm OS Bitmap Format — Decoding Reference

How the bitmap sprites are stored, decompressed, and colorized in the
Space Trader PRC file. Written for agents and contributors working on
the extraction pipeline.

---

## 1. PRC Container

`SpaceTrader.prc` is a Palm OS **PRC** (Palm Resource Collection) file. All
integers are big-endian.

| Offset | Size | Field |
|--------|------|-------|
| 0 | 32 | Database name (null-terminated ASCII) |
| 32 | 2 | Attributes (bit 0 = resource DB) |
| 34 | 2 | Version |
| 36 | 24 | Dates, modification number, offsets |
| 60 | 4 | Type (e.g. `appl`) |
| 64 | 4 | Creator (4-char code, `STrd`) |
| 68 | 8 | Unique-ID seed, next-record-list |
| 76 | 2 | **Number of records** |

After the 78-byte header come **record entries**, each 10 bytes:

| Offset | Size | Field |
|--------|------|-------|
| 0 | 4 | Resource type (ASCII, e.g. `Tbmp`, `tAIB`, `code`) |
| 4 | 2 | Resource ID (`uint16`) |
| 6 | 4 | Byte offset of data from start of file |

A record's data extends from its offset to the next record's offset
(or EOF for the last record).

Space Trader's PRC contains **584 records total, of which 71 are `Tbmp`
(bitmap) resources**.

---

## 2. BitmapType Header

Each `Tbmp` resource is a Palm OS `BitmapType` structure (v0, v1, or v2).
All Space Trader bitmaps are **version 2**. The 16-byte header:

| Offset | Size | Field | Notes |
|--------|------|-------|-------|
| 0 | 2 | `width` | Pixels |
| 2 | 2 | `height` | Pixels |
| 4 | 2 | `rowBytes` | Bytes per row (≥ `⌈width × bpp / 8⌉`, word-aligned) |
| 6 | 2 | `flags` | See below |
| 8 | 1 | `pixelSize` | Bits per pixel: 1, 2, 4, 8, or 16 |
| 9 | 1 | `version` | 0, 1, 2, or 3 |
| 10 | 2 | `nextBitmapOffset` | In **BitmapFamily** chains; 0 = last/only |
| 12 | 1 | `transparencyIndex` | (v1+) palette index treated as transparent |
| 13 | 1 | `compressionType` | (v2+) 0 = none, **1 = RLE**, 2 = unused, 3 = PackBits |
| 14 | 2 | `reserved` | |

### Flags (bitmask)

| Bit | Mask | Meaning |
|-----|------|---------|
| 15 | `0x8000` | **Compressed** |
| 14 | `0x4000` | Has embedded color table |
| 13 | `0x2000` | **Has transparency** |
| 11 | `0x0800` | For-screen only |
| 10 | `0x0400` | Direct color (16bpp) |

### Compressed-size field (v2)

When `compressed` is set and `version ≥ 2`, an extra **2-byte
`compressedSize` field** appears immediately after the 16-byte header
(before any pixel data). Skip it before decompressing.

---

## 3. Pixel Layout

### 8bpp (all ship/icon/screen bitmaps)

Each pixel is one byte — an **index into the 256-color system palette**
(section 5). Pixels are laid out left-to-right, top-to-bottom. Rows are
padded to `rowBytes` width (word-aligned).

### 1bpp (a few small UI markers)

Each pixel is one bit. Bit 7 of byte 0 is the leftmost pixel.  
1 = black (foreground), 0 = white/transparent.

---

## 4. RLE Decompression

All compressed Space Trader bitmaps use **compression type 1**.  Despite
the Palm SDK header naming this "ScanLine" compression, the actual encoding
in these v2 8bpp bitmaps is a straightforward **per-row run-length encoding
(RLE)**:

```
For each row (0..height-1):
    col = 0
    while col < row_bytes:
        count = read_byte()      # number of times to repeat
        value = read_byte()      # the byte value to repeat
        write value × count times starting at col
        col += count
```

- Each `(count, value)` pair occupies 2 bytes.
- Counts for a single row always sum to exactly `rowBytes`.
- After the last row, **two trailing `0x00 0x00` bytes** may appear as
  word-alignment padding — they can be ignored.
- A `count` of 0 should not appear in valid data (treat as end-of-stream
  safety).

### Common pitfall

Palm OS documentation and some emulators describe compression type 1 as a
*scanline delta* encoding (bitmask of changed bytes vs. the previous row).
That format is used in v0/v1 bitmaps but **not** in SpaceTrader's v2
bitmaps. Attempting delta decoding on these produces garbled output with
correct image dimensions but visibly wrong shapes.

---

## 5. Color Palette — `PalmPalette8bpp`

The bitmaps have **no embedded color table** (the `hasColorTable` flag is
clear on every resource in the PRC). All 8bpp pixels reference the canonical
**Palm OS 256-color system palette**, defined in the pilrc resource compiler
source as `PalmPalette8bpp[256][3]`.

### Structure

The palette is a **6 × 6 × 6 color cube** (216 non-black entries) followed
by supplementary grays and named colors:

| Index Range | Count | Description |
|-------------|-------|-------------|
| 0–107 | 108 | Color cube, blue half 1 (B ∈ {255, 204, 153}) |
| 108–214 | 107 | Color cube, blue half 2 (B ∈ {102, 51, 0}) |
| 215–224 | 10 | Supplementary grays |
| 225 | 1 | Silver (192, 192, 192) |
| 226–229 | 4 | Named colors (maroon, purple, dark green, teal) |
| 230–255 | 26 | Black (0, 0, 0) |

### Color cube formula (indices 0–214)

```python
for i in range(215):
    half  = 0 if i < 108 else 1
    local = i if i < 108 else i - 108

    R = (5 - local // 18) * 51       # changes every 18 entries
    G = (5 - local %  6) * 51        # changes every entry (fastest)
    B = (5 - (half*3 + (local%18)//6)) * 51   # split across halves
```

**Axis ordering**: R changes slowest (every 18 entries), G changes fastest
(every entry), and B is split — indices 0–107 cover B ∈ {255, 204, 153},
while 108–214 cover B ∈ {102, 51, 0}.

Key reference colors:

| Index | RGB | Color | Usage |
|-------|-----|-------|-------|
| 0 | (255, 255, 255) | White | Transparency index |
| 93 | (0, 0, 255) | Pure blue | — |
| 101 | (0, 0, 204) | Medium blue | Gnat body |
| 107 | (0, 0, 153) | Dark blue | — |
| 125 | (204, 0, 0) | Red | Thrusters |
| 215 | (17, 17, 17) | Near-black gray | — |
| 230+ | (0, 0, 0) | Black | Background, outlines |

### Common pitfall — wrong axis order

A "standard" 6×6×6 cube with formula `R = (5-i//36)*51, G = (5-(i//6)%6)*51,
B = (5-i%6)*51` produces a **different** index ordering than the actual Palm
OS palette. The most visible symptom: the Gnat (starting ship) appears
**brown** instead of **blue**, because index 101 maps to (153, 51, 0) under
the wrong formula vs. (0, 0, 204) under the correct one.

### Supplementary entries (215–229)

```python
# Grays not present in the 6×6×6 cube
215: (17,17,17)    216: (34,34,34)    217: (68,68,68)
218: (85,85,85)    219: (119,119,119) 220: (136,136,136)
221: (170,170,170) 222: (187,187,187) 223: (221,221,221)
224: (238,238,238)

# Named colors
225: (192,192,192)  # Silver
226: (128,0,0)      # Maroon
227: (128,0,128)    # Purple
228: (0,128,0)      # Dark green
229: (0,128,128)    # Teal

# 230–255: Black (0,0,0)
```

### Authoritative source

The palette comes from **pilrc** (Palm OS resource compiler) source code,
specifically the `PalmPalette8bpp[256][3]` array in `bitmap.c`. This is the
same palette that pilrc uses when compiling BMP/PNG files into Palm OS bitmap
resources — and therefore the palette that was used to create the original
SpaceTrader artwork.

---

## 6. Transparency

All ship sprites and icon bitmaps have the `hasTransparency` flag set with
`transparencyIndex = 0`. Index 0 in the Palm OS palette is **(255, 255, 255)
= white**, so white pixels are transparent.

The extractor writes these as RGBA PNGs with `alpha = 0` for transparent
pixels and `alpha = 255` for opaque pixels.

---

## 7. Resource ID Mapping

Resource IDs come from the `tbmf` (BitmapFamily) identifiers defined in
`Rsc/MerchantGraphics.h` in the
[original source](https://github.com/videogamepreservation/spacetrader).
The PRC's `Tbmp` resource IDs correspond to the family IDs used in the
resource definitions.

**Note**: `MerchantGraphics.h` also defines PICT IDs (e.g.
`FleaBitmap = 5300`, `GnatBitmap = 6100`) which are internal Constructor
identifiers for specific depth variants — they do NOT appear in the PRC.
Only the BitmapFamily IDs (`FleaBitmapFamily = 2200`, etc.) correspond to
extractable `Tbmp` resources.

### Ships (50 resources)

| Ship | IDs | Variants |
|------|-----|----------|
| Flea | 2200, 2300 | normal, damaged |
| Gnat | 2600, 2700 | normal, damaged |
| Firefly | 3000–3300 | normal, damaged, shielded, shielded+damaged |
| Mosquito | 3400–3700 | (same 4 variants) |
| Bumblebee | 3800–4100 | (same 4 variants) |
| Beetle | 4200–4500 | (same 4 variants) |
| Hornet | 4600–4900 | (same 4 variants) |
| Grasshopper | 5000–5300 | (same 4 variants) |
| Termite | 5400–5700 | (same 4 variants) |
| Wasp | 5800–6100 | (same 4 variants) |
| Monster | 6200, 6300 | normal, damaged |
| Dragonfly | 6600–6900 | normal, damaged, shielded, shielded+damaged |
| Mantis | 7000–7300 | (same 4 variants) |
| Scarab | 7400, 7500 | normal, damaged |
| Bottle | 7800, 7900 | normal, damaged |

### Icons (5), UI (16)

| ID | File | Description |
|----|------|-------------|
| 9500 | icons/pirate.png | Pirate encounter (12×12) |
| 9600 | icons/police.png | Police encounter (12×12) |
| 9700 | icons/trader.png | Trader encounter (12×12) |
| 9800 | icons/alien.png | Alien encounter (12×12) |
| 9900 | icons/special.png | Special encounter (12×12) |
| 1000 | ui/about.png | About screen image (32×32) |
| 1100 | ui/retire.png | Retire screen image |
| 1200 | ui/system.png | System marker (5×5) |
| 1300 | ui/current_system.png | Current system marker (7×7) |
| 1400 | ui/system_short_range.png | Short-range chart dot (7×7) |
| 1500 | ui/spacetrader.png | Title/logo image |
| 1600 | ui/wormhole.png | Wormhole marker (7×7) |
| 1700 | ui/small_wormhole.png | Small wormhole (5×5) |
| 1800 | ui/utopia.png | Utopia planet |
| 1900 | ui/visited_system.png | Visited system (5×5) |
| 2000 | ui/current_visited_system.png | Current + visited (7×7) |
| 2100 | ui/visited_short_range_system.png | Visited short-range dot |
| 11000 | ui/attack.png | Attack indicator (9×9) |
| 11100 | ui/attack2.png | Attack indicator alt (9×9) |
| 12000 | ui/destroyed.png | Ship destroyed screen |

---

## 8. Extraction Pipeline Summary

```
SpaceTrader.prc
    │
    ├── parse_prc()          → list of (type, id, bytes)
    │
    ├── filter Tbmp          → 71 bitmap resources
    │
    ├── decode header        → width, height, rowBytes, flags,
    │                          pixelSize, version, compression, transparency
    │
    ├── skip compressedSize  → +2 bytes after header for v2 compressed
    │
    ├── decompress_rle()     → row_bytes × height uncompressed pixel buffer
    │
    ├── apply palette        → PalmPalette8bpp[pixel_index] → (R, G, B)
    │
    ├── apply transparency   → index 0 → alpha=0, else alpha=255
    │
    └── save as RGBA PNG     → assets/{ships,icons,ui}/*.png
```

---

## 9. Tools

| File | Purpose |
|------|---------|
| `tools/extract_prc_bitmaps.py` | Main extractor — PRC → PNG. Run with `--dump-all` for unmapped resources. |
| `tools/sprite_gallery.py` | Generates `sprite_gallery.html` — side-by-side visual gallery of all decoded sprites at 1× and scaled sizes. |
| `tools/generate_placeholder_assets.py` | Creates colored geometric placeholder sprites (for development without PRC). |

Both extractor scripts are standalone Python 3 + Pillow with no other dependencies.

---

## 10. Credits

- **Original artwork**: Alexander Lawrence (al_virtual@yahoo.com)
- **Game**: Pieter Spronck — GPL v2+
- **Palette source**: pilrc (Palm OS resource compiler) — `PalmPalette8bpp[256][3]` in `bitmap.c`
- **PRC source**: [Internet Archive](https://archive.org/details/palm3_SpaceTrader), [PalmDB](https://palmdb.net/app/space-trader)
