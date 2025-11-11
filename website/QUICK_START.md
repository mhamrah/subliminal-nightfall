# Quick Start Guide

## 🚀 Get Up and Running

### First Time Setup

```bash
cd website
npm install
npm run dev
```

Visit: `http://localhost:4321/subliminal-nightfall`

---

## 📋 Common Commands

### Development
```bash
npm run dev              # Start dev server with hot reload
npm run build            # Build for production
npm run preview          # Preview production build locally
npm run astro check      # Check for errors
```

### Deployment
```bash
# Via Wrangler CLI
npm run build
npx wrangler pages deploy dist --project-name=subliminal-nightfall

# Or just push to main branch (auto-deploys via Cloudflare)
git push origin main
```

---

## 🎨 Editing Content

### Change Colors
Edit `src/styles/global.css`:
```css
@theme {
  --color-sn-cyan: #YOUR_HEX;
}
```

### Add Code Sample
1. Create file in `src/code-samples/example.lang`
2. Add to `CodePreview.astro` in `codeFiles` array
3. Add syntax highlighting styles

### Modify Hero Text
Edit `src/pages/index.astro` - Hero Section

### Update Installation Instructions
Edit `src/components/Installation.astro`

---

## 🐛 Troubleshooting

### Build Fails
```bash
rm -rf node_modules package-lock.json
npm install
npm run build
```

### Changes Not Showing
- Hard refresh: Cmd+Shift+R (Mac) or Ctrl+Shift+R (Windows)
- Restart dev server
- Clear browser cache

### Preview Not Working
```bash
npm run build
npm run preview
# Visit with base path: /subliminal-nightfall
```

---

## 📁 File Structure

```
website/
├── src/
│   ├── pages/index.astro          # Main page
│   ├── components/                 # Reusable components
│   ├── layouts/Layout.astro        # HTML wrapper
│   ├── styles/global.css           # Theme colors
│   └── code-samples/               # Example code files
├── public/                         # Static assets
└── astro.config.mjs                # Astro config
```

---

## 🔗 Quick Links

- **Local Dev**: http://localhost:4321/subliminal-nightfall
- **Full Docs**: See README.md
- **Deployment**: See DEPLOYMENT_INSTRUCTIONS.md
- **GitHub**: https://github.com/mhamrah/subliminal-nightfall

---

## ✅ Pre-Deploy Checklist

- [ ] `npm run build` succeeds
- [ ] `npm run preview` works
- [ ] All interactive features tested
- [ ] Mobile responsive checked
- [ ] Git committed and pushed

---

## 🎯 One-Line Deploy

```bash
npm run build && npx wrangler pages deploy dist
```

That's it! 🚀
