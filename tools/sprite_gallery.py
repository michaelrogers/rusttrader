#!/usr/bin/env python3
"""
Generate an HTML gallery of all decompressed sprites from SpaceTrader.prc.
Shows each sprite at 1x (actual size) and scaled up for easy inspection.
"""

import struct, base64, io, html
from pathlib import Path
from PIL import Image

def generate_palette():
    """Canonical Palm OS 256-color system palette (from pilrc PalmPalette8bpp)."""
    palette = [(0, 0, 0)] * 256
    # 6×6×6 cube minus black = 215 entries; R slow, G fast, B split across halves
    for i in range(215):
        if i < 108:
            half, local = 0, i
        else:
            half, local = 1, i - 108
        r = (5 - local // 18) * 51
        g = (5 - local % 6) * 51
        b = (5 - (half * 3 + (local % 18) // 6)) * 51
        palette[i] = (r, g, b)
    for j, v in enumerate([17, 34, 68, 85, 119, 136, 170, 187, 221, 238]):
        palette[215 + j] = (v, v, v)
    palette[225] = (192, 192, 192)
    palette[226] = (128, 0, 0)
    palette[227] = (128, 0, 128)
    palette[228] = (0, 128, 0)
    palette[229] = (0, 128, 128)
    return palette

def decompress_rle(data, height, row_bytes):
    """Decompress Palm OS RLE: (count, value) byte pairs per scanline."""
    output = bytearray(row_bytes * height)
    pos = 0
    rows_decoded = 0
    for y in range(height):
        col = 0
        while col < row_bytes:
            if pos + 1 >= len(data):
                break
            count = data[pos]
            value = data[pos + 1]
            pos += 2
            if count == 0:
                count = 1  # safety
            end = min(col + count, row_bytes)
            for i in range(col, end):
                output[y * row_bytes + i] = value
            col = end
        if col >= row_bytes:
            rows_decoded += 1
        else:
            break
    return bytes(output), rows_decoded

def decode_1bpp(rdata, width, height, row_bytes):
    data_offset = 16
    pixels = rdata[data_offset:]
    img = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    for y in range(height):
        for x in range(width):
            byte_idx = y * row_bytes + (x >> 3)
            if byte_idx < len(pixels):
                bit = (pixels[byte_idx] >> (7 - (x & 7))) & 1
                if bit:
                    img.putpixel((x, y), (0, 0, 0, 255))
    return img

def parse_prc(data):
    num_records = struct.unpack_from(">H", data, 76)[0]
    entries = []
    for i in range(num_records):
        off = 78 + i * 10
        rtype = data[off:off + 4].decode("ascii", errors="replace")
        rid = struct.unpack_from(">H", data, off + 4)[0]
        roff = struct.unpack_from(">I", data, off + 6)[0]
        entries.append((rtype, rid, roff))
    resources = []
    for i, (rtype, rid, roff) in enumerate(entries):
        end = entries[i + 1][2] if i + 1 < len(entries) else len(data)
        resources.append((rtype, rid, data[roff:end]))
    return resources

RESOURCE_MAP = {
    # Ships - normal and damaged
    2200: ("Flea", "ships"), 2300: ("Flea (damaged)", "ships"),
    2600: ("Gnat", "ships"), 2700: ("Gnat (damaged)", "ships"),
    3000: ("Firefly", "ships"), 3100: ("Firefly (damaged)", "ships"),
    3200: ("Firefly (shielded)", "ships"), 3300: ("Firefly (sh+dam)", "ships"),
    3400: ("Mosquito", "ships"), 3500: ("Mosquito (damaged)", "ships"),
    3600: ("Mosquito (shielded)", "ships"), 3700: ("Mosquito (sh+dam)", "ships"),
    3800: ("Bumblebee", "ships"), 3900: ("Bumblebee (damaged)", "ships"),
    4000: ("Bumblebee (shielded)", "ships"), 4100: ("Bumblebee (sh+dam)", "ships"),
    4200: ("Beetle", "ships"), 4300: ("Beetle (damaged)", "ships"),
    4400: ("Beetle (shielded)", "ships"), 4500: ("Beetle (sh+dam)", "ships"),
    4600: ("Hornet", "ships"), 4700: ("Hornet (damaged)", "ships"),
    4800: ("Hornet (shielded)", "ships"), 4900: ("Hornet (sh+dam)", "ships"),
    5000: ("Grasshopper", "ships"), 5100: ("Grasshopper (damaged)", "ships"),
    5200: ("Grasshopper (shielded)", "ships"), 5300: ("Grasshopper (sh+dam)", "ships"),
    5400: ("Termite", "ships"), 5500: ("Termite (damaged)", "ships"),
    5600: ("Termite (shielded)", "ships"), 5700: ("Termite (sh+dam)", "ships"),
    5800: ("Wasp", "ships"), 5900: ("Wasp (damaged)", "ships"),
    6000: ("Wasp (shielded)", "ships"), 6100: ("Wasp (sh+dam)", "ships"),
    # Special ships
    6200: ("Monster", "ships"), 6300: ("Monster (damaged)", "ships"),
    6600: ("Dragonfly", "ships"), 6700: ("Dragonfly (damaged)", "ships"),
    6800: ("Dragonfly (shielded)", "ships"), 6900: ("Dragonfly (sh+dam)", "ships"),
    7000: ("Mantis", "ships"), 7100: ("Mantis (damaged)", "ships"),
    7200: ("Mantis (shielded)", "ships"), 7300: ("Mantis (sh+dam)", "ships"),
    7400: ("Scarab", "ships"), 7500: ("Scarab (damaged)", "ships"),
    7800: ("Bottle", "ships"), 7900: ("Bottle (damaged)", "ships"),
    # Icons
    9500: ("Pirate", "icons"), 9600: ("Police", "icons"),
    9700: ("Trader", "icons"), 9800: ("Alien", "icons"), 9900: ("Special", "icons"),
    # UI
    1000: ("About", "ui"), 1100: ("Retire", "ui"),
    1200: ("System", "ui"), 1300: ("Current System", "ui"),
    1400: ("Short Range System", "ui"), 1500: ("SpaceTrader", "ui"),
    1600: ("Wormhole", "ui"), 1700: ("Small Wormhole", "ui"),
    1800: ("Utopia", "ui"), 1900: ("Visited System", "ui"),
    2000: ("Current Visited System", "ui"), 2100: ("Visited Short Range", "ui"),
    11000: ("Attack", "ui"), 11100: ("Attack2", "ui"), 12000: ("Destroyed", "ui"),
}

# Group ships by base name for organized display
SHIP_ORDER = [
    "Flea", "Gnat", "Firefly", "Mosquito", "Bumblebee",
    "Beetle", "Hornet", "Grasshopper", "Termite", "Wasp",
    "Monster", "Dragonfly", "Mantis", "Scarab", "Bottle",
]

def img_to_data_uri(img):
    buf = io.BytesIO()
    img.save(buf, format="PNG")
    b64 = base64.b64encode(buf.getvalue()).decode()
    return f"data:image/png;base64,{b64}"

def decode_bitmap(rdata, palette):
    """Decode a Palm OS bitmap resource into an RGBA PIL Image."""
    if len(rdata) < 16:
        return None, "Too short"

    width, height, row_bytes, flags = struct.unpack_from(">HHHH", rdata, 0)
    pixel_size = rdata[8]
    version = rdata[9]
    compressed = bool(flags & 0x8000)
    has_trans = bool(flags & 0x2000)
    trans_idx = rdata[12] if version >= 1 else 0

    if pixel_size == 1:
        return decode_1bpp(rdata, width, height, row_bytes), "1bpp"

    if pixel_size != 8:
        return None, f"{pixel_size}bpp unsupported"

    data_offset = 16
    if compressed and version >= 2:
        data_offset += 2

    raw = rdata[data_offset:]
    if compressed:
        pixels, rows_decoded = decompress_rle(raw, height, row_bytes)
    else:
        pixels = raw
        rows_decoded = height

    img = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    for y in range(height):
        for x in range(width):
            idx = pixels[y * row_bytes + x] if (y * row_bytes + x) < len(pixels) else 0
            if has_trans and idx == trans_idx:
                continue  # transparent
            r, g, b = palette[idx]
            img.putpixel((x, y), (r, g, b, 255))

    status = "OK" if rows_decoded == height else f"{rows_decoded}/{height} rows"
    return img, status


def main():
    prc_path = Path(__file__).parent / "SpaceTrader.prc"
    data = prc_path.read_bytes()
    resources = parse_prc(data)
    palette = generate_palette()

    # Decode all bitmaps
    decoded = {}
    for rtype, rid, rdata in resources:
        if rtype != "Tbmp":
            continue
        img, status = decode_bitmap(rdata, palette)
        if img:
            width, height = img.size
            decoded[rid] = {
                "img": img,
                "status": status,
                "width": width,
                "height": height,
                "name": RESOURCE_MAP.get(rid, (f"Unknown ({rid})", "unknown"))[0],
                "category": RESOURCE_MAP.get(rid, ("", "unknown"))[1],
            }

    # Build HTML
    lines = []
    lines.append("""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Space Trader — Sprite Gallery</title>
<style>
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background: #1a1a2e; color: #e0e0e0; padding: 20px; }
h1 { text-align: center; font-size: 28px; margin-bottom: 8px; color: #f0c040; }
.subtitle { text-align: center; color: #888; margin-bottom: 30px; font-size: 14px; }
h2 { color: #80c0ff; margin: 30px 0 15px; padding-bottom: 8px; border-bottom: 1px solid #333; }
h3 { color: #a0d0a0; margin: 20px 0 10px; font-size: 16px; }

.ship-group { margin-bottom: 30px; }
.sprite-row { display: flex; flex-wrap: wrap; gap: 16px; align-items: flex-end; margin-bottom: 10px; }

.sprite-card {
    background: #252545;
    border: 1px solid #3a3a5a;
    border-radius: 8px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    min-width: 120px;
    transition: border-color 0.2s;
}
.sprite-card:hover { border-color: #6080c0; }
.sprite-card.warn { border-color: #a08030; }

.sprite-label { font-size: 11px; color: #aaa; margin-bottom: 6px; text-align: center; }
.sprite-dims { font-size: 10px; color: #666; margin-top: 4px; }
.sprite-status { font-size: 10px; margin-top: 4px; }
.sprite-status.ok { color: #60c060; }
.sprite-status.partial { color: #c0a040; }

.img-container {
    display: flex;
    gap: 16px;
    align-items: center;
}
.img-box {
    display: flex;
    flex-direction: column;
    align-items: center;
}
.img-box span { font-size: 9px; color: #666; margin-bottom: 3px; }

.checker {
    background-image:
        linear-gradient(45deg, #2a2a4a 25%, transparent 25%),
        linear-gradient(-45deg, #2a2a4a 25%, transparent 25%),
        linear-gradient(45deg, transparent 75%, #2a2a4a 75%),
        linear-gradient(-45deg, transparent 75%, #2a2a4a 75%);
    background-size: 8px 8px;
    background-position: 0 0, 0 4px, 4px -4px, -4px 0px;
    background-color: #1e1e3e;
    display: inline-block;
    line-height: 0;
    border: 1px solid #3a3a5a;
}
.checker img { image-rendering: pixelated; display: block; }

.section-icons .sprite-row { align-items: center; }
.section-ui .sprite-card { min-width: 100px; }

.toc { text-align: center; margin-bottom: 20px; }
.toc a { color: #80c0ff; margin: 0 8px; text-decoration: none; font-size: 14px; }
.toc a:hover { text-decoration: underline; }

.stats { text-align: center; color: #888; font-size: 13px; margin-bottom: 20px; }
.legend { display: flex; justify-content: center; gap: 20px; font-size: 12px; margin-bottom: 20px; color: #aaa; }
.legend span { display: inline-flex; align-items: center; gap: 4px; }
.legend .dot { width: 10px; height: 10px; border-radius: 50%; display: inline-block; }
.legend .dot.green { background: #60c060; }
.legend .dot.yellow { background: #c0a040; }
</style>
</head>
<body>
<h1>Space Trader — Sprite Gallery</h1>
<p class="subtitle">All decompressed Tbmp resources from SpaceTrader.prc &middot; Palm OS system palette (256 color)</p>
""")

    total = len(decoded)
    full_ok = sum(1 for d in decoded.values() if d["status"] == "OK" or d["status"] == "1bpp")
    partial = total - full_ok
    lines.append(f'<p class="stats">{total} sprites decoded &middot; {full_ok} fully decoded &middot; {partial} partial (transparent bottom rows)</p>')
    lines.append("""
<div class="legend">
    <span><span class="dot green"></span> Fully decoded</span>
    <span><span class="dot yellow"></span> Partial (remaining rows transparent)</span>
</div>
<div class="toc">
    <a href="#ships">Ships</a>
    <a href="#icons">Icons</a>
    <a href="#ui">UI Elements</a>
    <a href="#unmapped">Unmapped</a>
</div>
""")

    # ---- SHIPS ----
    lines.append('<h2 id="ships">Ships</h2>')

    for ship_base in SHIP_ORDER:
        lines.append(f'<div class="ship-group"><h3>{html.escape(ship_base)}</h3>')
        lines.append('<div class="sprite-row">')

        # Find all variants for this ship
        variants = []
        for rid, info in sorted(decoded.items()):
            if info["category"] == "ships" and info["name"].startswith(ship_base):
                variants.append((rid, info))

        for rid, info in variants:
            img = info["img"]
            w, h = info["width"], info["height"]
            scale = max(4, min(8, 200 // max(w, h)))
            uri_1x = img_to_data_uri(img)

            is_partial = info["status"] != "OK" and info["status"] != "1bpp"
            card_class = "sprite-card warn" if is_partial else "sprite-card"
            status_class = "partial" if is_partial else "ok"

            label = info["name"]
            # Shorten label to just the variant part
            if label.startswith(ship_base):
                suffix = label[len(ship_base):].strip()
                if not suffix:
                    suffix = "Normal"
                else:
                    suffix = suffix.lstrip("(").rstrip(")")
            else:
                suffix = label

            lines.append(f'<div class="{card_class}">')
            lines.append(f'  <div class="sprite-label">{html.escape(suffix)}</div>')
            lines.append(f'  <div class="img-container">')
            lines.append(f'    <div class="img-box"><span>1×</span><div class="checker"><img src="{uri_1x}" width="{w}" height="{h}"></div></div>')
            lines.append(f'    <div class="img-box"><span>{scale}×</span><div class="checker"><img src="{uri_1x}" width="{w*scale}" height="{h*scale}"></div></div>')
            lines.append(f'  </div>')
            lines.append(f'  <div class="sprite-dims">{w}×{h} &middot; ID {rid}</div>')
            lines.append(f'  <div class="sprite-status {status_class}">{html.escape(info["status"])}</div>')
            lines.append(f'</div>')

        lines.append('</div></div>')

    # ---- ICONS ----
    lines.append('<h2 id="icons">Encounter Icons</h2>')
    lines.append('<div class="section-icons"><div class="sprite-row">')
    for rid, info in sorted(decoded.items()):
        if info["category"] != "icons":
            continue
        img = info["img"]
        w, h = info["width"], info["height"]
        scale = 8
        uri = img_to_data_uri(img)
        lines.append(f'<div class="sprite-card">')
        lines.append(f'  <div class="sprite-label">{html.escape(info["name"])}</div>')
        lines.append(f'  <div class="img-container">')
        lines.append(f'    <div class="img-box"><span>1×</span><div class="checker"><img src="{uri}" width="{w}" height="{h}"></div></div>')
        lines.append(f'    <div class="img-box"><span>{scale}×</span><div class="checker"><img src="{uri}" width="{w*scale}" height="{h*scale}"></div></div>')
        lines.append(f'  </div>')
        lines.append(f'  <div class="sprite-dims">{w}×{h} &middot; ID {rid}</div>')
        lines.append(f'  <div class="sprite-status ok">OK</div>')
        lines.append(f'</div>')
    lines.append('</div></div>')

    # ---- UI ----
    lines.append('<h2 id="ui">UI Elements</h2>')
    lines.append('<div class="section-ui"><div class="sprite-row">')
    for rid, info in sorted(decoded.items()):
        if info["category"] != "ui":
            continue
        img = info["img"]
        w, h = info["width"], info["height"]
        # Scale small markers up more, keep large images at 1-2x
        if max(w, h) <= 12:
            scale = 10
        elif max(w, h) <= 40:
            scale = 4
        else:
            scale = 2
        uri = img_to_data_uri(img)
        is_partial = info["status"] != "OK" and info["status"] != "1bpp"
        card_class = "sprite-card warn" if is_partial else "sprite-card"
        status_class = "partial" if is_partial else "ok"
        lines.append(f'<div class="{card_class}">')
        lines.append(f'  <div class="sprite-label">{html.escape(info["name"])}</div>')
        lines.append(f'  <div class="img-container">')
        lines.append(f'    <div class="img-box"><span>1×</span><div class="checker"><img src="{uri}" width="{w}" height="{h}"></div></div>')
        lines.append(f'    <div class="img-box"><span>{scale}×</span><div class="checker"><img src="{uri}" width="{w*scale}" height="{h*scale}"></div></div>')
        lines.append(f'  </div>')
        lines.append(f'  <div class="sprite-dims">{w}×{h} &middot; ID {rid}</div>')
        lines.append(f'  <div class="sprite-status {status_class}">{html.escape(info["status"])}</div>')
        lines.append(f'</div>')
    lines.append('</div></div>')

    # ---- UNMAPPED ----
    unmapped = [(rid, info) for rid, info in sorted(decoded.items()) if info["category"] == "unknown"]
    if unmapped:
        lines.append('<h2 id="unmapped">Unmapped Resources</h2>')
        lines.append('<div class="sprite-row">')
        for rid, info in unmapped:
            img = info["img"]
            w, h = info["width"], info["height"]
            scale = max(2, min(6, 160 // max(w, h)))
            uri = img_to_data_uri(img)
            lines.append(f'<div class="sprite-card">')
            lines.append(f'  <div class="sprite-label">ID {rid}</div>')
            lines.append(f'  <div class="img-container">')
            lines.append(f'    <div class="img-box"><span>1×</span><div class="checker"><img src="{uri}" width="{w}" height="{h}"></div></div>')
            lines.append(f'    <div class="img-box"><span>{scale}×</span><div class="checker"><img src="{uri}" width="{w*scale}" height="{h*scale}"></div></div>')
            lines.append(f'  </div>')
            lines.append(f'  <div class="sprite-dims">{w}×{h} &middot; {info["status"]}</div>')
            lines.append(f'</div>')
        lines.append('</div>')

    lines.append('</body></html>')

    out_path = Path(__file__).parent / "sprite_gallery.html"
    out_path.write_text("\n".join(lines))
    print(f"Generated {out_path} ({out_path.stat().st_size // 1024} KB)")


if __name__ == "__main__":
    main()
