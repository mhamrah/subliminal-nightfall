# Subliminal Nightfall for Neovim

A dark colorscheme for Neovim based on the Subliminal Nightfall theme.

## Installation

### Using [lazy.nvim](https://github.com/folke/lazy.nvim)

```lua
{
  "mhamrah/subliminal-nightfall",
  dir = vim.fn.stdpath("config") .. "/colors",
  lazy = false,
  priority = 1000,
  config = function()
    vim.cmd([[colorscheme subliminal-nightfall]])
  end,
}
```

Or install from GitHub directly:

```lua
{
  "mhamrah/subliminal-nightfall",
  lazy = false,
  priority = 1000,
  config = function()
    -- The colorscheme file is in neovim/colors/
    vim.opt.runtimepath:append(vim.fn.stdpath("data") .. "/lazy/subliminal-nightfall/neovim")
    vim.cmd([[colorscheme subliminal-nightfall]])
  end,
}
```

### Using [packer.nvim](https://github.com/wbthomason/packer.nvim)

```lua
use {
  'mhamrah/subliminal-nightfall',
  config = function()
    vim.opt.runtimepath:append(vim.fn.stdpath("data") .. "/site/pack/packer/start/subliminal-nightfall/neovim")
    vim.cmd([[colorscheme subliminal-nightfall]])
  end
}
```

### Manual Installation

#### Option 1: Copy to Neovim config directory

```bash
# Clone the repository
git clone https://github.com/mhamrah/subliminal-nightfall.git /tmp/subliminal-nightfall

# Copy the colorscheme file
mkdir -p ~/.config/nvim/colors
cp /tmp/subliminal-nightfall/neovim/colors/subliminal-nightfall.lua ~/.config/nvim/colors/

# Clean up
rm -rf /tmp/subliminal-nightfall
```

Then add to your `init.lua`:

```lua
vim.cmd([[colorscheme subliminal-nightfall]])
```

Or in your `init.vim`:

```vim
colorscheme subliminal-nightfall
```

#### Option 2: Add to runtimepath

Clone the repository somewhere and add it to your runtimepath:

```bash
# Clone to your preferred location
git clone https://github.com/mhamrah/subliminal-nightfall.git ~/.local/share/nvim/themes/subliminal-nightfall
```

Then in your `init.lua`:

```lua
vim.opt.runtimepath:append("~/.local/share/nvim/themes/subliminal-nightfall/neovim")
vim.cmd([[colorscheme subliminal-nightfall]])
```

## Features

- Full Treesitter support
- LSP semantic token highlighting
- Built-in terminal colors
- Popular plugin support:
  - Telescope
  - NvimTree
  - GitSigns
  - WhichKey
  - And more!

## Color Palette

- Background: `#191724`
- Foreground: `#e0def4`
- Red: `#bf616a`
- Green: `#a9cfa4`
- Yellow: `#ffe2a9`
- Blue: `#6699cc`
- Magenta: `#f1a5ab`
- Cyan: `#5fb3b3`
- Purple: `#c4a7e7`
- Teal: `#9ccfd8`

## Screenshots

(Add screenshots here if desired)

## License

MIT
