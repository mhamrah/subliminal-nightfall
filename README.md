# Subliminal Nightfall

A dark color scheme for multiple editors and terminals, featuring deep purple-black backgrounds with carefully calibrated accent colors.

## Preview

Visit [hamrah.com/subliminal-nightfall](https://hamrah.com/subliminal-nightfall) to see the theme in action with interactive examples.

## Supported Platforms

- **Zed** - Native theme JSON
- **Ghostty** - Terminal color scheme
- **Cursor/VS Code** - Extension (published to VS Code Marketplace & Open VSX)
- **Neovim** - Lua colorscheme with Treesitter & LSP support

## Installation

### Cursor/VS Code

Install from the marketplace:
- **VS Code Marketplace**: Search for "Subliminal Nightfall" in Extensions
- **Open VSX**: Available at https://open-vsx.org/extension/hamrahm/subliminal-nightfall

Or install manually:
```bash
npm i -g @vscode/vsce
cd cursor
vsce package
# Then in Cursor/VS Code: Extensions → Install from VSIX
```

### Neovim

#### Using lazy.nvim

```lua
{
  "mhamrah/subliminal-nightfall",
  lazy = false,
  priority = 1000,
  config = function()
    vim.opt.runtimepath:append(vim.fn.stdpath("data") .. "/lazy/subliminal-nightfall/neovim")
    vim.cmd([[colorscheme subliminal-nightfall]])
  end,
}
```

#### Manual installation

```bash
git clone https://github.com/mhamrah/subliminal-nightfall.git /tmp/subliminal-nightfall
mkdir -p ~/.config/nvim/colors
cp /tmp/subliminal-nightfall/neovim/colors/subliminal-nightfall.lua ~/.config/nvim/colors/
```

Then in your `init.lua`:
```lua
vim.cmd([[colorscheme subliminal-nightfall]])
```

See [neovim/README.md](neovim/README.md) for detailed installation instructions.

### Zed

```bash
mkdir -p ~/.config/zed/themes
cp zed/themes/subliminal-nightfall.json ~/.config/zed/themes/
```

Then in Zed: Command Palette → "Theme: Select Theme" → Subliminal Nightfall

### Ghostty

```bash
mkdir -p ~/.config/ghostty/themes
cp ghostty/subliminal-nightfall ~/.config/ghostty/themes/
```

Add to `~/.config/ghostty/config`:
```
theme = subliminal-nightfall
```

## Development

### Repository Structure

This is a monorepo organized by editor/platform:

- **`packages/core/`** - Single source of truth for all color definitions (TypeScript)
- **`packages/website/`** - Astro showcase site
- **`zed/themes/`** - Zed theme JSON (source of truth)
- **`themes/`** - Symlink to `zed/themes/` (required for Zed extension)
- **`ghostty/`** - Ghostty terminal color scheme
- **`cursor/`** - Cursor/VS Code extension
- **`neovim/`** - Neovim Lua colorscheme

**Note**: The `themes/` directory at the repository root is a symlink to `zed/themes/`. This allows:
- Organized repo structure with editor-specific directories
- Zed extension compatibility (requires `themes/` at root level with `extension.toml`)
- Single source of truth in `zed/themes/subliminal-nightfall.json`

### Verifying Symlink Setup

To verify the symlink is properly configured:

```bash
./scripts/verify-symlinks.sh
```

This will check that:
- The `themes/` symlink exists and points to `zed/themes/`
- Theme files are accessible through the symlink
- The Zed extension structure is correct

See [`SYMLINK_SETUP.md`](./SYMLINK_SETUP.md) for detailed documentation.

### File Locations

- Zed theme source: `zed/themes/subliminal-nightfall.json` (follows https://zed.dev/schema/themes/v0.2.0.json)
- Ghostty theme: `ghostty/subliminal-nightfall` (palette + background/foreground/cursor)
- Cursor theme: `cursor/themes/Subliminal-color-theme.json` + `cursor/package.json`
- Core colors: `packages/core/src/colors.ts`

## Color Palette

### Base Colors
Carefully calibrated bright and dim variants for optimal terminal and editor contrast.

<table>
<thead>
<tr>
<th>Color</th>
<th>Base</th>
<th>Bright</th>
<th>Dim</th>
<th>Usage</th>
</tr>
</thead>
<tbody>
<tr>
<td><strong>Red</strong></td>
<td><code>#bf616a</code> <img src="https://via.placeholder.com/15/bf616a/bf616a.png" alt="#bf616a"/></td>
<td><code>#e2848d</code> <img src="https://via.placeholder.com/15/e2848d/e2848d.png" alt="#e2848d"/></td>
<td><code>#85434a</code> <img src="https://via.placeholder.com/15/85434a/85434a.png" alt="#85434a"/></td>
<td>Errors, deletions, keywords</td>
</tr>
<tr>
<td><strong>Green</strong></td>
<td><code>#a9cfa4</code> <img src="https://via.placeholder.com/15/a9cfa4/a9cfa4.png" alt="#a9cfa4"/></td>
<td><code>#ccf2c7</code> <img src="https://via.placeholder.com/15/ccf2c7/ccf2c7.png" alt="#ccf2c7"/></td>
<td><code>#769072</code> <img src="https://via.placeholder.com/15/769072/769072.png" alt="#769072"/></td>
<td>Success, additions</td>
</tr>
<tr>
<td><strong>Yellow</strong></td>
<td><code>#ffe2a9</code> <img src="https://via.placeholder.com/15/ffe2a9/ffe2a9.png" alt="#ffe2a9"/></td>
<td><code>#ffffcc</code> <img src="https://via.placeholder.com/15/ffffcc/ffffcc.png" alt="#ffffcc"/></td>
<td><code>#b29e76</code> <img src="https://via.placeholder.com/15/b29e76/b29e76.png" alt="#b29e76"/></td>
<td>Warnings, modifications</td>
</tr>
<tr>
<td><strong>Blue</strong></td>
<td><code>#6699cc</code> <img src="https://via.placeholder.com/15/6699cc/6699cc.png" alt="#6699cc"/></td>
<td><code>#89bcef</code> <img src="https://via.placeholder.com/15/89bcef/89bcef.png" alt="#89bcef"/></td>
<td><code>#476b8e</code> <img src="https://via.placeholder.com/15/476b8e/476b8e.png" alt="#476b8e"/></td>
<td>Info, titles, headings</td>
</tr>
<tr>
<td><strong>Magenta</strong></td>
<td><code>#f1a5ab</code> <img src="https://via.placeholder.com/15/f1a5ab/f1a5ab.png" alt="#f1a5ab"/></td>
<td><code>#ffc8ce</code> <img src="https://via.placeholder.com/15/ffc8ce/ffc8ce.png" alt="#ffc8ce"/></td>
<td><code>#a87377</code> <img src="https://via.placeholder.com/15/a87377/a87377.png" alt="#a87377"/></td>
<td>Attributes, emphasis, booleans</td>
</tr>
<tr>
<td><strong>Cyan</strong></td>
<td><code>#5fb3b3</code> <img src="https://via.placeholder.com/15/5fb3b3/5fb3b3.png" alt="#5fb3b3"/></td>
<td><code>#82d6d6</code> <img src="https://via.placeholder.com/15/82d6d6/82d6d6.png" alt="#82d6d6"/></td>
<td><code>#427d7d</code> <img src="https://via.placeholder.com/15/427d7d/427d7d.png" alt="#427d7d"/></td>
<td>Focus borders, operators</td>
</tr>
</tbody>
</table>

### Syntax Colors
Special colors for code syntax highlighting across all supported editors.

<table>
<thead>
<tr>
<th>Color</th>
<th>Hex</th>
<th>Usage</th>
</tr>
</thead>
<tbody>
<tr>
<td><strong>Cyan Teal</strong></td>
<td><code>#9ccfd8</code> <img src="https://via.placeholder.com/15/9ccfd8/9ccfd8.png" alt="#9ccfd8"/></td>
<td>Functions, methods, strings</td>
</tr>
<tr>
<td><strong>Blue Green</strong></td>
<td><code>#31748f</code> <img src="https://via.placeholder.com/15/31748f/31748f.png" alt="#31748f"/></td>
<td>Keywords, types, constructors</td>
</tr>
<tr>
<td><strong>Lavender</strong></td>
<td><code>#c4a7e7</code> <img src="https://via.placeholder.com/15/c4a7e7/c4a7e7.png" alt="#c4a7e7"/></td>
<td>Numbers, constants, inline code</td>
</tr>
<tr>
<td><strong>Gray</strong></td>
<td><code>#7f7f7f</code> <img src="https://via.placeholder.com/15/7f7f7f/7f7f7f.png" alt="#7f7f7f"/></td>
<td>Comments</td>
</tr>
</tbody>
</table>

### Background & Foreground

<table>
<thead>
<tr>
<th>Color</th>
<th>Hex</th>
<th>Usage</th>
</tr>
</thead>
<tbody>
<tr>
<td><strong>Background</strong></td>
<td><code>#191724</code> <img src="https://via.placeholder.com/15/191724/191724.png" alt="#191724"/></td>
<td>Deep purple-black editor background</td>
</tr>
<tr>
<td><strong>Background Alt</strong></td>
<td><code>#1f1d2e</code> <img src="https://via.placeholder.com/15/1f1d2e/1f1d2e.png" alt="#1f1d2e"/></td>
<td>Sidebar, panels, inactive tabs</td>
</tr>
<tr>
<td><strong>Foreground</strong></td>
<td><code>#e0def4</code> <img src="https://via.placeholder.com/15/e0def4/e0def4.png" alt="#e0def4"/></td>
<td>Soft white text</td>
</tr>
</tbody>
</table>

## Theme Comparisons

### vs Tokyo Night

Subliminal Nightfall takes a different approach from Tokyo Night:

| Aspect | Tokyo Night | Subliminal Nightfall |
|--------|-------------|---------------------|
| **Background** | Blue-gray (#1a1b26) | Purple-black (#191724) - warmer, less harsh |
| **Accent Colors** | Vibrant, high saturation | Muted, carefully calibrated for comfort |
| **Contrast** | High contrast | Medium contrast - easier on eyes |
| **Functions** | Bright blue (#7aa2f7) | Cyan teal (#9ccfd8) - softer |
| **Keywords** | Purple (#bb9af7) | Blue-green (#31748f) - more subtle |
| **Philosophy** | Vibrant Tokyo nightlife | Calm, subliminal depth |

**When to choose Subliminal Nightfall:**
- You prefer warmer, purple-tinted backgrounds
- You want lower eye strain for long coding sessions
- You like subtle, muted accent colors
- You appreciate Nord-inspired color harmony

### vs Rosé Pine

Both themes share a love for muted colors, but differ in execution:

| Aspect | Rosé Pine | Subliminal Nightfall |
|--------|-----------|---------------------|
| **Background** | Rose-tinted (#191724) | Purple-black (#191724) - same base! |
| **Color Philosophy** | Warm, cozy, rose-inspired | Cool, deep, night-inspired |
| **Accent Approach** | Pastel, desaturated | Calibrated with bright/dim variants |
| **Primary Accent** | Rose (#ebbcba) | Cyan (#5fb3b3) |
| **Terminal Support** | 3 variants (main, moon, dawn) | Single variant with 3-level colors |

**When to choose Subliminal Nightfall:**
- You want structured bright/dim color variants
- You prefer cyan/blue accents over rose/gold
- You need consistent terminal ANSI colors
- You like the depth of purple-black

### vs Original Subliminal (iTerm2/Ghostty)

Subliminal Nightfall is a complete reimagining of the original Subliminal theme:

| Aspect | Original Subliminal | Subliminal Nightfall |
|--------|---------------------|---------------------|
| **Scope** | Terminal only | Multi-editor + terminal |
| **Color Variants** | Base colors only | Base + bright + dim variants |
| **Background** | Pure black (#000000) | Purple-black (#191724) - warmer |
| **Syntax Colors** | Limited terminal ANSI | Extensive syntax highlighting |
| **Consistency** | Terminal-focused | Unified across all platforms |
| **Philosophy** | Minimal, pure | Carefully calibrated, comprehensive |

**Subliminal Nightfall improvements:**
- ✅ Consistent experience across VSCode, Neovim, Zed, and terminals
- ✅ Three-level color system (base/bright/dim) for better contrast
- ✅ Dedicated syntax colors for modern languages (Rust, TypeScript, Swift)
- ✅ Warmer purple-black background reduces eye strain
- ✅ Nord-inspired red for better error visibility
- ✅ Treesitter and LSP semantic highlighting support

## Color Philosophy

Subliminal Nightfall follows these principles:

1. **Calibrated Contrast**: Medium contrast for comfortable long coding sessions
2. **Three-Level System**: Base, bright, and dim variants for every ANSI color
3. **Purple Warmth**: Deep purple-black background instead of pure black or blue-gray
4. **Nord Harmony**: Red borrowed from Nord palette for consistency
5. **Subtle Accents**: Muted colors that don't fatigue the eyes
6. **Semantic Precision**: Different colors for functions, types, keywords, and constants

## Development Commands

This project uses pnpm workspaces for monorepo management. See [MONOREPO.md](MONOREPO.md) for detailed architecture documentation.

### Root Level (all packages)

```bash
# Install all dependencies
pnpm install

# Build all packages
pnpm build

# Run website dev server
pnpm dev

# Build specific package
pnpm build:core
pnpm build:vscode
pnpm build:website

# Preview website
pnpm preview

# Clean all build outputs
pnpm clean

# Format all code
pnpm format
```

### Working with Specific Packages

```bash
# Work in website package
pnpm --filter @subliminal/website dev
pnpm --filter @subliminal/website build

# Work in core package
pnpm --filter @subliminal/core build
pnpm --filter @subliminal/core dev

# Recursive operations
pnpm -r build        # Build all packages
pnpm -r clean        # Clean all packages
pnpm -r test         # Run tests in all packages
```

### Website Development

```bash
# From root
pnpm dev
# Visit: http://localhost:4321/subliminal-nightfall

# Build for production
pnpm build:website

# Preview production build
pnpm preview
```

## Notes
- Red uses Nord red (#bf616a) for consistency with Nord color palette
- All bright colors are visibly brighter than base (not duplicates)
- Dim colors are 30% darker for proper terminal contrast
- Swift/Rust accents: cyan functions, blue‑green types/variants, pink attributes/macros, sand strings, lavender numbers/inline code
- Background shares the same hex (#191724) as Rosé Pine but with different color interpretation
