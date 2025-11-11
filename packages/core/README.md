# @subliminal/core

Core color definitions and theme utilities for Subliminal Nightfall.

## Overview

This package is the **single source of truth** for all color values used across the Subliminal Nightfall theme ecosystem. It provides:

- Color definitions (base, bright, and dim variants)
- Syntax highlighting colors
- Theme generation utilities
- TypeScript types for type-safe theme consumption

## Usage

### In other packages (monorepo)

```typescript
import { palette, colors, theme } from '@subliminal/core';

// Use flat palette for quick access
const myColor = palette.cyan; // '#5fb3b3'

// Or use structured colors
const redVariant = colors.base.red.bright; // '#e2848d'

// Get complete theme objects
const terminalTheme = theme.terminal;
const editorTheme = theme.editor;
```

### Color Structure

All base ANSI colors follow a three-level system:

```typescript
{
  base: string;   // Standard color
  bright: string; // Brighter variant (30% lighter)
  dim: string;    // Dimmer variant (30% darker)
}
```

### Available Exports

#### Color Definitions

- `baseColors` - ANSI colors (red, green, yellow, blue, magenta, cyan)
- `syntaxColors` - Code syntax colors (teal, blueGreen, lavender, gray)
- `backgroundColors` - Background and foreground colors
- `borderColors` - UI border colors
- `uiColors` - Selection, cursor, line highlight

#### Flat Palette

- `palette` - All colors flattened for easy access

#### Theme Objects

- `theme` - Complete theme with terminal and editor variants
- `terminal` - Terminal-specific theme (16 ANSI colors)
- `editor` - Editor-specific theme (VSCode format)

#### Utilities

- `createTheme()` - Generate complete theme object
- `createTerminalTheme()` - Generate terminal theme
- `createEditorTheme()` - Generate editor theme

## Color Palette

### Base Colors

| Color | Base | Bright | Dim |
|-------|------|--------|-----|
| Red | `#bf616a` | `#e2848d` | `#85434a` |
| Green | `#a9cfa4` | `#ccf2c7` | `#769072` |
| Yellow | `#ffe2a9` | `#ffffcc` | `#b29e76` |
| Blue | `#6699cc` | `#89bcef` | `#476b8e` |
| Magenta | `#f1a5ab` | `#ffc8ce` | `#a87377` |
| Cyan | `#5fb3b3` | `#82d6d6` | `#427d7d` |

### Syntax Colors

| Color | Hex | Usage |
|-------|-----|-------|
| Cyan Teal | `#9ccfd8` | Functions, methods, strings |
| Blue Green | `#31748f` | Keywords, types, constructors |
| Lavender | `#c4a7e7` | Numbers, constants, inline code |
| Gray | `#7f7f7f` | Comments |

### Background & Foreground

| Color | Hex |
|-------|-----|
| Background | `#191724` |
| Background Alt | `#1f1d2e` |
| Background Elevated | `#26233a` |
| Foreground | `#e0def4` |
| Foreground Muted | `#a0a0a0` |
| Foreground Dim | `#7f7f7f` |

## Development

```bash
# Build
pnpm build

# Watch mode
pnpm dev

# Clean
pnpm clean
```

## Philosophy

Colors are calibrated following these principles:

1. **Three-Level System** - Base, bright, and dim for every ANSI color
2. **Purple Warmth** - Deep purple-black background (#191724)
3. **Nord Harmony** - Red borrowed from Nord palette
4. **Subtle Accents** - Muted colors for reduced eye strain
5. **Semantic Precision** - Different colors for different code elements

## License

MIT
