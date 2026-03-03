#!/usr/bin/env python3
"""
Palm OS PRC Bitmap Extractor for Space Trader
==============================================

Directly parses the PRC binary format and extracts bitmap resources as PNG files.
No external tools needed — only requires Python 3 and Pillow.

Usage:
    python3 extract_prc_bitmaps.py [path/to/SpaceTrader.prc] [--output-dir path/to/assets]
    python3 extract_prc_bitmaps.py --dump-all   # Extract ALL bitmaps, not just mapped ones

Download the PRC file from:
    https://archive.org/download/palm3_SpaceTrader/SpaceTrader.prc

Original artwork © Alexander Lawrence (al_virtual@yahoo.com)
Game by Pieter Spronck — GPL v2+
"""

import struct
import sys
import os
import argparse
from pathlib import Path

try:
    from PIL import Image
except ImportError:
    print("Error: Pillow is required. Install with: pip install Pillow")
    sys.exit(1)


# ============================================================================
# Palm OS System Color Palette (256 colors)
# ============================================================================

def generate_palm_system_palette():
    """Generate the canonical Palm OS 256-color system palette.

    Source: PalmPalette8bpp[256][3] from pilrc (the Palm OS resource compiler).

    Indices 0–214: 6×6×6 color cube minus pure black (215 entries)
        The cube is organized with R changing slowest (every 18 entries),
        G changing fastest (every entry), and B split across two halves:
          First half  (indices 0–107):   B ∈ {255, 204, 153}
          Second half (indices 108–214): B ∈ {102, 51, 0}

    Index 215–224: supplementary grays (17, 34, 68, 85, 119, 136, 170, 187, 221, 238)
    Index 225: (192, 192, 192) — silver
    Index 226–229: named colors (maroon, purple, dark green, teal)
    Index 230–255: black (0, 0, 0)
    """
    palette = [(0, 0, 0)] * 256

    # 6×6×6 color cube minus black = 215 entries (indices 0–214)
    for i in range(215):
        if i < 108:
            half, local = 0, i
        else:
            half, local = 1, i - 108

        r = (5 - local // 18) * 51
        g = (5 - local % 6) * 51
        b = (5 - (half * 3 + (local % 18) // 6)) * 51
        palette[i] = (r, g, b)

    # Supplementary grays (indices 215–224)
    for j, v in enumerate([17, 34, 68, 85, 119, 136, 170, 187, 221, 238]):
        palette[215 + j] = (v, v, v)

    # Named colors
    palette[225] = (192, 192, 192)  # Silver
    palette[226] = (128, 0, 0)      # Maroon
    palette[227] = (128, 0, 128)    # Purple
    palette[228] = (0, 128, 0)      # Dark green
    palette[229] = (0, 128, 128)    # Teal

    # 230–255: black (already initialized to (0,0,0))
    return palette


# ============================================================================
# PRC Container Parser
# ============================================================================

def parse_prc(data):
    """Parse a Palm OS PRC (resource database) file.

    Returns a list of (resource_type, resource_id, resource_bytes) tuples.

    PRC header (78 bytes):
        0–31   : Database name (null-terminated ASCII)
        32–33  : Attributes (bit 0 = resource DB)
        34–35  : Version
        36–59  : Dates, modification number, offsets
        60–63  : Type (e.g. 'appl')
        64–67  : Creator (4-char code)
        68–75  : Unique ID seed, next record list
        76–77  : Number of records

    Each record entry (10 bytes):
        0–3  : Resource type (ASCII, e.g. 'Tbmp')
        4–5  : Resource ID (uint16 big-endian)
        6–9  : Data offset from start of file
    """
    if len(data) < 78:
        raise ValueError("File too small to be a valid PRC")

    name = data[0:32].split(b"\x00")[0].decode("ascii", errors="replace")
    attrs = struct.unpack_from(">H", data, 32)[0]
    if not (attrs & 0x0001):
        raise ValueError(f"Not a resource database (attrs=0x{attrs:04x})")

    db_type = data[60:64].decode("ascii", errors="replace")
    creator = data[64:68].decode("ascii", errors="replace")
    num_records = struct.unpack_from(">H", data, 76)[0]

    print(f"PRC: '{name}'  type={db_type}  creator={creator}  records={num_records}")

    # Parse record entries
    entries = []
    for i in range(num_records):
        off = 78 + i * 10
        rtype = data[off : off + 4].decode("ascii", errors="replace")
        rid = struct.unpack_from(">H", data, off + 4)[0]
        roff = struct.unpack_from(">I", data, off + 6)[0]
        entries.append((rtype, rid, roff))

    # Resolve data slices (each record extends to the next record's offset)
    resources = []
    for i, (rtype, rid, roff) in enumerate(entries):
        end = entries[i + 1][2] if i + 1 < len(entries) else len(data)
        resources.append((rtype, rid, data[roff:end]))

    return resources


# ============================================================================
# RLE Decompression (Palm OS compression type 1)
# ============================================================================

def decompress_rle(compressed_data, height, row_bytes):
    """Decompress Palm OS compressed bitmap data (RLE per scanline).

    Despite compression type 1 being documented as 'ScanLine', the actual
    encoding used in SpaceTrader.prc (v2 8bpp bitmaps) is a simple per-row
    run-length encoding:

        For each row: a sequence of (count, value) byte pairs.
        Each pair means 'repeat value count times'.
        The counts for each row sum to exactly row_bytes.

    Trailing 00 00 padding may follow the last row (word-alignment).
    """
    output = bytearray(row_bytes * height)
    pos = 0

    for y in range(height):
        row_pos = 0
        while row_pos < row_bytes:
            if pos + 2 > len(compressed_data):
                return bytes(output)  # truncated data
            count = compressed_data[pos]
            value = compressed_data[pos + 1]
            pos += 2
            if count == 0:
                return bytes(output)  # safety: avoid infinite loop
            end = min(row_pos + count, row_bytes)
            for i in range(row_pos, end):
                output[y * row_bytes + i] = value
            row_pos = end

    return bytes(output)


# ============================================================================
# Palm OS Bitmap Decoder
# ============================================================================

def decode_palm_bitmap(resource_data, palette):
    """Decode a Palm OS BitmapType (v0/v1/v2) resource into a PIL Image.

    BitmapType header (16 bytes):
        0–1  : width  (uint16)
        2–3  : height (uint16)
        4–5  : rowBytes (uint16)
        6–7  : flags  (uint16)
                bit 15 (0x8000): compressed
                bit 14 (0x4000): hasColorTable
                bit 13 (0x2000): hasTransparency
                bit 11 (0x0800): forScreen
                bit 10 (0x0400): directColor
        8    : pixelSize (1, 2, 4, 8, or 16)
        9    : version (0, 1, 2, or 3)
        10–11: nextBitmapOffset (for BitmapFamily chains; 0 = last/none)
        12   : transparencyIndex (v1+) or v2 transparencyIndex
        13   : compressionType (v2+): 0=none, 1=ScanLine, 2=RLE, 3=PackBits
        14–15: reserved (v2+)

    Returns (PIL.Image, has_transparency) or (None, False) on failure.
    """
    if len(resource_data) < 16:
        return None, False

    width, height, row_bytes, flags = struct.unpack_from(">HHHH", resource_data, 0)
    pixel_size, version = struct.unpack_from(">BB", resource_data, 8)

    compressed = bool(flags & 0x8000)
    has_color_table = bool(flags & 0x4000)
    has_transparency = bool(flags & 0x2000)
    direct_color = bool(flags & 0x0400)

    transparency_index = 0
    compression_type = 0
    if version >= 1:
        transparency_index = resource_data[12]
    if version >= 2:
        compression_type = resource_data[13]

    if width == 0 or height == 0:
        return None, False

    # Currently only handle 8bpp indexed color
    if pixel_size != 8:
        if pixel_size == 1:
            return _decode_1bpp(resource_data, width, height, row_bytes,
                                compressed, compression_type, has_transparency,
                                transparency_index)
        return None, False

    # Determine where pixel data starts (after header and optional color table)
    data_offset = 16

    local_palette = palette
    if has_color_table:
        if data_offset + 2 > len(resource_data):
            return None, False
        num_entries = struct.unpack_from(">H", resource_data, data_offset)[0]
        data_offset += 2
        if 0 < num_entries <= 256:
            local_palette = list(palette)
            for _ in range(num_entries):
                if data_offset + 4 > len(resource_data):
                    break
                idx = resource_data[data_offset]
                r = resource_data[data_offset + 1]
                g = resource_data[data_offset + 2]
                b = resource_data[data_offset + 3]
                local_palette[idx] = (r, g, b)
                data_offset += 4

    # V2+ compressed bitmaps have a 2-byte compressedSize field after the header
    # (and after any color table). Skip it so we decompress the actual scanline data.
    if compressed and version >= 2:
        data_offset += 2

    pixel_data_raw = resource_data[data_offset:]

    # Decompress if needed
    if compressed:
        if compression_type in (1, 2):
            # Type 1 in SpaceTrader.prc v2 bitmaps uses per-row RLE encoding:
            # (count, value) byte pairs per scanline, counts summing to row_bytes.
            pixel_data = decompress_rle(pixel_data_raw, height, row_bytes)
        elif compression_type == 3:
            print("    Warning: PackBits compression not implemented")
            return None, False
        else:
            # v0/v1 compressed — try RLE
            pixel_data = decompress_rle(pixel_data_raw, height, row_bytes)
    else:
        pixel_data = pixel_data_raw

    # Convert indexed pixels to RGBA image
    img = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    pixels = img.load()

    for y in range(height):
        for x in range(width):
            offset = y * row_bytes + x
            if offset < len(pixel_data):
                idx = pixel_data[offset]
                r, g, b = local_palette[idx]
                if has_transparency and idx == transparency_index:
                    pixels[x, y] = (r, g, b, 0)
                else:
                    pixels[x, y] = (r, g, b, 255)

    return img, has_transparency


def _decode_1bpp(resource_data, width, height, row_bytes,
                 compressed, compression_type, has_transparency, transparency_index):
    """Decode a 1-bit-per-pixel Palm OS bitmap."""
    data_offset = 16
    pixel_data_raw = resource_data[data_offset:]

    if compressed:
        pixel_data = decompress_rle(pixel_data_raw, height, row_bytes)
    else:
        pixel_data = pixel_data_raw

    img = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    pixels = img.load()

    for y in range(height):
        for x in range(width):
            byte_offset = y * row_bytes + (x >> 3)
            bit_pos = 7 - (x & 7)
            if byte_offset < len(pixel_data):
                bit = (pixel_data[byte_offset] >> bit_pos) & 1
                if bit:
                    pixels[x, y] = (0, 0, 0, 255)  # black
                else:
                    if has_transparency:
                        pixels[x, y] = (255, 255, 255, 0)  # transparent
                    else:
                        pixels[x, y] = (255, 255, 255, 255)  # white

    return img, has_transparency


# ============================================================================
# Resource ID → Filename Mapping
# ============================================================================

# Based on 'tbmf' (BitmapFamily) IDs from Rsc/MerchantGraphics.h
# These map directly to the Tbmp resource IDs in the color PRC.

RESOURCE_MAP = {
    # --- Ships ---
    # Flea (no shields)
    2200: "ships/flea.png",
    2300: "ships/flea_damaged.png",
    # Gnat (no shields)
    2600: "ships/gnat.png",
    2700: "ships/gnat_damaged.png",
    # Firefly
    3000: "ships/firefly.png",
    3100: "ships/firefly_damaged.png",
    3200: "ships/firefly_shielded.png",
    3300: "ships/firefly_shielded_damaged.png",
    # Mosquito
    3400: "ships/mosquito.png",
    3500: "ships/mosquito_damaged.png",
    3600: "ships/mosquito_shielded.png",
    3700: "ships/mosquito_shielded_damaged.png",
    # Bumblebee
    3800: "ships/bumblebee.png",
    3900: "ships/bumblebee_damaged.png",
    4000: "ships/bumblebee_shielded.png",
    4100: "ships/bumblebee_shielded_damaged.png",
    # Beetle
    4200: "ships/beetle.png",
    4300: "ships/beetle_damaged.png",
    4400: "ships/beetle_shielded.png",
    4500: "ships/beetle_shielded_damaged.png",
    # Hornet
    4600: "ships/hornet.png",
    4700: "ships/hornet_damaged.png",
    4800: "ships/hornet_shielded.png",
    4900: "ships/hornet_shielded_damaged.png",
    # Grasshopper
    5000: "ships/grasshopper.png",
    5100: "ships/grasshopper_damaged.png",
    5200: "ships/grasshopper_shielded.png",
    5300: "ships/grasshopper_shielded_damaged.png",
    # Termite
    5400: "ships/termite.png",
    5500: "ships/termite_damaged.png",
    5600: "ships/termite_shielded.png",
    5700: "ships/termite_shielded_damaged.png",
    # Wasp
    5800: "ships/wasp.png",
    5900: "ships/wasp_damaged.png",
    6000: "ships/wasp_shielded.png",
    6100: "ships/wasp_shielded_damaged.png",
    # Monster (no shields)
    6200: "ships/monster.png",
    6300: "ships/monster_damaged.png",
    # Dragonfly
    6600: "ships/dragonfly.png",
    6700: "ships/dragonfly_damaged.png",
    6800: "ships/dragonfly_shielded.png",
    6900: "ships/dragonfly_shielded_damaged.png",
    # Mantis
    7000: "ships/mantis.png",
    7100: "ships/mantis_damaged.png",
    7200: "ships/mantis_shielded.png",
    7300: "ships/mantis_shielded_damaged.png",
    # Scarab (no shields)
    7400: "ships/scarab.png",
    7500: "ships/scarab_damaged.png",
    # Bottle (no shields)
    7800: "ships/bottle.png",
    7900: "ships/bottle_damaged.png",

    # --- Encounter Icons ---
    9500: "icons/pirate.png",
    9600: "icons/police.png",
    9700: "icons/trader.png",
    9800: "icons/alien.png",
    9900: "icons/special.png",

    # --- UI Elements ---
    1200: "ui/system.png",
    1300: "ui/current_system.png",
    1400: "ui/system_short_range.png",
    1600: "ui/wormhole.png",
    1700: "ui/small_wormhole.png",
    1900: "ui/visited_system.png",

    # --- Additional UI / Screens ---
    1000: "ui/about.png",
    1100: "ui/retire.png",
    1500: "ui/spacetrader.png",
    1800: "ui/utopia.png",
    2000: "ui/current_visited_system.png",
    2100: "ui/visited_short_range_system.png",
    11000: "ui/attack.png",
    11100: "ui/attack2.png",
    12000: "ui/destroyed.png",
}


# ============================================================================
# Main
# ============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="Extract bitmap sprites from a Palm OS SpaceTrader.prc file"
    )
    parser.add_argument(
        "prc_file",
        nargs="?",
        default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "SpaceTrader.prc"),
        help="Path to SpaceTrader.prc (default: tools/SpaceTrader.prc)",
    )
    parser.add_argument(
        "--output-dir",
        default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "assets"),
        help="Output directory for PNGs (default: assets/)",
    )
    parser.add_argument(
        "--dump-all",
        action="store_true",
        help="Extract ALL Tbmp resources, not just the mapped ones",
    )
    parser.add_argument(
        "--scale",
        type=int,
        default=1,
        help="Nearest-neighbor upscale factor (e.g. 2 or 3)",
    )
    args = parser.parse_args()

    prc_path = args.prc_file
    if not os.path.exists(prc_path):
        print(f"Error: PRC file not found: {prc_path}")
        print()
        print("Download from:")
        print("  https://archive.org/download/palm3_SpaceTrader/SpaceTrader.prc")
        print()
        print("Or run:")
        print(f"  curl -L -o {prc_path} https://archive.org/download/palm3_SpaceTrader/SpaceTrader.prc")
        return 1

    print(f"Reading {prc_path} ({os.path.getsize(prc_path)} bytes)...")
    with open(prc_path, "rb") as f:
        data = f.read()

    # Parse PRC container
    resources = parse_prc(data)

    # Generate system palette
    palette = generate_palm_system_palette()

    # Create output directories
    output_path = Path(args.output_dir)
    (output_path / "ships").mkdir(parents=True, exist_ok=True)
    (output_path / "icons").mkdir(parents=True, exist_ok=True)
    (output_path / "ui").mkdir(parents=True, exist_ok=True)
    if args.dump_all:
        (output_path / "raw").mkdir(parents=True, exist_ok=True)

    # Filter to Tbmp resources
    tbmp_resources = [(rid, rdata) for rtype, rid, rdata in resources if rtype == "Tbmp"]
    print(f"\nFound {len(tbmp_resources)} Tbmp resources")
    print()

    extracted = 0
    skipped = 0
    unmapped = 0

    for rid, rdata in tbmp_resources:
        filename = RESOURCE_MAP.get(rid)
        if filename is None:
            if args.dump_all:
                filename = f"raw/tbmp_{rid}.png"
            else:
                unmapped += 1
                continue

        img, has_transp = decode_palm_bitmap(rdata, palette)
        if img is None:
            w = struct.unpack_from(">H", rdata, 0)[0] if len(rdata) >= 2 else 0
            h = struct.unpack_from(">H", rdata, 2)[0] if len(rdata) >= 4 else 0
            ps = rdata[8] if len(rdata) >= 9 else 0
            print(f"  SKIP  ID {rid:5d}: {w}x{h} {ps}bpp — unsupported format")
            skipped += 1
            continue

        # Post-process: convert white background to transparent for full-screen
        # UI images (about, retire, spacetrader, utopia, destroyed) that were
        # stored without transparency on Palm's white-background screens.
        # This allows them to overlay our dark game background correctly.
        TRANSPARENT_BG_IMAGES = {
            "ui/spacetrader.png", "ui/retire.png", "ui/destroyed.png",
            "ui/utopia.png", "ui/about.png",
        }
        if not has_transp and filename in TRANSPARENT_BG_IMAGES:
            pixels = img.load()
            for y in range(img.height):
                for x in range(img.width):
                    r, g, b, a = pixels[x, y]
                    if r == 255 and g == 255 and b == 255:
                        pixels[x, y] = (255, 255, 255, 0)
            has_transp = True

        # Optional upscaling
        if args.scale > 1:
            img = img.resize(
                (img.width * args.scale, img.height * args.scale),
                Image.NEAREST,
            )

        out_file = output_path / filename
        img.save(str(out_file), "PNG")

        t_str = " [transparent]" if has_transp else ""
        s_str = f" (scaled {args.scale}x)" if args.scale > 1 else ""
        print(f"  OK    ID {rid:5d}: {img.width:3d}x{img.height:<3d} -> {filename}{t_str}{s_str}")
        extracted += 1

    print()
    print(f"Results: {extracted} extracted, {skipped} skipped, {unmapped} unmapped")
    if unmapped > 0 and not args.dump_all:
        print(f"  (use --dump-all to also extract the {unmapped} unmapped resources)")

    return 0


if __name__ == "__main__":
    sys.exit(main())
