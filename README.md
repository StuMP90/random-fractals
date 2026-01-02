# random-fractals
A simple Rust GUI programme to test cross-platform compilation and packaging.

## Project layout

The GUI application lives in:

`random-fractals-gui/`

## Running (Linux developer build)

```bash
cd random-fractals-gui
cargo run
```

## Using the app

- **Select**: choose fractal type and set refresh time.
- **Start**: starts continuous rendering. A new random fractal is generated after each refresh interval.
- **Stop**: stops continuous rendering.
- **Exit**: closes the application.

The fractal image is drawn to fill the window (below the menu bar).

## Linux AppImage

Build:

```bash
cd random-fractals-gui
./packaging/linux/build_appimage.sh
```

Output:

- `random-fractals-gui/target/appimage/out/RandomFractals-x86_64.AppImage`

Note:

- The script downloads `appimagetool` on first run.

## Windows

### Option A (recommended): build on Windows

On a Windows machine with Rust installed:

```powershell
cd random-fractals-gui
cargo build --release
```

Output:

- `random-fractals-gui\target\release\random-fractals-gui.exe`

### Option B: cross-compile from Linux (GNU toolchain)

Add the target:

```bash
rustup target add x86_64-pc-windows-gnu
```

Install a MinGW-w64 toolchain (package name varies by distro), then:

```bash
cd random-fractals-gui
cargo build --release --target x86_64-pc-windows-gnu
```

Output:

- `random-fractals-gui/target/x86_64-pc-windows-gnu/release/random-fractals-gui.exe`
