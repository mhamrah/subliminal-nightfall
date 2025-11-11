# Subliminal Nightfall - Monorepo Architecture

This project uses **pnpm workspaces** for managing multiple packages in a single repository.

## Why a Monorepo?

The color scheme definition and its implementations are tightly coupled:
- The website needs to reference theme files
- All packages should version together
- Single source of truth for colors
- Atomic commits across theme + website + extensions

## Structure

```
subliminal-nightfall/
├── packages/
│   ├── core/              # Color definitions (single source of truth)
│   └── website/           # Showcase site (Astro)
├── cursor/                # VSCode/Cursor extension
├── neovim/                # Neovim Lua colorscheme
├── ghostty/               # Ghostty terminal theme
├── zed/
│   └── themes/            # Zed theme JSON (source of truth)
├── themes/                # Symlink to zed/themes/ (for Zed extension)
├── extension.toml         # Zed extension manifest
├── pnpm-workspace.yaml    # Workspace configuration
├── package.json           # Root package with scripts
└── README.md              # Main documentation
```

**Note**: The `themes/` directory is a symlink to `zed/themes/` because:
- Zed extensions require `themes/` at the repository root (same level as `extension.toml`)
- We want to keep editor-specific files organized in their own directories
- The symlink maintains both requirements without duplication

## Package Overview

### @subliminal/core

**Purpose**: Single source of truth for all color values

**Exports**:
- Color definitions (TypeScript)
- Theme generation utilities
- Type definitions

**Used by**: All other packages

**Technology**: TypeScript

```typescript
// Other packages import from core
import { palette, theme } from '@subliminal/core';
```

### @subliminal/vscode

**Purpose**: VSCode and Cursor extension

**Dependencies**: `@subliminal/core`

**Generates**: VSCode theme JSON from core colors

**Technology**: TypeScript, VSCode Extension API

### @subliminal/neovim

**Purpose**: Neovim color scheme

**Dependencies**: `@subliminal/core` (optional, for generation)

**Files**: Lua colorscheme with Treesitter support

**Technology**: Lua

### @subliminal/zed

**Purpose**: Zed editor theme

**Dependencies**: `@subliminal/core`

**Generates**: Zed theme JSON from core colors

**Technology**: JSON

### @subliminal/ghostty

**Purpose**: Ghostty terminal theme

**Dependencies**: `@subliminal/core`

**Generates**: Ghostty config from core colors

**Technology**: Configuration file

### @subliminal/website

**Purpose**: Showcase website with live demos

**Dependencies**: `@subliminal/core`

**Features**:
- Interactive color palette
- Live code examples
- Installation guides
- Theme comparisons

**Technology**: Astro, Tailwind CSS, TypeScript

## Package Manager: pnpm

We use **pnpm** instead of npm/yarn for:

✅ **Faster installs** - Efficient disk usage via hard links
✅ **Stricter** - Catches phantom dependencies
✅ **Better workspace protocol** - `workspace:*` for local packages
✅ **Disk space** - Shared dependency cache

## Workspace Configuration

```yaml
# pnpm-workspace.yaml
packages:
  - "packages/*"
```

This tells pnpm to treat all directories in `packages/` as separate packages.

## Common Commands

### Root Level (all packages)

```bash
# Install all dependencies
pnpm install

# Build all packages
pnpm build

# Run website dev server
pnpm dev

# Clean all build outputs
pnpm clean

# Format all code
pnpm format

# Run tests (if available)
pnpm test
```

### Specific Package

```bash
# Build only core
pnpm build:core

# Build only website
pnpm build:website

# Run website preview
pnpm preview

# Work in specific package
pnpm --filter @subliminal/core build
pnpm --filter @subliminal/website dev
```

### Recursive Operations

```bash
# Build all packages in dependency order
pnpm -r build

# Clean all packages
pnpm -r clean

# Run tests in all packages
pnpm -r test
```

## Dependency Management

### Installing Dependencies

```bash
# Add dependency to specific package
pnpm --filter @subliminal/website add astro

# Add dev dependency to root
pnpm add -D -w prettier

# Add workspace dependency
# (automatically uses workspace:* protocol)
pnpm --filter @subliminal/website add @subliminal/core
```

### Workspace Protocol

Packages reference each other using `workspace:*`:

```json
{
  "dependencies": {
    "@subliminal/core": "workspace:*"
  }
}
```

This ensures:
- Always uses local version during development
- Replaced with actual version on publish
- Type safety across packages

## Build Order

Dependencies are built in this order:

1. **@subliminal/core** (no dependencies)
2. **@subliminal/vscode** (depends on core)
3. **@subliminal/zed** (depends on core)
4. **@subliminal/neovim** (independent)
5. **@subliminal/ghostty** (independent)
6. **@subliminal/website** (depends on core)

pnpm handles this automatically when running `pnpm -r build`.

## Development Workflow

### 1. Working on Core Colors

```bash
# Start core in watch mode
cd packages/core
pnpm dev

# In another terminal, run website
cd packages/website
pnpm dev
```

Changes to core automatically rebuild and reflect in website.

### 2. Working on Website

```bash
# From root
pnpm dev

# Or
cd packages/website
pnpm dev
```

### 3. Working on Extensions

```bash
# VSCode extension
cd packages/vscode
pnpm build

# Neovim (manual file editing)
cd packages/neovim
# Edit Lua files directly
```

## Adding a New Package

1. Create directory in `packages/`:
```bash
mkdir packages/new-package
```

2. Create `package.json`:
```json
{
  "name": "@subliminal/new-package",
  "version": "1.0.0",
  "dependencies": {
    "@subliminal/core": "workspace:*"
  }
}
```

3. Install dependencies:
```bash
pnpm install
```

4. Import from core:
```typescript
import { palette } from '@subliminal/core';
```

## Why Not Turborepo?

**TL;DR**: Not needed for this project size.

Turborepo is recommended when:
- Many packages (10+)
- Slow builds needing intelligent caching
- Complex dependency graphs
- Remote caching for CI/CD

Our project:
- Small number of packages (6)
- Fast builds (~400ms for website)
- Simple dependencies (mostly core → packages)
- pnpm workspaces handle our needs

**We can add Turborepo later if needed.**

## Publishing

### Individual Packages

```bash
# Build and publish VSCode extension
cd packages/vscode
pnpm build
vsce package
vsce publish

# Publish to npm (if needed)
pnpm publish
```

### Website Deployment

```bash
# Build website
pnpm build:website

# Deploy to Cloudflare Pages
cd packages/website
npx wrangler pages deploy dist
```

## Advantages of This Setup

✅ **Single Source of Truth** - All colors in `@subliminal/core`
✅ **Type Safety** - TypeScript across packages
✅ **Atomic Changes** - Update colors once, all packages get them
✅ **Easy Development** - `pnpm dev` runs everything
✅ **Version Sync** - All packages version together
✅ **Simple** - No Turborepo complexity until needed

## Troubleshooting

### Build Fails

```bash
# Clean everything and rebuild
pnpm clean
rm -rf node_modules
pnpm install
pnpm build
```

### Workspace Dependency Not Found

```bash
# Reinstall to link workspaces
pnpm install
```

### pnpm Not Found

```bash
# Install pnpm globally
npm install -g pnpm

# Or use corepack (Node 16.9+)
corepack enable
corepack prepare pnpm@latest --activate
```

### Website Not Seeing Core Changes

```bash
# Rebuild core
pnpm build:core

# Restart website dev server
pnpm dev
```

## References

- [pnpm Workspaces](https://pnpm.io/workspaces)
- [pnpm CLI](https://pnpm.io/cli/install)
- [Monorepo Best Practices](https://monorepo.tools/)
- [Astro + pnpm](https://docs.astro.build/en/guides/integrations-guide/)

## Contributing

When contributing:

1. Make changes in appropriate package
2. Update `@subliminal/core` if changing colors
3. Run `pnpm build` to ensure everything builds
4. Test website with `pnpm dev`
5. Commit all changed packages together

## License

MIT - See individual package LICENSE files
