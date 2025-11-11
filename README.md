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

## Color Palette

### Syntax Colors
- ![#9ccfd8](https://via.placeholder.com/100x30/9ccfd8/000000?text=Teal) `#9ccfd8` - Functions, methods, strings
- ![#31748f](https://via.placeholder.com/100x30/31748f/ffffff?text=Blue-Green) `#31748f` - Keywords, types, constructors
- ![#c4a7e7](https://via.placeholder.com/100x30/c4a7e7/000000?text=Lavender) `#c4a7e7` - Numbers, constants, enums
- ![#7f7f7f](https://via.placeholder.com/100x30/7f7f7f/ffffff?text=Gray) `#7f7f7f` - Comments

### Base ANSI Colors
| Color | Base | Bright | Dim |
|-------|------|--------|-----|
| **Red** | ![#bf616a](https://via.placeholder.com/80x25/bf616a/ffffff?text=Base) `#bf616a` | ![#e2848d](https://via.placeholder.com/80x25/e2848d/000000?text=Bright) `#e2848d` | ![#85434a](https://via.placeholder.com/80x25/85434a/ffffff?text=Dim) `#85434a` |
| **Green** | ![#a9cfa4](https://via.placeholder.com/80x25/a9cfa4/000000?text=Base) `#a9cfa4` | ![#ccf2c7](https://via.placeholder.com/80x25/ccf2c7/000000?text=Bright) `#ccf2c7` | ![#769072](https://via.placeholder.com/80x25/769072/ffffff?text=Dim) `#769072` |
| **Yellow** | ![#ffe2a9](https://via.placeholder.com/80x25/ffe2a9/000000?text=Base) `#ffe2a9` | ![#ffffcc](https://via.placeholder.com/80x25/ffffcc/000000?text=Bright) `#ffffcc` | ![#b29e76](https://via.placeholder.com/80x25/b29e76/000000?text=Dim) `#b29e76` |
| **Blue** | ![#6699cc](https://via.placeholder.com/80x25/6699cc/ffffff?text=Base) `#6699cc` | ![#89bcef](https://via.placeholder.com/80x25/89bcef/000000?text=Bright) `#89bcef` | ![#476b8e](https://via.placeholder.com/80x25/476b8e/ffffff?text=Dim) `#476b8e` |
| **Magenta** | ![#f1a5ab](https://via.placeholder.com/80x25/f1a5ab/000000?text=Base) `#f1a5ab` | ![#ffc8ce](https://via.placeholder.com/80x25/ffc8ce/000000?text=Bright) `#ffc8ce` | ![#a87377](https://via.placeholder.com/80x25/a87377/ffffff?text=Dim) `#a87377` |
| **Cyan** | ![#5fb3b3](https://via.placeholder.com/80x25/5fb3b3/ffffff?text=Base) `#5fb3b3` | ![#82d6d6](https://via.placeholder.com/80x25/82d6d6/000000?text=Bright) `#82d6d6` | ![#427d7d](https://via.placeholder.com/80x25/427d7d/ffffff?text=Dim) `#427d7d` |

### UI Colors
- ![#191724](https://via.placeholder.com/100x30/191724/ffffff?text=Background) `#191724` - Deep purple-black
- ![#1f1d2e](https://via.placeholder.com/100x30/1f1d2e/ffffff?text=BG+Alt) `#1f1d2e` - Sidebars, panels
- ![#e0def4](https://via.placeholder.com/100x30/e0def4/000000?text=Foreground) `#e0def4` - Primary text
- ![#484e5b](https://via.placeholder.com/100x30/484e5b/ffffff?text=Selection) `#484e5b` - Text selection
- ![#5fb3b3](https://via.placeholder.com/100x30/5fb3b3/000000?text=Cursor) `#5fb3b3` - Cursor color

## Philosophy

Subliminal Nightfall is designed for extended coding sessions with:
- Deep purple-black backgrounds that reduce eye strain
- Three-level ANSI color system (base/bright/dim) for terminal flexibility
- Carefully calibrated contrast ratios for readability
- Distinct colors for different syntax elements
- Muted accent colors that don't fatigue

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

# Or ensure you have Node.js 22+ and pnpm 10+
node --version  # >= 22
pnpm --version  # >= 10

# Install dependencies
pnpm install
```

### Development

```bash
# Start website dev server
pnpm dev
# Visit http://localhost:4321

# Build all packages
pnpm build

# Preview website
pnpm preview

# Verify symlink setup
./scripts/verify-symlinks.sh

# Or use specific package filters
pnpm --filter website dev
pnpm --filter website build
pnpm --filter website preview
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
- Triggers on changes to `cursor/**` or `packages/core/**`
- Auto-bumps patch version
- Publishes to VS Code Marketplace and Open VSX
- Commits version bump back to repository

**Manual:**
```bash
# Trigger workflow from GitHub Actions tab
# Or manually publish:
cd cursor
vsce publish
ovsx publish
```

### Making Theme Changes

All colors are defined in `packages/core/src/colors.ts` as the single source of truth. Theme files in each platform directory should derive from these core colors.

**Note**: The website package is named `website` (not scoped), so use `pnpm --filter website` when working with it directly.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Links

- **Website**: [subliminal-nightfall.hamrah.com](https://subliminal-nightfall.hamrah.com)
- **Repository**: [github.com/mhamrah/subliminal-nightfall](https://github.com/mhamrah/subliminal-nightfall)
- **Issues**: [github.com/mhamrah/subliminal-nightfall/issues](https://github.com/mhamrah/subliminal-nightfall/issues)
- **VS Code Marketplace**: Search "Subliminal Nightfall"
