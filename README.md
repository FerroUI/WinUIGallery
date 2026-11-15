# WinUI Gallery on FerroUI

This repository is the source and the deployment of the browser site of the WinUI Gallery of
[FerroUI](https://github.com/wieslawsoltes/FerroUI), a cross-platform UI framework for Rust. The
gallery shows the WinUI controls of FerroUI with their samples, in the WinUI theme, as a WebAssembly
module on a canvas.

**Open it in the browser:** https://ferroui.github.io/WinUIGallery/

The gallery is the sample `samples/WinUIGallery` of FerroUI
([its README](https://github.com/wieslawsoltes/FerroUI/blob/main/samples/WinUIGallery/README.md)),
built from the sources of the FerroUI submodule of this repository. This repository adds only what
the browser needs: a host that starts the gallery in the page, the host page, and the workflow that
builds the site and publishes it to GitHub Pages.

## Repository layout

| Path | Content |
|---|---|
| `WinUIGallery.Browser/` | the browser host (`winui-gallery-browser`): the WebAssembly entry point and its host page in `wwwroot/` |
| `FerroUI/` | FerroUI as a git submodule; the gallery and the framework crates build from it |
| `.github/workflows/ci.yml` | the build of the site and its deployment, with FerroUI's shared application workflow |

The repository follows the layout of FerroUI applications described in
[FerroUI/docs/apps.md](https://github.com/wieslawsoltes/FerroUI/blob/main/docs/apps.md): the workspace
dependencies and profiles of `Cargo.toml` are kept in step with the submodule by
`FerroUI/scripts/apps/sync-workspace.py`.

## Build

Every push to `main` builds the site and deploys it to GitHub Pages; pull requests build it and run
the smoke test without deploying. To build the site yourself:

```sh
git clone --recurse-submodules https://github.com/FerroUI/WinUIGallery.git
cd WinUIGallery
FerroUI/scripts/browser/setup.sh .tools
source .tools/env.sh
FERROUI_BROWSER_WORKSPACE=$PWD FerroUI/scripts/build-browser.sh winui-gallery-browser
```

`setup.sh` installs the browser toolchain FerroUI pins (the `wasm32-unknown-emscripten` target,
Emscripten and the `wasm-bindgen` command-line tool); see
[FerroUI/docs/porting/browser-platform.md](https://github.com/wieslawsoltes/FerroUI/blob/main/docs/porting/browser-platform.md).
Serve the directory the script prints with any static web server.

The query string of the page selects the renderer and the first page: `?RenderingMode=Software2D`
draws without WebGL, and `?page=Button` opens a page of the catalog.

## Update FerroUI

```sh
git -C FerroUI fetch origin && git -C FerroUI checkout origin/main
python3 FerroUI/scripts/apps/sync-workspace.py .
cargo update --workspace --offline
```

## Licence

MIT, as FerroUI ([LICENSE](LICENSE)). See [NOTICE.md](NOTICE.md) for the notices of the gallery.
