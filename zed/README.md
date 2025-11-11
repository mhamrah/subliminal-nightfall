# Subliminal Nightfall - Zed Extension

This directory contains the Zed editor theme for Subliminal Nightfall.

## Structure

- `themes/` - Contains the Zed theme JSON file(s)

## Symlink Setup

The repository root has a `themes/` symlink that points to `zed/themes/`. This is required because:

1. **Zed Extension Convention**: Zed extensions must have themes in a `themes/` directory at the repository root (same level as `extension.toml`)
2. **Repository Organization**: We want to keep editor-specific files organized in their own directories

The symlink allows us to:
- Keep the source of truth in `zed/themes/`
- Maintain a clean, organized monorepo structure
- Meet Zed's extension requirements

## Development

The theme file `zed/themes/subliminal-nightfall.json` follows the [Zed Theme Schema v0.2.0](https://zed.dev/schema/themes/v0.2.0.json).

### Colors Source of Truth

All colors should be derived from `@subliminal/core` package (`packages/core/src/colors.ts`):

- **teal** (`#9ccfd8`) - Functions, methods, strings
- **blueGreen** (`#31748f`) - Keywords, types, constructors
- **lavender** (`#c4a7e7`) - Numbers, constants, enums, type parameters
- **gray** (`#7f7f7f`) - Comments

### Testing the Theme

1. Make sure you're in the repository root
2. Install the extension locally in Zed:
   - Open Zed
   - Run command: `zed: install dev extension`
   - Select this repository root directory
3. Select the theme:
   - Open command palette
   - Run: `theme selector: toggle`
   - Choose "Subliminal Nightfall"

## Publishing

The Zed extension is published from the repository root where `extension.toml` resides. The `themes/` symlink ensures Zed can find the theme files during both local development and when installed from the extension registry.

```zsh
# Publish to Zed extension registry (from repo root)
zed --dev-server-token <token> extensions publish
```

## See Also

- [Zed Theme Documentation](https://zed.dev/docs/extensions/themes)
- [Extension Manifest](../extension.toml)
- [Core Color Definitions](../packages/core/src/colors.ts)
