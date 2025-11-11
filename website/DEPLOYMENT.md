# Cloudflare Pages Deployment Guide

This guide will help you deploy the Subliminal Nightfall website to Cloudflare Pages under `hamrah.com/subliminal-nightfall`.

## Prerequisites

- A Cloudflare account
- Domain `hamrah.com` configured in Cloudflare
- Git repository pushed to GitHub/GitLab

## Deployment Steps

### Option 1: Deploy via Cloudflare Dashboard (Recommended)

1. **Log in to Cloudflare Dashboard**
   - Go to https://dash.cloudflare.com/
   - Select your account

2. **Create a New Pages Project**
   - Navigate to "Workers & Pages" in the sidebar
   - Click "Create application"
   - Select "Pages" tab
   - Click "Connect to Git"

3. **Connect Your Repository**
   - Authorize Cloudflare to access your GitHub/GitLab account
   - Select the `subliminal-nightfall` repository
   - Click "Begin setup"

4. **Configure Build Settings**
   ```
   Project name: subliminal-nightfall
   Production branch: main
   Framework preset: Astro
   Build command: cd website && npm run build
   Build output directory: website/dist
   Root directory: /
   ```

5. **Environment Variables**
   - No special environment variables needed for this project

6. **Deploy**
   - Click "Save and Deploy"
   - Wait for the build to complete (usually 1-2 minutes)

7. **Configure Custom Domain**
   - After deployment, go to your project settings
   - Click "Custom domains"
   - Add custom domain: `hamrah.com`
   - Set up a path: `/subliminal-nightfall`

   **Note**: For subdirectory hosting, you may need to use Cloudflare Workers or set up redirects.

### Option 2: Deploy via Wrangler CLI

1. **Install Wrangler**
   ```bash
   npm install -g wrangler
   ```

2. **Login to Cloudflare**
   ```bash
   wrangler login
   ```

3. **Build the Project**
   ```bash
   cd website
   npm run build
   ```

4. **Deploy to Pages**
   ```bash
   wrangler pages deploy dist --project-name=subliminal-nightfall
   ```

5. **Set Up Custom Domain**
   ```bash
   wrangler pages domain add hamrah.com/subliminal-nightfall
   ```

## Subdirectory Configuration

Since the site needs to be hosted at `hamrah.com/subliminal-nightfall`, the Astro config is already set up:

```javascript
// astro.config.mjs
export default defineConfig({
  site: 'https://hamrah.com',
  base: '/subliminal-nightfall',
  // ... other config
});
```

This ensures all assets and links are correctly prefixed with `/subliminal-nightfall`.

## Custom Domain Setup

### If using Cloudflare Pages at root domain

If you want to use a custom Cloudflare Pages domain like `subliminal-nightfall.pages.dev`:

1. Go to your Pages project in Cloudflare Dashboard
2. Navigate to "Custom domains"
3. Click "Set up a custom domain"
4. Enter your desired subdomain or use the provided `.pages.dev` domain
5. Cloudflare will automatically configure DNS and SSL

### If integrating with existing hamrah.com site

To serve the site at `hamrah.com/subliminal-nightfall`:

**Option A: Using Cloudflare Workers (Recommended)**

Create a Worker to route the subdirectory:

```javascript
// worker.js
export default {
  async fetch(request, env) {
    const url = new URL(request.url);

    // Route /subliminal-nightfall to Pages project
    if (url.pathname.startsWith('/subliminal-nightfall')) {
      return env.ASSETS.fetch(request);
    }

    // Handle other routes for main site
    return fetch(request);
  }
}
```

**Option B: Using _redirects file**

If your main site is also on Cloudflare Pages, use a `_redirects` file:

```
/subliminal-nightfall/* https://subliminal-nightfall.pages.dev/:splat 200
```

## Automatic Deployments

Cloudflare Pages automatically deploys when you push to your connected Git repository:

- **Production**: Pushes to `main` branch
- **Preview**: Pull requests and other branches

## Build Configuration

The project uses these npm scripts:

```json
{
  "scripts": {
    "dev": "astro dev",
    "build": "astro build",
    "preview": "astro preview"
  }
}
```

## Troubleshooting

### Build Fails

1. Check Node.js version (should be 18+)
2. Clear cache and rebuild:
   ```bash
   rm -rf node_modules package-lock.json
   npm install
   npm run build
   ```

### Assets not loading

1. Verify `base` is set correctly in `astro.config.mjs`
2. Check that all asset paths use the Astro `import.meta.env.BASE_URL`
3. Review the `_headers` file for correct cache settings

### 404 Errors

1. Ensure the `dist` directory is being deployed
2. Check that the output directory in Cloudflare settings matches `website/dist`
3. Verify that `output: 'static'` is set in astro.config.mjs

## Performance Optimization

Cloudflare Pages automatically provides:

- ✅ Global CDN distribution
- ✅ Automatic HTTPS/SSL
- ✅ Automatic minification
- ✅ HTTP/2 and HTTP/3 support
- ✅ Brotli compression

Additional optimizations already configured:

- Cache headers in `public/_headers`
- Inlined critical CSS
- Optimized asset loading

## Monitoring

Monitor your deployment:

1. **Analytics**: Enable Cloudflare Web Analytics
   - Go to "Analytics" in your Pages project
   - Enable free Web Analytics

2. **Build Logs**: View build logs in the Cloudflare Dashboard
   - Each deployment shows detailed logs
   - Helpful for debugging build issues

## Local Preview

Test the production build locally before deploying:

```bash
cd website
npm run build
npm run preview
```

Then visit `http://localhost:4321/subliminal-nightfall` to preview.

## Rollback

If a deployment has issues:

1. Go to your Pages project in Cloudflare
2. Navigate to "Deployments"
3. Find a previous working deployment
4. Click "..." menu and select "Rollback to this deployment"

## Support

- Cloudflare Pages Docs: https://developers.cloudflare.com/pages/
- Astro Deployment Guide: https://docs.astro.build/en/guides/deploy/cloudflare/
- Project Issues: https://github.com/mhamrah/subliminal-nightfall/issues
