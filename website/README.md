# Subliminal Nightfall Website

A showcase website for the Subliminal Nightfall color scheme, built with Astro and deployed on Cloudflare Pages.

🌐 **Live Site**: [hamrah.com/subliminal-nightfall](https://hamrah.com/subliminal-nightfall)

## Features

- 🎨 **Interactive Color Palette** - Click to copy any color hex code
- 💻 **Live Code Examples** - Syntax highlighting for Go, Rust, TypeScript, Swift, and Shell
- 📱 **Responsive Design** - Optimized for mobile, tablet, and desktop
- ⚡ **Fast Performance** - Static site generation with Astro
- 🔒 **Secure** - Security headers and best practices
- 🎯 **SEO Optimized** - Meta tags, Open Graph, and semantic HTML

## Tech Stack

- **Framework**: [Astro](https://astro.build/) 4.x
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) 4.x
- **Deployment**: [Cloudflare Pages](https://pages.cloudflare.com/)
- **Language**: TypeScript

## Project Structure

```
website/
├── src/
│   ├── pages/
│   │   └── index.astro          # Main landing page
│   ├── components/
│   │   ├── Header.astro         # Navigation header
│   │   ├── Footer.astro         # Footer with links
│   │   ├── ColorPalette.astro   # Interactive color showcase
│   │   ├── CodePreview.astro    # Tabbed code examples
│   │   └── Installation.astro   # Platform-specific install guides
│   ├── layouts/
│   │   └── Layout.astro         # Base HTML layout
│   ├── styles/
│   │   └── global.css           # Global styles with theme colors
│   └── code-samples/
│       ├── example.go           # Go sample code
│       ├── example.rs           # Rust sample code
│       ├── example.ts           # TypeScript sample code
│       ├── example.swift        # Swift sample code
│       └── example.sh           # Shell/terminal sample
├── public/
│   ├── favicon.svg              # Site favicon
│   └── _headers                 # Cloudflare Pages headers
├── astro.config.mjs             # Astro configuration
├── package.json                 # Dependencies and scripts
└── tsconfig.json                # TypeScript configuration
```

## Development

### Prerequisites

- Node.js 18+
- npm or yarn

### Installation

```bash
# Install dependencies
npm install
```

### Available Scripts

```bash
# Start development server
npm run dev
# Opens at http://localhost:4321/subliminal-nightfall

# Build for production
npm run build

# Preview production build
npm run preview

# Check for errors
npm run astro check
```

### Development Workflow

1. **Start dev server**: `npm run dev`
2. **Make changes**: Edit files in `src/`
3. **Preview**: Changes auto-reload in browser
4. **Build**: Run `npm run build` to test production build
5. **Preview build**: Run `npm run preview` to test locally

## Configuration

### Astro Config

The site is configured for deployment at `hamrah.com/subliminal-nightfall`:

```javascript
// astro.config.mjs
export default defineConfig({
  site: 'https://hamrah.com',
  base: '/subliminal-nightfall',
  output: 'static',
  // ...
});
```

### Theme Colors

All theme colors are defined in `src/styles/global.css` using the `@theme` directive:

```css
@theme {
  --color-sn-bg: #191724;
  --color-sn-fg: #e0def4;
  --color-sn-cyan: #5fb3b3;
  /* ... more colors */
}
```

## Deployment

### Cloudflare Pages (Recommended)

See [DEPLOYMENT.md](./DEPLOYMENT.md) for detailed instructions.

**Quick Deploy via CLI:**

```bash
# Build the project
npm run build

# Deploy with Wrangler
npx wrangler pages deploy dist --project-name=subliminal-nightfall
```

**Build Settings for Cloudflare Dashboard:**

- **Build command**: `cd website && npm run build`
- **Build output directory**: `website/dist`
- **Root directory**: `/`
- **Framework preset**: Astro

### Other Platforms

The site can be deployed to any static hosting provider:

- **Netlify**: Works out of the box
- **Vercel**: Automatic Astro detection
- **GitHub Pages**: Requires repository settings adjustment

## Components

### ColorPalette

Displays all theme colors with:
- Base, bright, and dim variants
- Click-to-copy functionality
- Usage descriptions
- Toast notifications

### CodePreview

Shows syntax-highlighted code samples:
- Tabbed interface for multiple languages
- Copy button for each example
- Window chrome for editor feel
- Custom syntax highlighting matching the theme

### Installation

Platform-specific installation instructions:
- Tabbed UI for VS Code, Neovim, Zed, Ghostty
- Copy-to-clipboard for commands
- Step-by-step guides
- External links to documentation

## Adding New Code Samples

1. Create a new file in `src/code-samples/`
2. Add entry to the `codeFiles` array in `CodePreview.astro`:

```javascript
{
  name: 'Python',
  lang: 'python',
  file: 'example.py',
  icon: '🐍'
}
```

3. Add syntax highlighting styles in the `<style>` section

## Customization

### Changing Colors

Edit `src/styles/global.css` to modify theme colors:

```css
@theme {
  --color-sn-cyan: #YOUR_COLOR;
}
```

### Adding Sections

1. Create a new component in `src/components/`
2. Import and add to `src/pages/index.astro`
3. Update navigation in `src/components/Header.astro`

## Performance

The site is optimized for performance:

- ✅ Static site generation (no JavaScript required for content)
- ✅ Minimal JavaScript (only for interactive features)
- ✅ Optimized CSS (Tailwind purging)
- ✅ Fast CDN delivery (Cloudflare)
- ✅ Proper caching headers
- ✅ HTTP/2 and Brotli compression

**Lighthouse Scores** (Target):
- Performance: 95+
- Accessibility: 100
- Best Practices: 100
- SEO: 100

## Browser Support

- Chrome/Edge: Latest 2 versions
- Firefox: Latest 2 versions
- Safari: Latest 2 versions
- Mobile Safari: iOS 14+
- Mobile Chrome: Latest 2 versions

## Troubleshooting

### Build Errors

**Problem**: Build fails with module not found
```bash
# Solution: Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
```

**Problem**: CSS not applying
```bash
# Solution: Check Tailwind configuration
# Ensure global.css is imported in Layout.astro
```

### Development Issues

**Problem**: Changes not reflecting
```bash
# Solution: Hard reload browser
# Mac: Cmd + Shift + R
# Windows: Ctrl + Shift + R
```

**Problem**: Base URL not working locally
```bash
# Solution: Use preview command after build
npm run build
npm run preview
# Visit: http://localhost:4321/subliminal-nightfall
```

## Contributing

When adding new features:

1. Maintain responsive design
2. Follow existing component patterns
3. Use theme colors from CSS variables
4. Test in multiple browsers
5. Ensure accessibility (ARIA labels, keyboard navigation)
6. Update this README if needed

## Resources

- [Astro Documentation](https://docs.astro.build/)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [Cloudflare Pages Documentation](https://developers.cloudflare.com/pages/)
- [Subliminal Nightfall Repository](https://github.com/mhamrah/subliminal-nightfall)

## License

Same as parent project - see [LICENSE](../LICENSE) file.

## Support

- Report issues: [GitHub Issues](https://github.com/mhamrah/subliminal-nightfall/issues)
- Main project: [Subliminal Nightfall](https://github.com/mhamrah/subliminal-nightfall)
