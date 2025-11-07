# Subliminal Nightfall for Neovim

A dark colorscheme for Neovim based on the Subliminal Nightfall theme.

## Installation

### Using [lazy.nvim](https://github.com/folke/lazy.nvim)

```lua
{
  dir = "path/to/subliminal-nightfall/neovim",
  lazy = false,
  priority = 1000,
  config = function()
    vim.cmd([[colorscheme subliminal-nightfall]])
  end,
}
```

### Using [packer.nvim](https://github.com/wbthomason/packer.nvim)

```lua
use {
  'path/to/subliminal-nightfall/neovim',
  config = function()
    vim.cmd([[colorscheme subliminal-nightfall]])
  end
}
```

### Manual Installation

Copy the `colors/subliminal-nightfall.lua` file to your Neovim configuration directory:

```bash
mkdir -p ~/.config/nvim/colors
cp neovim/colors/subliminal-nightfall.lua ~/.config/nvim/colors/
```

Then add to your `init.lua`:

```lua
vim.cmd([[colorscheme subliminal-nightfall]])
```

Or in your `init.vim`:

```vim
colorscheme subliminal-nightfall
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
