# Icon Setup Summary

The SN logo icon has been set up for use across all distribution channels.

## Files Created/Updated

### VS Code / Cursor Extension
- ✅ `cursor/icon.svg` - Source SVG icon (128x128, matches website design)
- ✅ `cursor/package.json` - Updated with `"icon": "icon.png"`
- ✅ `cursor/generate-icon.js` - Script to generate PNG from SVG
- ✅ `cursor/ICON_README.md` - Detailed instructions for generating PNG
- ⚠️ `cursor/icon.png` - **NEEDS TO BE GENERATED** (see below)

### Website
- ✅ `website/public/favicon.svg` - Updated to match SN logo design

## Next Steps

### 1. Generate PNG Icon for VS Code Extension

The VS Code Marketplace requires a PNG file. Generate it using one of these methods:

**Option A: Using Node.js (if you have sharp installed)**
```bash
cd cursor
npm install sharp
node generate-icon.js
```

**Option B: Using ImageMagick**
```bash
cd cursor
convert -background none -density 512 icon.svg -resize 512x512 icon.png
```

**Option C: Using Inkscape**
```bash
cd cursor
inkscape icon.svg --export-filename=icon.png --export-width=512 --export-height=512
```

**Option D: Online Converter**
1. Open `cursor/icon.svg` in a browser
2. Use an online SVG to PNG converter
3. Set size to 512x512 pixels
4. Save as `cursor/icon.png`

### 2. Verify Icon Usage

The icon will be used in:
- ✅ VS Code Marketplace listing (`cursor/package.json` → `"icon": "icon.png"`)
- ✅ Open VSX marketplace
- ✅ Extension manager in VS Code/Cursor
- ✅ Extension details page
- ✅ Website favicon (already updated)

### 3. Test the Extension

After generating `icon.png`:
```bash
cd cursor
vsce package
```

The generated `.vsix` file should include the icon, and it will appear in the marketplace.

## Icon Design

The icon matches the website's hero logo:
- **Shape**: Rounded square (24px border radius)
- **Gradient**: Cyan (#5fb3b3) → Blue (#6699cc) → Lavender (#c4a7e7)
- **Text**: "SN" in bold, dark background color (#191724)
- **Size**: 512x512 pixels (recommended) or 128x128 pixels (minimum)

## Notes

- The `icon.png` file is gitignored (see `cursor/.gitignore`) since it's generated
- The SVG source (`icon.svg`) is tracked in git
- The icon design is consistent across website and extension

