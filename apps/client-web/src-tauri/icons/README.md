# Tauri Icons

This directory contains icons for the Tauri desktop application.

## Required Files

- `32x32.png` - 32x32 pixel icon
- `128x128.png` - 128x128 pixel icon
- `128x128@2x.png` - 256x256 pixel icon (2x retina)
- `icon.ico` - Windows icon file
- `icon.icns` - macOS icon file

## Quick Setup

### Option 1: Use Tauri CLI (Recommended)

```bash
# Install Tauri CLI if not already installed
pnpm add -D @tauri-apps/cli

# Generate icons from a source image (1024x1024 PNG recommended)
cd apps/client-web/src-tauri
npx tauri icon path/to/your-icon.png
```

This will automatically generate all required icon sizes and formats.

### Option 2: Manual Creation

1. Create a 1024x1024 PNG icon with your logo
2. Use online tools to convert:
   - PNG to ICO: <https://convertio.co/png-ico/>
   - PNG to ICNS: <https://cloudconvert.com/png-to-icns>
3. Resize to required sizes:
   - `<32x32.png>`
   - `<128x128.png>`
   - `<128x128@2x.png>` (256x256)

### Option 3: Use ImageMagick (if installed)

```bash
# Convert PNG to ICO
magick convert icon-256.png -define icon:auto-resize=256,128,64,48,32,16 icon.ico

# Create ICNS (macOS only)
iconutil -c icns icon.iconset
```

## Current Status

⚠️ **Icons are currently disabled in dev mode** (`bundle.active: false` in `tauri.conf.json`).

To enable icons for production builds:

1. Generate icons using one of the methods above
2. Set `bundle.active: true` in `tauri.conf.json`
