# 🚀 Deployment Instructions for Subliminal Nightfall Website

## Quick Start

The Subliminal Nightfall showcase website is **ready for deployment** to Cloudflare Pages at `hamrah.com/subliminal-nightfall`.

## ✅ Pre-Deployment Checklist

- [x] Astro project built successfully
- [x] All components working (Header, Footer, ColorPalette, CodePreview, Installation)
- [x] Code samples created for Go, Rust, TypeScript, Swift, and Shell
- [x] Tailwind CSS configured with theme colors
- [x] Responsive design tested (mobile, tablet, desktop)
- [x] Interactive features implemented (copy-to-clipboard, tabs, mobile menu)
- [x] SEO meta tags added
- [x] Security headers configured
- [x] Base path set to `/subliminal-nightfall`
- [x] Build output: `website/dist/` (160KB total)

## 📋 Deployment Options

### Option 1: Cloudflare Pages Dashboard (Recommended for First Deploy)

#### Step 1: Push to GitHub (if not already done)

```bash
cd subliminal-nightfall
git add .
git commit -m "Add showcase website for Subliminal Nightfall theme"
git push origin main
```

#### Step 2: Create Cloudflare Pages Project

1. Go to https://dash.cloudflare.com/
2. Select your account
3. Click **Workers & Pages** in the sidebar
4. Click **Create application**
5. Select the **Pages** tab
6. Click **Connect to Git**

#### Step 3: Connect Repository

1. Authorize Cloudflare to access your GitHub account
2. Select the `subliminal-nightfall` repository
3. Click **Begin setup**

#### Step 4: Configure Build Settings

```
Project name:           subliminal-nightfall
Production branch:      main
Framework preset:       Astro
Build command:          cd website && npm run build
Build output directory: website/dist
Root directory:         / (leave as root)
Environment variables:  (none needed)
```

#### Step 5: Deploy

1. Click **Save and Deploy**
2. Wait for build to complete (~1-2 minutes)
3. You'll get a URL like: `https://subliminal-nightfall.pages.dev`

#### Step 6: Set Up Custom Domain Path

**For subdirectory hosting at `hamrah.com/subliminal-nightfall`:**

This requires integrating with your main hamrah.com site. Choose one approach:

**Approach A: Cloudflare Worker (Recommended)**

Create a Worker to route the subdirectory:

```javascript
// worker.js
export default {
  async fetch(request, env) {
    const url = new URL(request.url);

    // Route /subliminal-nightfall to Pages project
    if (url.pathname.startsWith('/subliminal-nightfall')) {
      // Remove /subliminal-nightfall prefix for the Pages fetch
      const newPath = url.pathname.replace('/subliminal-nightfall', '') || '/';
      const newUrl = new URL(newPath, 'https://subliminal-nightfall.pages.dev');
      newUrl.search = url.search;

      const response = await fetch(newUrl);
      return response;
    }

    // Handle other routes for main hamrah.com site
    return fetch(request);
  }
}
```

**Approach B: Redirect Rules**

If your main site is also on Cloudflare Pages:
1. Add to main site's `_redirects` file:
```
/subliminal-nightfall/* https://subliminal-nightfall.pages.dev/:splat 200
```

**Approach C: Separate Subdomain**

Alternatively, use a subdomain:
1. In Cloudflare Pages project settings, go to **Custom domains**
2. Add: `subliminal-nightfall.hamrah.com` or `sn.hamrah.com`
3. Cloudflare automatically configures DNS and SSL

---

### Option 2: Wrangler CLI (Recommended for Updates)

#### Step 1: Install Wrangler

```bash
npm install -g wrangler
```

#### Step 2: Login to Cloudflare

```bash
wrangler login
```

This opens a browser for authentication.

#### Step 3: Build the Project

```bash
cd website
npm run build
```

#### Step 4: Deploy

```bash
npx wrangler pages deploy dist --project-name=subliminal-nightfall
```

Follow prompts:
- Create new project? Yes
- Production branch? main

#### Step 5: Subsequent Deploys

After first deploy, subsequent deployments are simple:

```bash
npm run build
npx wrangler pages deploy dist
```

---

## 🔧 Verifying Your Deployment

### Test the Deployed Site

1. **Navigate to your deployment URL**
   - e.g., `https://subliminal-nightfall.pages.dev`
   - or `https://hamrah.com/subliminal-nightfall` (after domain setup)

2. **Check all features work:**
   - [ ] Navigation menu (desktop and mobile)
   - [ ] Color palette displays correctly
   - [ ] Click any color to copy (toast notification appears)
   - [ ] Code examples tab switching works
   - [ ] Copy code button works
   - [ ] Installation tabs switch correctly
   - [ ] All external links open in new tabs
   - [ ] Footer links work

3. **Test responsive design:**
   - [ ] Mobile view (hamburger menu works)
   - [ ] Tablet view
   - [ ] Desktop view

4. **Verify SEO:**
   - View page source and check:
   - [ ] `<title>` tag is correct
   - [ ] Meta description present
   - [ ] Open Graph tags present
   - [ ] Canonical URL correct

---

## 🔄 Making Updates

### Update Workflow

1. **Make changes locally**
   ```bash
   cd website
   npm run dev
   # Make your changes
   ```

2. **Test locally**
   ```bash
   npm run build
   npm run preview
   # Visit http://localhost:4321/subliminal-nightfall
   ```

3. **Deploy update**

   **Via Git (automatic):**
   ```bash
   git add .
   git commit -m "Update website"
   git push origin main
   # Cloudflare automatically builds and deploys
   ```

   **Via Wrangler (manual):**
   ```bash
   npm run build
   npx wrangler pages deploy dist
   ```

---

## 📊 Monitoring

### Cloudflare Pages Dashboard

Monitor your deployment:

1. **Build logs**: Check for build errors
2. **Deployments**: See all deployments and their status
3. **Analytics**: Enable Cloudflare Web Analytics for free
4. **Custom domains**: Manage domain routing

### Enable Analytics

1. Go to your Pages project
2. Click **Analytics** tab
3. Enable **Web Analytics** (free, privacy-friendly)
4. Add analytics script to site (optional, tracking is automatic)

---

## 🐛 Troubleshooting

### Build Fails

**Error: Module not found**
```bash
cd website
rm -rf node_modules package-lock.json
npm install
npm run build
```

**Error: Astro build fails**
- Check Node.js version: `node --version` (should be 18+)
- Check build logs in Cloudflare dashboard
- Verify all imports are correct

### Assets Not Loading

**Problem: CSS/JS not loading**
- Verify `base: '/subliminal-nightfall'` is set in `astro.config.mjs`
- Check browser console for 404 errors
- Ensure `_headers` file is in `public/` directory

**Problem: Favicon not showing**
- Clear browser cache
- Verify `favicon.svg` is in `public/` directory
- Check path in Layout.astro

### Routing Issues

**Problem: 404 on deployment**
- Verify build output directory is `website/dist`
- Check that `dist/index.html` exists
- Ensure `output: 'static'` in astro.config.mjs

---

## 🔒 Security

Security headers are configured in `public/_headers`:

- X-Frame-Options: DENY
- X-Content-Type-Options: nosniff
- X-XSS-Protection: 1; mode=block
- Referrer-Policy: strict-origin-when-cross-origin
- Cache-Control: Optimized for static assets

These are automatically applied by Cloudflare Pages.

---

## 📝 Post-Deployment Tasks

### 1. Update Main README

Update the main project README to include the live website link:

```markdown
## Website

Visit the showcase website at [hamrah.com/subliminal-nightfall](https://hamrah.com/subliminal-nightfall)
```

### 2. Share the Site

- Update VS Code marketplace listing with website link
- Add to GitHub repository description
- Share on social media / developer communities

### 3. Monitor Performance

- Check Lighthouse scores
- Monitor Core Web Vitals
- Review analytics for usage patterns

---

## 🎯 Expected Results

After successful deployment, you should have:

- ✅ Live website at `hamrah.com/subliminal-nightfall`
- ✅ Automatic deployments on git push
- ✅ Global CDN distribution
- ✅ Automatic HTTPS
- ✅ Fast load times (<1s)
- ✅ Mobile-responsive design
- ✅ Interactive color palette
- ✅ Live code examples
- ✅ Platform-specific installation guides

---

## 📞 Support

- **Cloudflare Pages Docs**: https://developers.cloudflare.com/pages/
- **Astro Deployment Guide**: https://docs.astro.build/en/guides/deploy/cloudflare/
- **Project Issues**: https://github.com/mhamrah/subliminal-nightfall/issues
- **Website Documentation**: See `website/README.md`
- **Deployment Details**: See `website/DEPLOYMENT.md`

---

## ✨ Success!

Once deployed, your Subliminal Nightfall showcase website will be live and accessible to users worldwide, beautifully presenting your color scheme with interactive examples and installation guides.

**Build Status**: ✅ Ready for Production
**Deployment Target**: Cloudflare Pages
**Live URL**: `hamrah.com/subliminal-nightfall`
**Build Size**: ~160KB
**Build Time**: ~400ms

Happy deploying! 🚀
