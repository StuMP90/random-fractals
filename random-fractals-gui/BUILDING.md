# Building

## Linux (developer build)

```bash
cargo build
cargo run
```

## Linux AppImage

The repository includes a simple AppImage build script.

Requirements:
- `bash`
- `curl`
- A reasonably standard Linux system capable of running AppImages

Build:

```bash
./packaging/linux/build_appimage.sh
```

Output:
- `target/appimage/out/RandomFractals-x86_64.AppImage`

Notes:
- The script downloads `appimagetool` into `target/appimage/out/` on first run.

## Windows

### Option A (recommended): native Windows build

On a Windows machine with Rust installed:

```powershell
cargo build --release
```

Output:
- `target\release\random-fractals-gui.exe`

This produces a runnable `.exe` without requiring the user to install anything else.

### Option B: cross-compile from Linux (GNU toolchain)

1. Add the Windows target:

```bash
rustup target add x86_64-pc-windows-gnu
```

2. Install a MinGW-w64 toolchain (package name varies by distro), then build:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

Output:
- `target/x86_64-pc-windows-gnu/release/random-fractals-gui.exe`

If you want an installer later, we can add one (e.g. with Inno Setup on Windows), but a standalone `.exe` is usually simplest.
