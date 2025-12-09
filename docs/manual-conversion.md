# Manual Draw.io to PNG Conversion Guide

This guide covers manual conversion of .drawio files to PNG format across different platforms and tools.

## Desktop Application (All Platforms)

### Installation

Download from [diagrams.net](https://www.diagrams.net/):
- **macOS**: draw.io-*.dmg
- **Windows**: draw.io-*.exe
- **Linux**: draw.io-*.AppImage or `snap install drawio`

### GUI Export

1. Open the .drawio file
2. File -> Export as -> PNG...
3. Configure options:
   - **Zoom**: 100% (or 200% for high resolution)
   - **Border Width**: 10 (adds padding)
   - **Selection Only**: Uncheck
   - **Include a copy of my diagram**: Check (embeds source)
   - **Transparent Background**: Uncheck for documentation
4. Click Export
5. Save alongside the .drawio file with same name

### Command Line Export

#### macOS

```bash
# Single file
/Applications/draw.io.app/Contents/MacOS/draw.io -x -f png -o output.png input.drawio

# With options (2x scale for high resolution)
/Applications/draw.io.app/Contents/MacOS/draw.io -x -f png \
  --border 10 \
  --scale 2 \
  -o output.png input.drawio

# Batch export all diagrams in current directory
for f in $(find . -name "*.drawio"); do
  /Applications/draw.io.app/Contents/MacOS/draw.io -x -f png -o "${f%.drawio}.png" "$f"
done

# Batch export with scale
for f in $(find . -name "*.drawio"); do
  /Applications/draw.io.app/Contents/MacOS/draw.io -x -f png --scale 2 -o "${f%.drawio}.png" "$f"
done
```

#### Windows

```powershell
# Single file
& "C:\Program Files\draw.io\draw.io.exe" -x -f png -o output.png input.drawio

# With options
& "C:\Program Files\draw.io\draw.io.exe" -x -f png --border 10 --scale 2 -o output.png input.drawio

# Batch export all diagrams
Get-ChildItem -Recurse -Filter "*.drawio" | ForEach-Object {
  $output = $_.FullName -replace '\.drawio$', '.png'
  & "C:\Program Files\draw.io\draw.io.exe" -x -f png -o $output $_.FullName
}
```

#### Linux

```bash
# Single file (AppImage)
./draw.io-*.AppImage -x -f png -o output.png input.drawio

# If installed via snap
drawio -x -f png -o output.png input.drawio

# Batch export
find . -name "*.drawio" -exec sh -c '
  drawio -x -f png -o "${1%.drawio}.png" "$1"
' _ {} \;
```

## VS Code Extension

### Installation

1. Open VS Code
2. Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Search "Draw.io Integration"
4. Install `hediet.vscode-drawio`

### Export

1. Open .drawio file in VS Code (opens visual editor)
2. Click three-dot menu in editor toolbar
3. Select "Export"
4. Choose PNG format
5. Configure options and save

## Online (app.diagrams.net)

### Export

1. Go to [app.diagrams.net](https://app.diagrams.net)
2. File -> Open from -> Device (select .drawio file)
3. File -> Export as -> PNG...
4. Configure options and download

### Limitations

- Requires internet connection
- Manual process (no automation)
- Consider privacy for sensitive diagrams

## Docker-Based Export

For CI/CD or headless environments:

```bash
# Using rlespinasse/drawio-export image
docker run --rm -v $(pwd):/data rlespinasse/drawio-export \
  --format png \
  --output /data \
  /data/diagram.drawio

# Batch export entire folder
docker run --rm -v $(pwd):/data rlespinasse/drawio-export \
  --format png \
  --folder /data \
  --output /data

# With options
docker run --rm -v $(pwd):/data rlespinasse/drawio-export \
  --format png \
  --border 10 \
  --scale 2 \
  --folder /data \
  --output /data
```

## Recommended Export Settings

For consistent output across the repository:

| Setting | Value | Reason |
|---------|-------|--------|
| Format | PNG | Universal compatibility |
| Scale | 2x | High resolution for zoom/print |
| Border | 10px | Visual padding |
| Transparent | No | Better for dark mode viewers |
| Embed diagram | Yes | Allows re-editing from PNG |

## Command Line Options Reference

| Option | Description | Example |
|--------|-------------|---------|
| `-x` | Export mode | Required for CLI export |
| `-f png` | Output format | `png`, `svg`, `pdf`, `jpg` |
| `-o file` | Output file | `-o output.png` |
| `--scale N` | Scale factor | `--scale 2` for 2x |
| `--border N` | Border width in pixels | `--border 10` |
| `--width N` | Fixed width in pixels | `--width 1920` |
| `--height N` | Fixed height in pixels | `--height 1080` |
| `--crop` | Crop to content | Removes whitespace |
| `--transparent` | Transparent background | Use for overlays |
| `--page-index N` | Export specific page | `--page-index 0` |
| `--all-pages` | Export all pages | Creates multiple files |

## Troubleshooting

### "File not found" errors

- Use absolute paths
- Ensure .drawio extension is included
- Check file permissions

### Poor quality output

- Increase scale factor (`--scale 2` or higher)
- Check source diagram zoom level
- Ensure fonts are available on system

### Transparent backgrounds render incorrectly

- Disable transparency for documentation
- Use white background for consistency
- Some viewers have issues with alpha channel

### Large file sizes

- Reduce scale if file size is a concern
- Consider SVG for web-only use
- Use PNG compression tools (pngquant, optipng)

### Missing fonts

- Install fonts on the system running export
- Use web-safe fonts in diagrams
- Embed fonts in desktop app settings

## Automated Export via GitHub Actions

This repository includes automated export via GitHub Actions. When you push changes to .drawio files:

1. GitHub Actions detects the change
2. `rlespinasse/drawio-export-action` exports all diagrams
3. PNG files are committed automatically

See `.github/workflows/drawio-export.yml` for configuration.

To trigger manual export:
1. Go to Actions tab in GitHub
2. Select "Export Draw.io Diagrams"
3. Click "Run workflow"
