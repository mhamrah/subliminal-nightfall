# Subliminal Nightfall Website

Showcase website for the Subliminal Nightfall color scheme.

🌐 **Live**: [subliminal-nightfall.hamrah.com](https://subliminal-nightfall.hamrah.com)

## Tech Stack

- **Astro** 5.x - Static site generator
- **Tailwind CSS** 4.x - Styling
- **TypeScript** - Type safety
- **Cloudflare Pages** - Deployment

## Development

```bash
# Install dependencies (from repo root)
pnpm install

# Start dev server
pnpm --filter website dev
# Visit http://localhost:4321

# Build for production
pnpm --filter website build

# Preview production build
pnpm --filter website preview
```

## Deployment

**Cloudflare Pages Settings:**
- Build command: `pnpm run build:cloudflare`
- Output directory: `website/dist`
- Environment variables: `NODE_VERSION=22`, `PNPM_VERSION=10.11.1`
- Custom domain: `subliminal-nightfall.hamrah.com`

**Deploy via CLI:**
```bash
pnpm run build:cloudflare
cd website
npx wrangler pages deploy dist
```

## Structure

```
website/
├── src/
│   ├── pages/index.astro       # Main page
│   ├── components/             # Reusable components
│   ├── layouts/Layout.astro    # Base layout
│   └── code-samples/           # Example code files
├── public/
│   ├── favicon.svg
│   └── _headers                # Cloudflare headers
└── astro.config.mjs            # Astro config
```

## Configuration

The site is configured for the `subliminal-nightfall.hamrah.com` domain in `astro.config.mjs`.

All theme colors are defined using Tailwind's `@theme` directive in `src/styles/global.css`.
