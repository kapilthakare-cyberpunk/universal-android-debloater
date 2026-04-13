# Release Guide — Universal Android Debloater

> How to create a new release. Takes ~2 minutes.

---

## Automated Release (Recommended)

The CI pipeline handles everything: building, packaging, and publishing.

### Step 1: Update Version

Edit `Cargo.toml` and update the version:

```toml
version = "0.7.0"  # Change this to your new version
```

### Step 2: Commit & Tag

```bash
git add Cargo.toml
git commit -m "chore: bump version to 0.7.0"
git tag v0.7.0
git push origin main --tags
```

### Step 3: Wait for CI

The `release.yml` workflow will automatically:
1. ✅ Build for macOS (WGPU + OpenGL)
2. ✅ Build for Windows (WGPU + OpenGL)
3. ✅ Build for Linux (WGPU + OpenGL)
4. ✅ Create .dmg installers (macOS)
5. ✅ Create .exe/.zip packages (Windows)
6. ✅ Create .deb/.AppImage packages (Linux)
7. ✅ Publish to GitHub Releases

### Step 4: Verify

Visit: `https://github.com/kapilthakare/universal-android-debloater/releases/tag/v0.7.0`

Check that all platform packages are present and downloadable.

---

## Manual Release (Local Packaging)

If you need to build locally:

```bash
# Build and package for all platforms
just release

# Or individually:
just dmg          # macOS
just windows      # Windows
just linux        # Linux
```

Output files will be in `dist/`. Upload them manually to GitHub Releases.

---

## Manual Release via GitHub Actions

Trigger the workflow without a tag:

1. Go to **Actions** → **Release**
2. Click **Run workflow**
3. Enter version (e.g., `0.7.0-dev`)
4. Click **Run workflow**

---

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0) — Breaking changes
- **MINOR** (0.7.0) — New features, backward compatible
- **PATCH** (0.6.1) — Bug fixes

Pre-release tags: `v0.7.0-beta.1`, `v0.7.0-rc.1`

---

## Checklist Before Release

- [ ] All tests pass (`just test`)
- [ ] Clippy has no warnings (`just lint`)
- [ ] CHANGELOG.md is updated
- [ ] Version bumped in Cargo.toml
- [ ] Tag follows semver format (`vX.Y.Z`)
- [ ] Release notes describe changes clearly

---

## After Release

1. Update Homebrew cask (if applicable):
   ```bash
   brew bump-cask-pr universal-android-debloater --version 0.7.0
   ```

2. Update AUR package (if applicable)

3. Announce on relevant channels
