#!/usr/bin/env python3
"""
Batch convert BMP images to PNG format
Useful for converting Palm OS bitmap resources extracted via various methods
"""

import os
import sys
from pathlib import Path

def convert_with_imagemagick(input_path, output_path):
    """Convert BMP to PNG using ImageMagick"""
    import subprocess
    try:
        subprocess.run([
            'convert',
            str(input_path),
            str(output_path)
        ], check=True, capture_output=True)
        return True
    except (FileNotFoundError, subprocess.CalledProcessError) as e:
        print(f"  ✗ ImageMagick failed: {e}")
        return False

def convert_with_pillow(input_path, output_path):
    """Convert BMP to PNG using PIL/Pillow"""
    try:
        from PIL import Image
        
        img = Image.open(str(input_path))
        if img.mode != 'RGBA':
            img = img.convert('RGBA')
        img.save(str(output_path), 'PNG')
        return True
    except ImportError:
        print("  ✗ Pillow not installed: pip3 install Pillow")
        return False
    except Exception as e:
        print(f"  ✗ Pillow conversion failed: {e}")
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 batch_convert_bmp_to_png.py <directory>")
        print("Converts all BMP files to PNG in specified directory")
        sys.exit(1)
    
    input_dir = Path(sys.argv[1])
    if not input_dir.exists():
        print(f"Error: {input_dir} not found")
        sys.exit(1)
    
    print("BMP to PNG Batch Converter")
    print("=" * 50)
    print(f"Processing: {input_dir}")
    print()
    
    # Find all BMP files
    bmp_files = sorted(input_dir.glob("**/*.bmp"))
    if not bmp_files:
        print("No BMP files found")
        sys.exit(1)
    
    print(f"Found {len(bmp_files)} BMP files")
    print()
    
    converted = 0
    failed = 0
    
    for bmp_file in bmp_files:
        png_file = bmp_file.with_suffix('.png')
        
        # Skip if already exists
        if png_file.exists():
            print(f"Skip: {bmp_file.name} (PNG already exists)")
            continue
        
        print(f"Converting: {bmp_file.name}")
        
        # Try PIL first (more reliable)
        if convert_with_pillow(bmp_file, png_file):
            print(f"  ✓ Saved to {png_file.name}")
            converted += 1
        # Fall back to ImageMagick
        elif convert_with_imagemagick(bmp_file, png_file):
            print(f"  ✓ Saved to {png_file.name}")
            converted += 1
        else:
            print(f"  ✗ Failed to convert")
            failed += 1
    
    print()
    print("=" * 50)
    print(f"Results: {converted} converted, {failed} failed")
    
    if failed > 0:
        print("\nTroubleshooting:")
        print("1. Install Pillow: pip3 install Pillow")
        print("2. Install ImageMagick: brew install imagemagick")
        print("3. Verify BMP files are valid")

if __name__ == "__main__":
    main()
