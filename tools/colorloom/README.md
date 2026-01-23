# ColorLoom

A powerful Rust CLI tool for generating consistent color themes across multiple editors and terminals from a single TOML configuration file.

## Overview

ColorLoom is the theme generation engine behind [Subliminal Nightfall](https://github.com/mhamrah/subliminal-nightfall). It reads a centralized `theme.toml` configuration and generates native theme files for:

- **Zed** - Full theme JSON with opaque, hazy, and cloudy variants
- **VS Code / Cursor** - Color theme JSON with comprehensive UI coverage
- **Neovim** - Lua colorscheme with highlight groups
- **Ghostty** - Terminal palette configuration
- **Website** - JSON palette for web showcase

## Features

- **Single Source of Truth**: Define colors once in `theme.toml`, generate everywhere
- **Variant Support**: Automatically generates base, hazy (75% opacity), and cloudy (90% opacity) theme variants
- **Blur Effects**: Full support for Zed's `background.appearance: "blurred"` with proper transparency
- **Comprehensive Coverage**: Generates 100+ color settings per target for complete UI theming
- **Type-Safe**: Written in Rust with full serde serialization

## Installation

```bash
# From crates.io (when published)
cargo install colorloom

# From source
cd tools/colorloom
cargo install --path .
```

## Usage

```bash
# Generate all themes (default command)
colorloom generate

# Validate configuration
colorloom validate

# List targets and variants
colorloom list

# Use custom config file
colorloom -c custom-theme.toml generate
```

## Configuration

ColorLoom reads from `theme.toml` in the current directory:

```toml
version = "1"

[meta]
name = "Subliminal Nightfall"
author = "Michael Hamrah"
description = "A dark color scheme with purple-black backgrounds"
license = "MIT"

# ANSI terminal colors with base/bright/dim variants
[palette.base.ansi.red]
base = "#bf616a"
bright = "#e2848d"
dim = "#85434a"

[palette.base.ansi.green]
base = "#a9cfa4"
bright = "#ccf2c7"
dim = "#769072"

# ... more ANSI colors

# Syntax highlighting colors
[palette.syntax]
teal = "#9ccfd8"
blue_green = "#6699cc"
lavender = "#c4a7e7"
gray = "#7f7f7f"

# UI colors
[palette.ui]
background = "#191724"
background_alt = "#1f1d2e"
background_elevated = "#26233a"
foreground = "#e0def4"
foreground_muted = "#a0a0a0"
foreground_dim = "#7f7f7f"
selection = "#484e5b"
cursor = "#5fb3b3"
line_highlight = "#2e3239bf"

# Border colors
[palette.border]
border = "#484e5b"
border_variant = "#363b45"
border_focused = "#6699cc"
border_selected = "#5fb3b3"

# Theme variants
[[variants]]
name = "base"
alpha = 1.0

[[variants]]
name = "hazy"
alpha = 0.75
blur_radius = 20

[[variants]]
name = "cloudy"
alpha = 0.90
blur_radius = 12

# Target configurations
[[targets]]
id = "zed"
enabled = true
path = "zed/themes"
out_file = "subliminal-nightfall.json"

[[targets]]
id = "cursor"
enabled = true
path = "cursor/themes"
out_names = { base = "subliminal-nightfall-color-theme.json", hazy = "subliminal-nightfall-color-theme-hazy.json", cloudy = "subliminal-nightfall-color-theme-cloudy.json" }

[[targets]]
id = "neovim"
enabled = true
path = "neovim/colors"
out_names = { base = "subliminal-nightfall.lua", hazy = "subliminal-nightfall-hazy.lua", cloudy = "subliminal-nightfall-cloudy.lua" }

[[targets]]
id = "ghostty"
enabled = true
path = "ghostty"
out_names = { base = "subliminal-nightfall", hazy = "subliminal-nightfall-hazy", cloudy = "subliminal-nightfall-cloudy" }

[[targets]]
id = "website"
enabled = true
path = "website/src/data"
out_file = "palette.json"
```

## Transparency & Blur

ColorLoom automatically handles transparency for blur-enabled variants:

| Variant | Alpha | Editor BG | Surface BG | Tabs | Appearance |
|---------|-------|-----------|------------|------|------------|
| Base | 100% | `#191724FF` | `#1f1d2eFF` | 50% | opaque |
| Hazy | 75% | `#19172400` | `#1f1d2eBF` | 22% | blurred |
| Cloudy | 90% | `#19172400` | `#1f1d2eE6` | 27% | blurred |

For blurred variants:
- Editor and gutter backgrounds are fully transparent (`00`) to show blur
- Surfaces and panels use variant alpha for layering
- Tabs and toolbars use reduced opacity for subtle differentiation

## Supported Targets

### Zed
Generates a single JSON file containing all variants with:
- `background.appearance: "blurred"` for transparent variants
- Complete syntax highlighting
- Panel, tab bar, status bar, terminal colors
- Git decoration colors
- Player colors for collaboration

### VS Code / Cursor
Generates separate JSON files per variant with:
- 100+ workbench colors
- Token colors for syntax
- Semantic highlighting support
- Git and diff colors

### Neovim
Generates Lua colorscheme files with:
- Highlight group definitions
- Terminal colors
- TreeSitter support ready

### Ghostty
Generates palette files with:
- 16-color ANSI palette
- Background/foreground
- Selection colors
- Cursor colors

## Development

```bash
# Build
cargo build --release

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Lint
cargo clippy
```

## Architecture

```
colorloom/
├── src/
│   ├── main.rs      # CLI entry point
│   ├── config.rs    # TOML configuration types
│   └── targets.rs   # Theme generators per target
├── Cargo.toml
└── README.md
```

## License

MIT License - see the main repository for details.

## Links

- **Documentation**: [subliminal-nightfall.hamrah.com/colorloom](https://subliminal-nightfall.hamrah.com/colorloom)
- **Main Repository**: [github.com/mhamrah/subliminal-nightfall](https://github.com/mhamrah/subliminal-nightfall)
- **Theme Website**: [subliminal-nightfall.hamrah.com](https://subliminal-nightfall.hamrah.com)
- **crates.io**: [crates.io/crates/colorloom](https://crates.io/crates/colorloom)
