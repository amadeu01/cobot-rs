# Banner Image Troubleshooting Guide

This guide helps resolve issues with the banner image not displaying on GitHub.

## Common Banner Issues and Solutions

### Issue 1: Banner Not Showing on GitHub

**Symptoms:**
- Banner appears as broken image link on GitHub
- Shows alt text instead of image: "Cobot-RS banner"
- Works locally but not on GitHub website

**Root Causes:**
- Relative paths don't always work in different GitHub contexts
- File not committed to repository
- Case sensitivity issues
- Large file size causing loading issues

## Solution Options

### ✅ **Solution 1: Use Raw GitHub URLs (Recommended)**

Replace relative paths with absolute GitHub raw URLs:

```markdown
![Cobot-RS banner](https://raw.githubusercontent.com/USERNAME/cobot-rs/main/assets/banner.png)
```

**Advantages:**
- Works everywhere (GitHub, GitHub Pages, forks, etc.)
- Reliable across all GitHub contexts
- No dependency on repository structure

**How to implement:**
1. Replace `USERNAME` with your actual GitHub username
2. Ensure the file exists in your repository
3. Commit and push changes

### ✅ **Solution 2: Relative Path (Simple)**

Use the relative path format:

```markdown
![Cobot-RS banner](assets/banner.png)
```

**Advantages:**
- Simple and clean
- Works when repository is cloned

**Requirements:**
- File must be committed to git
- Repository structure must match exactly
- May not work in all GitHub contexts

### ✅ **Solution 3: GitHub Issues as CDN**

Upload image to a GitHub issue and use that URL:

1. Create a new issue in your repository
2. Drag and drop your banner image into the issue description
3. GitHub generates a permanent URL like: `https://github.com/user/repo/assets/12345/image.png`
4. Copy that URL and use it in your README

**Advantages:**
- Guaranteed to work everywhere
- GitHub handles hosting and CDN
- No repository bloat

### ✅ **Solution 4: External Image Hosting**

Host the banner on external services:

```markdown
![Cobot-RS banner](https://imgur.com/your-image-id.png)
![Cobot-RS banner](https://i.postimg.cc/your-image-id/banner.png)
```

**Services:**
- Imgur (free, reliable)
- PostImage (free, simple)
- GitHub Releases (for project assets)

## Verification Steps

### Step 1: Check File Exists
```bash
# Verify file exists and is tracked by git
git ls-files assets/banner.png
ls -la assets/banner.png
```

### Step 2: Test Image Validity
```bash
# Check if it's a valid image file
file assets/banner.png
```

### Step 3: Check Repository Status
```bash
# Ensure file is committed
git status assets/banner.png
git log --oneline -- assets/banner.png
```

### Step 4: Test Different URLs

Try each approach and verify in:
- GitHub repository view
- GitHub README preview
- Local markdown viewers
- Forked repositories

## Banner Best Practices

### Image Specifications
- **Format**: PNG or JPG (PNG preferred for logos)
- **Size**: Maximum 1MB for fast loading
- **Dimensions**: 1200px wide × 400px tall (3:1 ratio works well)
- **Content**: Project logo, name, and brief description

### File Organization
```
assets/
├── banner.png          # Main banner (optimized)
├── banner-large.png    # High-res version
├── logo.png           # Logo only
└── README.md          # Asset documentation
```

### Repository Setup
```bash
# Ensure proper git tracking
git add assets/banner.png
git commit -m "Add project banner"
git push origin main
```

## Template Usage

### For New Projects
Use this template format in your README:

```markdown
<!-- Replace YOUR_USERNAME with your GitHub username -->
![Project Name](https://raw.githubusercontent.com/YOUR_USERNAME/repo-name/main/assets/banner.png)

# Project Name

[![CI Status](https://github.com/YOUR_USERNAME/repo-name/workflows/CI/badge.svg)]
[![License](https://img.shields.io/badge/License-MIT-blue.svg)]
```

### For Contributors
When someone forks your repository:

1. **Banner will work automatically** if using absolute URLs
2. **Update URLs** if using YOUR_USERNAME placeholder
3. **Test locally** before submitting pull requests

## Advanced Solutions

### Dynamic Banners
Create context-aware banners using GitHub's features:

```markdown
<!-- Different banners for different contexts -->
![Banner](https://raw.githubusercontent.com/USER/REPO/main/assets/banner.png#gh-light-mode-only)
![Banner](https://raw.githubusercontent.com/USER/REPO/main/assets/banner-dark.png#gh-dark-mode-only)
```

### Responsive Banners
Use HTML for more control:

```html
<div align="center">
  <img src="https://raw.githubusercontent.com/USER/REPO/main/assets/banner.png" 
       alt="Project Banner" 
       width="100%" 
       style="max-width: 800px;">
</div>
```

## Troubleshooting Checklist

- [ ] File exists in repository
- [ ] File is committed to git
- [ ] File size is reasonable (<1MB)
- [ ] URL is correct and accessible
- [ ] Username is correct in URLs
- [ ] Branch name is correct (main/master)
- [ ] No typos in file path
- [ ] Image format is supported (PNG/JPG)
- [ ] Repository is public (for raw URLs)

## Quick Fix Commands

```bash
# Quick verification
./scripts/setup.sh  # Run project setup

# Manual checks
git status assets/
ls -la assets/banner.png
file assets/banner.png

# Test different approaches
echo "Test relative: ![Banner](assets/banner.png)"
echo "Test absolute: ![Banner](https://raw.githubusercontent.com/$(git config user.name)/cobot-rs/main/assets/banner.png)"
```

## Getting Help

If banner issues persist:

1. **Check GitHub Issues**: Look for similar problems
2. **Test in Incognito**: Rule out caching issues
3. **Try Different Browsers**: Verify it's not browser-specific
4. **Check Repository Settings**: Ensure repository is public if using raw URLs
5. **Create Issue**: Document the problem with screenshots

The banner should display correctly once the underlying issue is resolved. Most problems are related to file paths or repository configuration rather than the image itself.