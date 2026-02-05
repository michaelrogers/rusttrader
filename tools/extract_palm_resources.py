#!/usr/bin/env python3
"""
Palm OS Resource Extractor for Space Trader
Extracts bitmap resources from Palm OS .rsrc files and converts them to PNG

This script extracts the original Alexander Lawrence bitmap artwork from the
Space Trader Palm OS resource files for use in the Rust port.

Based on resource IDs from MerchantGraphics.h
"""

import os
import sys
import subprocess
from pathlib import Path

# Resource ID mappings from MerchantGraphics.h
SHIP_RESOURCES = {
    # Ship type: (base_id, damaged_id, shielded_id, shielded_damaged_id)
    "flea": (2200, 2300, None, None),  # FleaBitmapFamily, FleaDamagedBitmapFamily
    "gnat": (2600, 2700, None, None),  # No shields
    "firefly": (3000, 3100, 3200, 3300),  # First ship with shields
    "mosquito": (3400, 3500, 3600, 3700),
    "bumblebee": (3800, 3900, 4000, 4100),
    "beetle": (4200, 4300, 4400, 4500),
    "hornet": (4600, 4700, 4800, 4900),
    "grasshopper": (5000, 5100, 5200, 5300),
    "termite": (5400, 5500, 5600, 5700),
    "wasp": (5800, 5900, 6000, 6100),
    "monster": (6200, 6300, None, None),  # Space monster
    "dragonfly": (6600, 6700, 6800, 6900),
    "mantis": (7000, 7100, 7200, 7300),
    "scarab": (7400, 7500, None, None),  # Special ship
    "bottle": (7800, 7900, None, None),  # Special bottle ship
}

ICON_RESOURCES = {
    "pirate": 9500,   # PirateBitmapFamily
    "police": 9600,   # PoliceBitmapFamily
    "trader": 9700,   # TraderBitmapFamily
    "alien": 9800,    # AlienBitmapFamily
    "special": 9900,  # SpecialBitmapFamily
}

UI_RESOURCES = {
    "system": 1200,              # SystemBitmapFamily
    "system_current": 1300,      # CurrentSystemBitmapFamily
    "system_short_range": 1400,  # ShortRangeSystemBitmapFamily
    "wormhole": 1600,            # WormholeBitmapFamily
    "wormhole_small": 1700,      # SmallWormholeBitmapFamily
    "system_visited": 1900,      # VisitedSystemBitmapFamily
}

def check_pilrc():
    """Check if pilrc (Palm Resource Compiler) is installed"""
    try:
        result = subprocess.run(['pilrc', '-h'], capture_output=True, text=True)
        return True
    except FileNotFoundError:
        return False

def install_pilrc_macos():
    """Install pilrc on macOS using Homebrew"""
    print("pilrc not found. Installing via Homebrew...")
    print("If Homebrew is not installed, visit: https://brew.sh")
    
    try:
        subprocess.run(['brew', 'install', 'pilrc'], check=True)
        return True
    except (FileNotFoundError, subprocess.CalledProcessError):
        print("\nFailed to install pilrc automatically.")
        print("Please install manually:")
        print("  brew install pilrc")
        return False

def decompile_rsrc(rsrc_file, output_dir):
    """Decompile a Palm OS .rsrc file using pilrc"""
    print(f"Decompiling {rsrc_file}...")
    
    try:
        subprocess.run([
            'pilrc',
            '-d',  # Decompile mode
            '-o', output_dir,
            rsrc_file
        ], check=True)
        return True
    except subprocess.CalledProcessError as e:
        print(f"Error decompiling: {e}")
        return False

def convert_bmp_to_png(bmp_path, png_path):
    """Convert a BMP file to PNG using PIL/Pillow"""
    try:
        from PIL import Image
        
        img = Image.open(bmp_path)
        # Convert to RGBA if not already
        if img.mode != 'RGBA':
            img = img.convert('RGBA')
        img.save(png_path, 'PNG')
        return True
    except ImportError:
        print("Pillow not installed. Install with: pip install Pillow")
        return False
    except Exception as e:
        print(f"Error converting {bmp_path}: {e}")
        return False

def extract_and_convert(rsrc_dir, output_dir):
    """
    Extract and convert Palm OS resources to PNG
    
    Args:
        rsrc_dir: Path to directory containing .rsrc files
        output_dir: Path to output directory for PNG files
    """
    rsrc_path = Path(rsrc_dir)
    output_path = Path(output_dir)
    
    # Look for MerchantColor.rsrc (color version)
    color_rsrc = rsrc_path / "MerchantColor.rsrc"
    
    if not color_rsrc.exists():
        print(f"Error: {color_rsrc} not found")
        print(f"Please clone the repository: git clone https://github.com/videogamepreservation/spacetrader")
        return False
    
    # Create temp directory for decompiled resources
    temp_dir = output_path / "temp_decompiled"
    temp_dir.mkdir(parents=True, exist_ok=True)
    
    # Decompile the resource file
    if not decompile_rsrc(str(color_rsrc), str(temp_dir)):
        return False
    
    # Create output directories
    (output_path / "ships").mkdir(parents=True, exist_ok=True)
    (output_path / "icons").mkdir(parents=True, exist_ok=True)
    (output_path / "ui").mkdir(parents=True, exist_ok=True)
    
    # Convert ship bitmaps
    print("\nConverting ship sprites...")
    for ship_name, ids in SHIP_RESOURCES.items():
        base_id, damaged_id, shielded_id, shielded_damaged_id = ids
        
        # Base ship
        convert_resource(temp_dir, output_path / "ships" / f"{ship_name}.png", base_id)
        
        # Damaged variant
        convert_resource(temp_dir, output_path / "ships" / f"{ship_name}_damaged.png", damaged_id)
        
        # Shielded variants (if ship has shields)
        if shielded_id:
            convert_resource(temp_dir, output_path / "ships" / f"{ship_name}_shielded.png", shielded_id)
        if shielded_damaged_id:
            convert_resource(temp_dir, output_path / "ships" / f"{ship_name}_shielded_damaged.png", shielded_damaged_id)
    
    # Convert icon bitmaps
    print("\nConverting encounter icons...")
    for icon_name, icon_id in ICON_RESOURCES.items():
        convert_resource(temp_dir, output_path / "icons" / f"{icon_name}.png", icon_id)
    
    # Convert UI bitmaps
    print("\nConverting UI elements...")
    for ui_name, ui_id in UI_RESOURCES.items():
        convert_resource(temp_dir, output_path / "ui" / f"{ui_name}.png", ui_id)
    
    print(f"\n✓ Extraction complete! Assets saved to {output_path}")
    print(f"\nYou can now delete the temp directory: {temp_dir}")
    
    return True

def convert_resource(temp_dir, output_path, resource_id):
    """Convert a single resource by ID"""
    if not resource_id:
        return
    
    # Try to find the BMP file (pilrc typically extracts as bmp###.bmp)
    # The exact naming depends on pilrc's output format
    potential_files = [
        temp_dir / f"bmp{resource_id}.bmp",
        temp_dir / f"{resource_id}.bmp",
        temp_dir / f"Tbmp{resource_id}.bmp",
    ]
    
    for bmp_file in potential_files:
        if bmp_file.exists():
            if convert_bmp_to_png(bmp_file, output_path):
                print(f"  ✓ {output_path.name}")
                return
    
    print(f"  ✗ Resource {resource_id} not found")

def main():
    print("=" * 60)
    print("Space Trader Palm OS Resource Extractor")
    print("=" * 60)
    print()
    print("This tool extracts the original Alexander Lawrence artwork from")
    print("the Palm OS Space Trader resource files.")
    print()
    print("Copyright: Alexander Lawrence (original artwork)")
    print("License: GPL v2+ (as per original Space Trader)")
    print()
    
    # Check for pilrc
    if not check_pilrc():
        if sys.platform == "darwin":
            if not install_pilrc_macos():
                return 1
        else:
            print("Error: pilrc not found")
            print("Please install pilrc:")
            print("  macOS: brew install pilrc")
            print("  Linux: apt-get install pilrc (or equivalent)")
            return 1
    
    # Get paths
    if len(sys.argv) > 1:
        spacetrader_repo = Path(sys.argv[1])
    else:
        spacetrader_repo = Path.home() / "projects" / "spacetrader"
    
    script_dir = Path(__file__).parent
    output_dir = script_dir.parent / "assets"
    
    rsrc_dir = spacetrader_repo / "Rsc"
    
    print(f"Looking for resources in: {rsrc_dir}")
    print(f"Output directory: {output_dir}")
    print()
    
    if not rsrc_dir.exists():
        print(f"Error: {rsrc_dir} not found")
        print()
        print("Please clone the Space Trader repository:")
        print("  cd ~/projects")
        print("  git clone https://github.com/videogamepreservation/spacetrader")
        print()
        print(f"Then run this script again, or specify the path:")
        print(f"  python3 {Path(__file__).name} /path/to/spacetrader")
        return 1
    
    # Extract and convert
    if extract_and_convert(rsrc_dir, output_dir):
        return 0
    else:
        return 1

if __name__ == "__main__":
    sys.exit(main())
