# Subliminal Nightfall

A dark color scheme for multiple editors and terminals, featuring deep purple-black backgrounds with carefully calibrated accent colors.

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
- Zed theme source: themes/subliminal-nightfall.json (follows https://zed.dev/schema/themes/v0.2.0.json)
- Ghostty theme: ghostty/subliminal-nightfall (palette + background/foreground/cursor)
- Cursor theme: cursor/themes/Subliminal-color-theme.json + cursor/package.json

## Color Palette

Base colors with properly calibrated bright and dim variants:

| Color   | Base    | Bright  | Dim     | Usage                           |
|---------|---------|---------|---------|----------------------------------|
| Red     | #bf616a | #e2848d | #85434a | Errors, deletions               |
| Green   | #a9cfa4 | #ccf2c7 | #769072 | Success, additions              |
| Yellow  | #ffe2a9 | #ffffcc | #b29e76 | Warnings, modifications         |
| Blue    | #6699cc | #89bcef | #476b8e | Info, titles, predictive        |
| Magenta | #f1a5ab | #ffc8ce | #a87377 | Attributes, emphasis            |
| Cyan    | #5fb3b3 | #82d6d6 | #427d7d | Focus borders, operators        |

Background: #191724 (deep purple-black)  
Foreground: #e0def4 (soft white)

## Notes
- Red uses Nord red (#bf616a) for consistency with Nord color palette
- All bright colors are visibly brighter than base (not duplicates)
- Dim colors are 30% darker for proper terminal contrast
- Swift/Rust accents: cyan functions, blue‑green types/variants, pink attributes/macros, sand strings, lavender numbers/inline code
