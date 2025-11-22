# Icon Setup for VS Code Extension

The extension uses the SN logo icon that matches the website design.

## Icon Files

- `icon.svg` - Source SVG file (128x128)
- `icon.png` - PNG version for VS Code Marketplace (128x128 or 512x512)

## Generating PNG from SVG

To create the PNG file from the SVG:

```bash
cd cursor
convert -background none -density 512 icon.svg -resize 512x512 icon.png
```

This uses ImageMagick's `convert` command to generate a high-quality PNG with transparency.

**Note:** On newer versions of ImageMagick (7+), use `magick convert` instead of `convert`:
```bash
magick convert -background none -density 512 icon.svg -resize 512x512 icon.png
```

### Alternative Methods

If ImageMagick is not available, you can use:

**Inkscape:**
```bash
inkscape icon.svg --export-filename=icon.png --export-width=512 --export-height=512
```

**Online Converter:**
1. Open `icon.svg` in a browser
2. Use an online SVG to PNG converter (e.g., https://convertio.co/svg-png/)
3. Set size to 512x512 pixels
4. Save as `icon.png` in the `cursor/` directory

## Icon Specifications

- **Size**: 512x512 pixels (recommended) or 128x128 pixels (minimum)
- **Format**: PNG with transparency
- **Background**: Transparent
- **Design**: Rounded square with gradient (cyan → blue → lavender) and "SN" text

## Usage

The icon is referenced in `package.json`:
```json
{
  "icon": "icon.png"
}
```

This icon will be used in:
- VS Code Marketplace listing
- Extension details page
- Extension manager in VS Code/Cursor
- Open VSX marketplace


