## Development

This project builds and packages for four platforms from a single codebase: **Terminal** (native binary), **Web** (WASM on GitHub Pages), **macOS** (native GUI), and **Windows** (native GUI). Make sure you meet the [development prerequisites](./markdowns/DEVELOPMENT_PREREQUISITES.md) first.

**Build the tools** (from the repo root):

Build all three tools (config, package, run):

```bash
cargo build-tools
```

Then copy the freshly built binaries to the project root:

**macOS:**

```bash
cp target/release/config target/release/package target/release/run .
chmod +x config package run
```

**Windows (PowerShell):**

```powershell
copy target\release\config.exe .
copy target\release\package.exe .
copy target\release\run.exe .
```

The tools are then executed from the project root (`./config`, `./package`, `./run` on macOS; `config.exe`, `package.exe`, `run.exe` on Windows).

**Configure the app:**

```bash
./config
```

Sets up the app (name, icon, etc.). Can be re-run at any time to change values. App code lives in `src/` — edit `app.rs` and `platform.rs`.

**Run / develop:**

```bash
./run
```

Launches an interactive menu to build and run for a chosen platform (Terminal, Web/WASM, macOS, or Windows).

**Package & publish:**

```bash
./package
```

Opens the interactive menu to bundle (and optionally publish) the app for your chosen targets.

**Publish with GitHub Actions:**

Two workflows in `.github/workflows/` build and distribute for you on GitHub's servers:

- **Create Downloadable Binaries** — builds the Terminal binary for all four platforms (macOS ARM, macOS Intel, Windows, Linux) plus the macOS and Windows native apps, and attaches them as `.zip` assets to a GitHub Release (or a rolling `latest` tag on manual runs).
- **Deploy to GitHub Pages** — builds the WASM/web version and deploys it as a static site, automatically on every push to `main` (or manually).

> Note: the workflows require an `INCREDIBLE_ALPHA` secret (a GitHub token with read access to the private crate) under Settings > Secrets and variables > Actions.

### Workflows vs. package tool

Some, but not all, of what the **Create Downloadable Binaries** workflow does can also be done locally with the **Package tool**. `./package` builds and bundles the same targets for the platform you are running on, and `./package --publish` attaches the release assets to a GitHub Release. Note that the package tool builds only the platform it runs on, whereas GitHub Actions build the binaries for all platforms on separate runners in parallel.

## Download & Run with Docker

Build the Docker image:

```bash
docker build -t incredible .
```

Run the container:

```bash
docker run -it incredible
```

This downloads the latest release binary from GitHub and runs it inside the container.

---

[Home](./index.md) | [Markdowns](./markdowns/index.md)