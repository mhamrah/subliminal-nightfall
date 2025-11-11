# GitHub Actions Deployment Setup

This guide explains how to set up automated deployment to Cloudflare Pages using GitHub Actions.

## Overview

The workflow automatically deploys the website to Cloudflare Pages when:
- Changes are pushed to `main` branch
- Changes affect `website/**`, `packages/core/**`, or workspace configuration
- Manually triggered via GitHub Actions UI

## Prerequisites

1. A Cloudflare account
2. A Cloudflare Pages project named `subliminal-nightfall`
3. GitHub repository with admin access

## Setup Steps

### 1. Get Cloudflare Credentials

#### Account ID

1. Log in to [Cloudflare Dashboard](https://dash.cloudflare.com)
2. Go to any page in your account
3. Look in the URL or sidebar for your Account ID
4. Copy the Account ID (format: `a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6`)

#### API Token

1. Go to [Cloudflare API Tokens](https://dash.cloudflare.com/profile/api-tokens)
2. Click **Create Token**
3. Use the **"Edit Cloudflare Workers"** template, or create a custom token with:
   - **Permissions**:
     - Account → Cloudflare Pages → Edit
   - **Account Resources**:
     - Include → Your Account
4. Click **Continue to summary**
5. Click **Create Token**
6. **Copy the token immediately** (you won't be able to see it again)

### 2. Add Secrets to GitHub

1. Go to your GitHub repository
2. Click **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Add the following secrets:

#### CLOUDFLARE_ACCOUNT_ID
- **Name**: `CLOUDFLARE_ACCOUNT_ID`
- **Value**: Your Cloudflare Account ID (from step 1)

#### CLOUDFLARE_API_TOKEN
- **Name**: `CLOUDFLARE_API_TOKEN`
- **Value**: Your Cloudflare API Token (from step 1)

### 3. Verify Cloudflare Pages Project

Ensure your Cloudflare Pages project is set up:

1. Go to [Cloudflare Pages](https://dash.cloudflare.com/pages)
2. Verify project name is **exactly** `subliminal-nightfall`
3. If not, either:
   - Rename the project in Cloudflare, or
   - Update `projectName` in `.github/workflows/deploy-website.yml`

### 4. Test the Workflow

#### Automatic Trigger

Make a change to the website and push:

```bash
# Make a small change
echo "# Test" >> website/README.md

# Commit and push
git add website/README.md
git commit -m "test: trigger deployment"
git push origin main
```

#### Manual Trigger

1. Go to **Actions** tab in GitHub
2. Select **Deploy Website to Cloudflare Pages**
3. Click **Run workflow**
4. Select branch `main`
5. Click **Run workflow**

### 5. Monitor Deployment

1. Go to **Actions** tab in your repository
2. Click on the running workflow
3. Watch the build and deployment progress
4. Check for any errors in the logs

The deployment should:
- ✅ Checkout code
- ✅ Setup Node.js 22
- ✅ Setup pnpm 10.11.1
- ✅ Install dependencies
- ✅ Build website (`pnpm run build:cloudflare`)
- ✅ Deploy to Cloudflare Pages

## Workflow Configuration

The workflow is defined in `.github/workflows/deploy-website.yml`.

### Trigger Paths

The workflow runs when these paths change:
- `website/**` - Website source code
- `packages/core/**` - Core color definitions
- `pnpm-workspace.yaml` - Workspace configuration
- `package.json` - Root package configuration
- `.github/workflows/deploy-website.yml` - Workflow itself

### Build Command

The workflow runs: `pnpm run build:cloudflare`

This:
1. Uses pnpm workspace filtering
2. Builds only the `website` package
3. Outputs to `website/dist/`

### Deployment

Uses the official `cloudflare/pages-action@v1` to:
- Upload `website/dist/` to Cloudflare Pages
- Create a production deployment
- Update deployment status in GitHub

## Troubleshooting

### Build Fails: "No projects matched the filters"

**Cause**: `website` not in pnpm workspace

**Fix**: Ensure `pnpm-workspace.yaml` includes:
```yaml
packages:
  - "packages/*"
  - "website"
```

### Deployment Fails: "Unauthorized"

**Cause**: Invalid API token or account ID

**Fix**:
1. Verify secrets in GitHub Settings → Secrets
2. Regenerate Cloudflare API token if needed
3. Ensure token has Cloudflare Pages Edit permission

### Deployment Fails: "Project not found"

**Cause**: Project name mismatch

**Fix**:
1. Check Cloudflare Pages project name
2. Update `projectName` in workflow file if different from `subliminal-nightfall`

### Build Succeeds but Site Not Updated

**Cause**: DNS/CDN cache

**Fix**:
1. Wait 5-10 minutes for CDN propagation
2. Hard refresh browser (Cmd+Shift+R or Ctrl+Shift+R)
3. Check Cloudflare Pages dashboard for deployment status

## Benefits of GitHub Actions Deployment

✅ **Automatic deployments** - Push to main and deploy automatically
✅ **Consistent builds** - Same environment every time
✅ **Build caching** - Faster subsequent builds with pnpm cache
✅ **Deployment history** - Track all deployments in GitHub Actions
✅ **PR previews** - Can be extended to deploy preview environments
✅ **Status checks** - See deployment status in pull requests

## Advanced Configuration

### Deploy Preview Environments

To deploy preview environments for pull requests, add this to the workflow:

```yaml
on:
  pull_request:
    branches:
      - main
```

And update the `cloudflare/pages-action` step to use:
```yaml
branch: ${{ github.head_ref || github.ref_name }}
```

### Custom Domain

The custom domain `subliminal-nightfall.hamrah.com` is configured in Cloudflare Pages dashboard and will be used automatically.

### Build Notifications

Add notifications on deployment success/failure:
- Slack notifications
- Discord webhooks
- Email notifications
- GitHub deployment environment

## Verification Checklist

- [ ] Cloudflare Account ID added to GitHub secrets
- [ ] Cloudflare API Token added to GitHub secrets
- [ ] API Token has Cloudflare Pages Edit permission
- [ ] Cloudflare Pages project exists and is named `subliminal-nightfall`
- [ ] Custom domain configured: `subliminal-nightfall.hamrah.com`
- [ ] Workflow file exists: `.github/workflows/deploy-website.yml`
- [ ] Test deployment succeeds
- [ ] Website accessible at https://subliminal-nightfall.hamrah.com

## Support

- **GitHub Actions docs**: https://docs.github.com/en/actions
- **Cloudflare Pages docs**: https://developers.cloudflare.com/pages/
- **Cloudflare Pages Action**: https://github.com/cloudflare/pages-action
- **Issues**: https://github.com/mhamrah/subliminal-nightfall/issues
