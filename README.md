# Subliminal Nightfall

A dark color scheme featuring deep purple-black backgrounds with carefully calibrated accent colors for optimal contrast and readability.

**[View Interactive Showcase →](https://subliminal-nightfall.hamrah.com)**

## Installation

### VS Code / Cursor

Install from the marketplace:
1. Open Extensions (⌘+Shift+X / Ctrl+Shift+X)
2. Search for "Subliminal Nightfall"
3. Click Install
4. Select theme: Preferences → Color Theme → Subliminal Nightfall

Or install via command line:
```bash
code --install-extension hamrahm.subliminal-nightfall
```

### Zed

1. Open Zed
2. Run command: `zed: extensions`
3. Search for "Subliminal Nightfall"
4. Click Install
5. Select theme: `theme selector: toggle` → Subliminal Nightfall

**Theme Variants:**
- **Subliminal Nightfall** - Standard opaque theme
- **Subliminal Nightfall Blurred** - Transparent blurred variant at ~80% opacity; editor uses a darker code pane and a distinct gutter background for clarity
- **Subliminal Nightfall Hazy** - Higher-opacity blurred variant (≈85% opacity); editor uses a darker code pane and a distinct gutter background for enhanced separation

Or install as dev extension:
```bash
# Clone this repository
git clone https://github.com/mhamrah/subliminal-nightfall.git

# In Zed, run: "zed: install dev extension"
# Select the cloned repository directory
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

#### Using packer.nvim

```lua
use {
  'mhamrah/subliminal-nightfall',
  config = function()
    vim.opt.runtimepath:append(vim.fn.stdpath("data") .. "/site/pack/packer/start/subliminal-nightfall/neovim")
    vim.cmd([[colorscheme subliminal-nightfall]])
  end
}
```

#### Manual installation

```bash
git clone https://github.com/mhamrah/subliminal-nightfall.git
ln -s "$(pwd)/subliminal-nightfall/neovim" ~/.config/nvim/colors/subliminal-nightfall
```

Add to your `init.lua` or `init.vim`:
```lua
vim.opt.runtimepath:append(vim.fn.expand("~/.config/nvim/colors/subliminal-nightfall"))
vim.cmd([[colorscheme subliminal-nightfall]])
```

### Ghostty

Add to `~/.config/ghostty/config`:
```
theme = subliminal-nightfall
```

Or manually configure colors in your Ghostty config:
```bash
# Copy the theme file
cp ghostty/subliminal-nightfall ~/.config/ghostty/themes/

# Add to config
theme = subliminal-nightfall
```

## Terminal Base Palette

### Expanded Editor Syntax Palette
- ![#9ccfd8](https://img.shields.io/badge/Teal-9ccfd8?style=for-the-badge&color=9ccfd8) `#9ccfd8` - Functions, methods, strings
- ![#31748f](https://img.shields.io/badge/Blue--Green-31748f?style=for-the-badge&color=31748f) `#31748f` - Keywords, types, constructors
- ![#c4a7e7](https://img.shields.io/badge/Lavender-c4a7e7?style=for-the-badge&color=c4a7e7) `#c4a7e7` - Numbers, constants, enums
- ![#7f7f7f](https://img.shields.io/badge/Gray-7f7f7f?style=for-the-badge&color=7f7f7f) `#7f7f7f` - Comments

### Base ANSI Colors
| Color | Base | Bright | Dim |
|-------|------|--------|-----|
| **Red** | ![#bf616a](https://img.shields.io/badge/Base-bf616a?style=flat-square&color=bf616a) `#bf616a` | ![#e2848d](https://img.shields.io/badge/Bright-e2848d?style=flat-square&color=e2848d) `#e2848d` | ![#85434a](https://img.shields.io/badge/Dim-85434a?style=flat-square&color=85434a) `#85434a` |
| **Green** | ![#a9cfa4](https://img.shields.io/badge/Base-a9cfa4?style=flat-square&color=a9cfa4) `#a9cfa4` | ![#ccf2c7](https://img.shields.io/badge/Bright-ccf2c7?style=flat-square&color=ccf2c7) `#ccf2c7` | ![#769072](https://img.shields.io/badge/Dim-769072?style=flat-square&color=769072) `#769072` |
| **Yellow** | ![#ffe2a9](https://img.shields.io/badge/Base-ffe2a9?style=flat-square&color=ffe2a9) `#ffe2a9` | ![#ffffcc](https://img.shields.io/badge/Bright-ffffcc?style=flat-square&color=ffffcc) `#ffffcc` | ![#b29e76](https://img.shields.io/badge/Dim-b29e76?style=flat-square&color=b29e76) `#b29e76` |
| **Blue** | ![#6699cc](https://img.shields.io/badge/Base-6699cc?style=flat-square&color=6699cc) `#6699cc` | ![#89bcef](https://img.shields.io/badge/Bright-89bcef?style=flat-square&color=89bcef) `#89bcef` | ![#476b8e](https://img.shields.io/badge/Dim-476b8e?style=flat-square&color=476b8e) `#476b8e` |
| **Magenta** | ![#f1a5ab](https://img.shields.io/badge/Base-f1a5ab?style=flat-square&color=f1a5ab) `#f1a5ab` | ![#ffc8ce](https://img.shields.io/badge/Bright-ffc8ce?style=flat-square&color=ffc8ce) `#ffc8ce` | ![#a87377](https://img.shields.io/badge/Dim-a87377?style=flat-square&color=a87377) `#a87377` |
| **Cyan** | ![#5fb3b3](https://img.shields.io/badge/Base-5fb3b3?style=flat-square&color=5fb3b3) `#5fb3b3` | ![#82d6d6](https://img.shields.io/badge/Bright-82d6d6?style=flat-square&color=82d6d6) `#82d6d6` | ![#427d7d](https://img.shields.io/badge/Dim-427d7d?style=flat-square&color=427d7d) `#427d7d` |

### UI Colors
- ![#191724](https://img.shields.io/badge/Background-191724?style=for-the-badge&color=191724) `#191724` - Deep purple-black
- ![#1f1d2e](https://img.shields.io/badge/Background_Alt-1f1d2e?style=for-the-badge&color=1f1d2e) `#1f1d2e` - Sidebars, panels
- ![#e0def4](https://img.shields.io/badge/Foreground-e0def4?style=for-the-badge&color=e0def4&labelColor=191724) `#e0def4` - Primary text
- ![#484e5b](https://img.shields.io/badge/Selection-484e5b?style=for-the-badge&color=484e5b) `#484e5b` - Text selection
- ![#5fb3b3](https://img.shields.io/badge/Cursor-5fb3b3?style=for-the-badge&color=5fb3b3) `#5fb3b3` - Cursor color

## Philosophy

Subliminal Nightfall is designed for extended coding sessions with:
- Deep purple-black backgrounds that reduce eye strain
- Three-level ANSI color system (base/bright/dim) for terminal flexibility
- Carefully calibrated contrast ratios for readability
- Distinct colors for different syntax elements
- Muted accent colors that don't fatigue

### Theme Variants

**Subliminal Nightfall Hazy** (Zed only) offers a transparent, blurred experience:
- Background blur effects that integrate with your desktop
- ≈85% opacity with a darker code pane and a distinct gutter background for improved readability
- All UI surfaces (editor, panels, toolbar, terminal) support transparency
- Maintains full syntax color readability despite transparency
- Perfect for those who prefer a less visually heavy workspace

## Repository Overview

This is a monorepo containing themes for multiple platforms:

```
subliminal-nightfall/
├── packages/
│   ├── core/           # Color definitions (TypeScript)
│   └── website/        # Showcase site (Astro)
├── zed/themes/         # Zed theme JSON
├── cursor/             # VS Code/Cursor extension
├── neovim/             # Neovim colorscheme
├── ghostty/            # Ghostty terminal theme
└── themes/             # Symlink to zed/themes/ (for Zed extension)
```

**Note**: The `themes/` directory is a symlink to `zed/themes/` to satisfy Zed's extension requirements while keeping the repository organized.

## Contributing

### Setup

```bash
# Install mise (recommended for tool version management)
curl https://mise.run | sh
mise install

# Or ensure you have Node.js, pnpm and Rust managed via mise
mise install

# Install dependencies
mise run install
```

### Development

```bash
# Install tool versions & deps
mise install && mise run install

# Generate themes
mise run gen

# Start website
mise run dev  # Visit http://localhost:4321

# Build everything
mise run build

# Preview website
mise run preview

# Verify symlink setup
mise run verify
```

### Deploying the Website

The showcase website is deployed to Cloudflare Pages at `subliminal-nightfall.hamrah.com`.

**Automatic Deployment:**

Cloudflare Pages automatically deploys when you push to `main`:

```bash
git push origin main
# Cloudflare detects changes and deploys automatically
```

The deployment uses these settings:
- Build command: `pnpm run build:cloudflare`
- Output directory: `website/dist`
- Node version: 22
- pnpm version: 10.11.1

**Manual Deployment via Wrangler CLI:**
```bash
pnpm run build:cloudflare
cd website
npx wrangler pages deploy dist --project-name=subliminal-nightfall
```

### Publishing to VS Code Marketplace

The VS Code/Cursor extension automatically publishes to both marketplaces when theme changes are pushed:

**Automatic (via GitHub Actions):**
- Triggers on changes to `theme.toml`, `packages/core/**`, platform directories, or `website/**`
- Generates themes via Rust CLI and rebuilds website
- Auto-bumps patch versions (Cursor, core if colors changed) and tags `v<cursor-version>` for Zed release PR
- Publishes VS Code/Cursor extension to Marketplace & Open VSX
- Opens automated PR to Zed extensions fork for release
- Commits version bumps back to repository

**Manual (VS Code / Cursor):**
```bash
mise install
mise run gen   # generate themes
cd cursor
vsce publish   # requires VSCE_PAT
ovsx publish   # requires OVSX_PAT
```

**Manual (Website deploy):**
```bash
mise run build:website
# Deploy manually if needed:
cd website
wrangler pages deploy dist --project-name=subliminal-nightfall
```

**Manual (Zed extension tag + PR):**
```bash
# Tag matching cursor version
VERSION=$(node -e "process.stdout.write(require('./cursor/package.json').version)")
git tag "v$VERSION" && git push origin "v$VERSION"
# GitHub Action will open release PR via zed-extension-action
```

### Making Theme Changes

Primary source of truth: `theme.toml`.
Rust generator: `tools/themegen`.

Workflow:
```bash
mise install             # install toolchain versions
mise run install         # install Node deps
mise run gen             # generate all theme artifacts
mise run dev             # start website with generated palette
```
Edit colors / variants (base, blurred, hazy) in `theme.toml`, then re-run `mise run gen`.

Legacy TS palette (`packages/core/src/colors.ts`) will be aligned to consume `theme.toml`; prefer editing the TOML.

Color values (2025-11-14):
- Background: `#191724`
- Background Alt: `#1f1d2e`
- Background Elevated: `#26233a`
- Foreground: `#e0def4`
- Foreground Muted: `#a0a0a0`
- Foreground Dim: `#7f7f7f`
- Selection: `#484e5b`
- Cursor: `#5fb3b3`
- Line Highlight: `#2e3239bf`

ANSI Base:
- Red: `#bf616a` / Bright `#e2848d` / Dim `#85434a`
- Green: `#a9cfa4` / Bright `#ccf2c7` / Dim `#769072`
- Yellow: `#ffe2a9` / Bright `#ffffcc` / Dim `#b29e76`
- Blue: `#6699cc` / Bright `#89bcef` / Dim `#476b8e`
- Magenta: `#f1a5ab` / Bright `#ffc8ce` / Dim `#a87377`
- Cyan: `#5fb3b3` / Bright `#82d6d6` / Dim `#427d7d`

Syntax:
- Teal: `#9ccfd8`
- Blue Green: `#31748f`
- Lavender: `#c4a7e7`
- Gray: `#7f7f7f`

Release tagging: Zed uses `v<cursor-version>`; keep versions in cursor/package.json authoritative.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Links

- **Website**: [subliminal-nightfall.hamrah.com](https://subliminal-nightfall.hamrah.com)
- **Repository**: [github.com/mhamrah/subliminal-nightfall](https://github.com/mhamrah/subliminal-nightfall)
- **Issues**: [github.com/mhamrah/subliminal-nightfall/issues](https://github.com/mhamrah/subliminal-nightfall/issues)
- **VS Code Marketplace**: Search "Subliminal Nightfall"
