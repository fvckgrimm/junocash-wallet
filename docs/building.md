# Building & Distribution Guide

## Prerequisites

### All Platforms

**Node.js & Package Manager:**
- Node.js 18+ ([nodejs.org](https://nodejs.org))
- Bun ([bun.sh](https://bun.sh)) - Recommended
- Or npm/pnpm/yarn (comes with Node.js)

**Rust:**
- Install from [rustup.rs](https://rustup.rs)
- Version 1.70 or newer

Verify installations:
```bash
node --version    # Should be v18.0.0 or higher
bun --version     # Should be 1.0.0 or higher
rustc --version   # Should be 1.70.0 or higher
```

### Linux

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Fedora/RHEL:**
```bash
sudo dnf install -y \
    webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

**Arch Linux:**
```bash
sudo pacman -S --needed \
    webkit2gtk \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg
```

**For AppImage builds (optional):**
```bash
# Install linuxdeploy
wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
chmod +x linuxdeploy-x86_64.AppImage
sudo mv linuxdeploy-x86_64.AppImage /usr/local/bin/linuxdeploy
```

### Windows

**Visual Studio Build Tools:**
- Download from [visualstudio.microsoft.com](https://visualstudio.microsoft.com/downloads/)
- Install "Desktop development with C++"
- Or install full Visual Studio Community

**WebView2:**
- Usually pre-installed on Windows 10/11
- If missing: [Download WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### macOS

**Xcode Command Line Tools:**
```bash
xcode-select --install
```

**Homebrew (optional but recommended):**
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

## Building from Source

### Clone Repository

```bash
git clone https://github.com/fvckgrimm/junocash-wallet.git
cd junocash-wallet
```

### Install Dependencies

```bash
bun install
# Or: npm install / pnpm install / yarn install
```

### Development Build

Start development server with hot reload:
```bash
bun run tauri dev
```

This will:
- Start Vite dev server on http://localhost:5173
- Launch Tauri app in development mode
- Enable hot module replacement
- Open DevTools automatically

**Development shortcuts:**
- `Ctrl+R` / `Cmd+R` - Reload frontend
- `F12` - Toggle DevTools
- `Ctrl+Q` / `Cmd+Q` - Quit app

### Production Build

Build optimized binaries:
```bash
bun run tauri build
```

Build process:
1. Compiles Vue app with Vite (production mode)
2. Builds Rust backend with optimizations
3. Creates platform-specific installers/bundles
4. Takes 2-5 minutes depending on hardware

## Build Outputs

### Linux

**Location:** `src-tauri/target/release/bundle/`

**Formats:**
- **Debian Package (.deb):**
  - `deb/junocash-wallet_0.1.0_amd64.deb`
  - Install: `sudo dpkg -i junocash-wallet_0.1.0_amd64.deb`
  - For Ubuntu, Debian, Linux Mint, Pop!_OS

- **AppImage:**
  - `appimage/junocash-wallet_0.1.0_amd64.AppImage`
  - Make executable: `chmod +x *.AppImage`
  - Run directly, no installation needed
  - Portable, works on most distros

- **Raw Binary:**
  - `src-tauri/target/release/junocash-wallet`
  - Standalone executable
  - No installer, just run it

**Recommended for distribution:** AppImage (most compatible)

### Windows

**Location:** `src-tauri\target\release\bundle\`

**Formats:**
- **MSI Installer:**
  - `msi\junocash-wallet_0.1.0_x64_en-US.msi`
  - Double-click to install
  - Creates Start Menu entry
  - Adds to Programs & Features

- **Portable EXE:**
  - `src-tauri\target\release\junocash-wallet.exe`
  - No installation required
  - Run from any location
  - Good for USB drives

**Recommended for distribution:** MSI (users expect installers on Windows)

### macOS

**Location:** `src-tauri/target/release/bundle/`

**Formats:**
- **DMG Image:**
  - `dmg/junocash-wallet_0.1.0_x64.dmg`
  - Standard macOS installer format
  - Drag to Applications folder
  - Most user-friendly

- **App Bundle:**
  - `macos/junocash-wallet.app`
  - Can be distributed directly
  - Zip and share

**Code Signing:**
- Unsigned builds will show security warning
- Users must right-click → Open to bypass Gatekeeper
- For distribution, consider signing with Apple Developer account

**Recommended for distribution:** DMG (expected on macOS)

## Build Optimization

### Reducing Binary Size

Edit `src-tauri/Cargo.toml`:
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Strip symbols
```

This can reduce size by 30-50% but increases build time.

### Build Without Bundling

Skip installer creation, just build binary:
```bash
bun run tauri build -- --no-bundle
```

Output: `src-tauri/target/release/junocash-wallet[.exe]`

Useful for:
- Quick testing
- Custom packaging
- CI/CD pipelines

## Platform-Specific Notes

### Linux: Choosing Bundle Format

**Use .deb if:**
- Targeting Ubuntu/Debian-based distros
- Want system integration (menu entries, file associations)
- Users comfortable with package managers

**Use AppImage if:**
- Want maximum compatibility
- Single file distribution
- Don't need deep system integration

**Skip AppImage build:**
If you get linuxdeploy errors and don't need AppImage:

Edit `src-tauri/tauri.conf.json`:
```json
{
  "bundle": {
    "targets": ["deb"]
  }
}
```

Or build with no bundles:
```bash
bun run tauri build -- --no-bundle
```

### Windows: MSVC vs GNU

Tauri uses MSVC toolchain by default (recommended).

To use GNU toolchain:
```bash
rustup target add x86_64-pc-windows-gnu
bun run tauri build -- --target x86_64-pc-windows-gnu
```

Stick with MSVC unless you have specific reasons.

### macOS: Universal Binaries

Build for both Intel and Apple Silicon:
```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
bun run tauri build -- --target universal-apple-darwin
```

Creates universal binary that works on both architectures.

## Cross-Compilation

### Linux → Windows

Possible but complex. Easier to build on Windows directly or use CI/CD.

### macOS → Other Platforms

Not supported. Must build on target platform.

**Solution:** Use GitHub Actions (see CI/CD section)

## Continuous Integration

### GitHub Actions Example

Create `.github/workflows/build.yml`:

```yaml
name: Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Setup Bun
        uses: oven-sh/setup-bun@v1

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install Linux dependencies
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt update
          sudo apt install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

      - name: Install dependencies
        run: bun install

      - name: Build
        run: bun run tauri build

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.os }}-build
          path: src-tauri/target/release/bundle/
```

Push a tag to trigger:
```bash
git tag v0.1.0
git push origin v0.1.0
```

## Troubleshooting Builds

### "Failed to bundle project"

**Linux:**
- Install all dependencies listed above
- Check `linuxdeploy` is in PATH (for AppImage)
- Try building without AppImage target

**Windows:**
- Install Visual Studio Build Tools
- Restart terminal after installation
- Check WebView2 is installed

**macOS:**
- Install Xcode Command Line Tools
- Accept Xcode license: `sudo xcodebuild -license accept`

### "ENOSPC: no space left on device"

Increase inotify watchers (Linux):
```bash
echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

Or close other dev servers/watchers.

### "error: linking with `cc` failed"

**Linux:**
- Install `build-essential` package
- Install specific library with error message

**Windows:**
- Install Visual Studio Build Tools
- Make sure "Desktop development with C++" is checked

### Slow build times

First build is always slow (10+ minutes). Subsequent builds are faster.

**Speed up:**
- Use SSD for project directory
- Increase RAM allocated to build (16GB+ recommended)
- Use `sccache` for Rust caching
- Close other heavy applications

### Binary too large

See "Build Optimization" section above.

Typical sizes:
- Debug build: 100-200MB
- Release build: 20-40MB
- Optimized release: 10-20MB

## Versioning

Update version in:
1. `src-tauri/Cargo.toml` → `version = "0.1.0"`
2. `src-tauri/tauri.conf.json` → `"version": "0.1.0"`
3. `package.json` → `"version": "0.1.0"`

Keep all three in sync.

## Distribution Checklist

Before releasing:

- [ ] Update version numbers
- [ ] Test on clean systems (VMs)
- [ ] Build for all target platforms
- [ ] Test installers work correctly
- [ ] Write changelog
- [ ] Create GitHub release
- [ ] Upload binaries to release
- [ ] Update documentation if needed
- [ ] Announce in community channels

## Code Signing (Optional)

### Windows

Requires code signing certificate ($100-300/year).

1. Get certificate from CA (Sectigo, DigiCert, etc.)
2. Sign binary with `signtool`:
```cmd
signtool sign /f certificate.pfx /p password /tr http://timestamp.digicert.com /td sha256 /fd sha256 junocash-wallet.exe
```

### macOS

Requires Apple Developer account ($99/year).

1. Get Developer ID certificate
2. Sign and notarize:
```bash
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" junocash-wallet.app
xcrun notarytool submit junocash-wallet.zip --keychain-profile "notary-profile" --wait
xcrun stapler staple junocash-wallet.app
```

### Linux

No code signing infrastructure. Use checksums:
```bash
sha256sum junocash-wallet_0.1.0_amd64.deb > checksums.txt
sha256sum junocash-wallet_0.1.0_amd64.AppImage >> checksums.txt
```

Publish checksums with release so users can verify.

## Building for Older Systems

**Minimum system requirements:**
- Linux: glibc 2.28+ (Ubuntu 18.04+)
- Windows: Windows 10 1803+
- macOS: 10.15+ (Catalina)

To support older systems, build on older OS versions or use compatibility layers (complex, not recommended).

## Resources

- [Tauri Build Documentation](https://tauri.app/v1/guides/building/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Vite Build Guide](https://vitejs.dev/guide/build.html)
- [GitHub Actions for Tauri](https://tauri.app/v1/guides/building/cross-platform)
