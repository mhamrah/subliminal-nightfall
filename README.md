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
- **Teal** `#9ccfd8` - Functions, methods, strings
- **Blue-Green** `#31748f` - Keywords, types, constructors
- **Lavender** `#c4a7e7` - Numbers, constants, enums
- **Gray** `#7f7f7f` - Comments

### Base ANSI Colors
| Color | Base | Bright | Dim |
|-------|------|--------|-----|
| **Red** | `#bf616a` | `#e2848d` | `#85434a` |
| **Green** | `#a9cfa4` | `#ccf2c7` | `#769072` |
| **Yellow** | `#ffe2a9` | `#ffffcc` | `#b29e76` |
| **Blue** | `#6699cc` | `#89bcef` | `#476b8e` |
| **Magenta** | `#f1a5ab` | `#ffc8ce` | `#a87377` |
| **Cyan** | `#5fb3b3` | `#82d6d6` | `#427d7d` |

### UI Colors
- **Background** `#191724` - Deep purple-black
- **Background Alt** `#1f1d2e` - Sidebars, panels
- **Foreground** `#e0def4` - Primary text
- **Selection** `#484e5b` - Text selection
- **Cursor** `#5fb3b3` - Cursor color

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

**Via Cloudflare Dashboard:**
1. Connect repository to Cloudflare Pages
2. Configure build settings:
   - Build command: `pnpm run build:cloudflare`
   - Output directory: `website/dist`
   - Environment variables: `NODE_VERSION=22`, `PNPM_VERSION=10.11.1`
3. Add custom domain: `subliminal-nightfall.hamrah.com`

**Via Wrangler CLI:**
```bash
pnpm run build:cloudflare
cd website
npx wrangler pages deploy dist --project-name=subliminal-nightfall
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
