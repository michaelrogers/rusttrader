#!/usr/bin/env python3
"""
Generate placeholder PNG assets for Space Trader Rust port.

These placeholders allow development to continue while the original
Palm OS bitmaps are being extracted and converted.

To extract actual bitmaps from the original game:
1. Get Rsc/MerchantColor.rsrc from the original repository
2. Use pilrc or palm-db-tools to decompile the resource file
3. Extract bitmaps using Palm OS emulator or conversion tools
4. Convert to PNG and replace these placeholders

Original artwork © Alexander Lawrence (al_virtual@yahoo.com)
"""

import os
from PIL import Image, ImageDraw, ImageFont

# Create output directory
ASSETS_DIR = "../assets"
os.makedirs(f"{ASSETS_DIR}/ships", exist_ok=True)
os.makedirs(f"{ASSETS_DIR}/icons", exist_ok=True)
os.makedirs(f"{ASSETS_DIR}/ui", exist_ok=True)

# Ship types from the original game
SHIPS = [
    "flea", "gnat", "firefly", "mosquito", "bumblebee",
    "beetle", "hornet", "grasshopper", "termite", "wasp",
    "monster", "dragonfly", "mantis", "scarab", "bottle"
]

# Ship colors (simple color coding for placeholders)
SHIP_COLORS = {
    "flea": "#888888",
    "gnat": "#AAAAAA",
    "firefly": "#FFAA00",
    "mosquito": "#00AAFF",
    "bumblebee": "#FFFF00",
    "beetle": "#AA0000",
    "hornet": "#FF6600",
    "grasshopper": "#00FF00",
    "termite": "#AAAA00",
    "wasp": "#FF00FF",
    "monster": "#FF0000",
    "dragonfly": "#00FFFF",
    "mantis": "#00AA00",
    "scarab": "#FFAAFF",
    "bottle": "#AAAAAA"
}

def create_ship_sprite(name, size=48, damaged=False, shielded=False):
    """Create a simple ship sprite placeholder."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    color = SHIP_COLORS.get(name, "#888888")
    
    # Draw simple ship shape (triangle pointing up)
    points = [(size//2, 5), (size-5, size-5), (5, size-5)]
    draw.polygon(points, fill=color if not damaged else "#444444")
    
    # Add outline
    draw.polygon(points, outline="#FFFFFF", width=2)
    
    # If damaged, add red marks
    if damaged:
        draw.line([(10, 10), (20, 20)], fill="#FF0000", width=2)
        draw.line([(size-10, 10), (size-20, 20)], fill="#FF0000", width=2)
    
    # If shielded, add shield effect
    if shielded:
        draw.ellipse([2, 2, size-2, size-2], outline="#00FFFF", width=2)
    
    return img

def create_icon(name, size=24):
    """Create simple encounter icon."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    colors = {
        "pirate": "#FF0000",
        "police": "#0000FF",
        "trader": "#00FF00",
        "alien": "#FF00FF",
        "special": "#FFFF00"
    }
    
    color = colors.get(name, "#888888")
    
    # Draw simple icon (circle with letter)
    draw.ellipse([0, 0, size-1, size-1], fill=color, outline="#FFFFFF", width=2)
    
    # Draw first letter
    try:
        font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 16)
    except:
        font = ImageFont.load_default()
    
    letter = name[0].upper()
    bbox = draw.textbbox((0, 0), letter, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]
    text_x = (size - text_width) // 2
    text_y = (size - text_height) // 2 - 2
    
    draw.text((text_x, text_y), letter, fill="#000000", font=font)
    
    return img

def create_system_marker(name, size=16):
    """Create system marker icons."""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    if "wormhole" in name:
        # Draw spiral for wormhole
        draw.ellipse([2, 2, size-2, size-2], outline="#FF00FF", width=2)
        draw.ellipse([4, 4, size-4, size-4], outline="#FF00FF", width=1)
    elif "current" in name:
        # Draw crosshair for current system
        draw.ellipse([0, 0, size-1, size-1], fill="#FFFF00", outline="#FFFFFF", width=2)
        draw.line([(size//2, 0), (size//2, size)], fill="#000000", width=2)
        draw.line([(0, size//2), (size, size//2)], fill="#000000", width=2)
    elif "visited" in name:
        # Draw filled circle for visited
        draw.ellipse([2, 2, size-2, size-2], fill="#888888", outline="#FFFFFF", width=1)
    else:
        # Draw outline circle for unvisited
        draw.ellipse([2, 2, size-2, size-2], outline="#FFFFFF", width=2)
    
    return img

def main():
    print("Generating placeholder assets...")
    
    # Generate ship sprites
    print("Generating ship sprites...")
    for ship in SHIPS:
        # Normal
        img = create_ship_sprite(ship)
        img.save(f"{ASSETS_DIR}/ships/{ship}.png")
        
        # Damaged
        img = create_ship_sprite(ship, damaged=True)
        img.save(f"{ASSETS_DIR}/ships/{ship}_damaged.png")
        
        # Shielded (only for ships that can have shields)
        if ship not in ["flea", "gnat", "monster"]:
            img = create_ship_sprite(ship, shielded=True)
            img.save(f"{ASSETS_DIR}/ships/{ship}_shielded.png")
            
            # Shielded + damaged
            img = create_ship_sprite(ship, damaged=True, shielded=True)
            img.save(f"{ASSETS_DIR}/ships/{ship}_shielded_damaged.png")
    
    # Generate encounter icons
    print("Generating encounter icons...")
    for icon_name in ["pirate", "police", "trader", "alien", "special"]:
        img = create_icon(icon_name)
        img.save(f"{ASSETS_DIR}/icons/{icon_name}.png")
    
    # Generate system markers
    print("Generating system markers...")
    for marker in ["system", "current_system", "visited_system", "wormhole", "small_wormhole"]:
        img = create_system_marker(marker)
        img.save(f"{ASSETS_DIR}/ui/{marker}.png")
    
    # Create attack indicator
    img = Image.new('RGBA', (32, 32), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    draw.polygon([(16, 4), (28, 28), (4, 28)], fill="#FF0000", outline="#FFFFFF", width=2)
    img.save(f"{ASSETS_DIR}/ui/attack.png")
    
    print(f"\n✓ Generated placeholder assets in {ASSETS_DIR}/")
    print("\nThese are simple placeholders. To use original artwork:")
    print("1. Extract bitmaps from Rsc/MerchantColor.rsrc")
    print("2. Convert Palm OS bitmaps to PNG")
    print("3. Replace files in assets/ directory")
    print("4. Original artwork © Alexander Lawrence")

if __name__ == "__main__":
    main()
