# Neutrino

Neutrino is a runtime for local apps built with the web stack.

Build the interface with HTML, CSS, and JavaScript. Ship the assets with a small native host. Open the app in the operating system's webview without starting Node.js, binding a localhost port, or embedding an entire browser runtime.

## The Idea

Neutrino is aimed at the space between a browser tab and a heavyweight desktop wrapper:

- **Web technologies for the UI:** Use the tools and skills of the web platform.
- **A native webview for presentation:** Use the webview already provided by the operating system.
- **An in-process bridge:** Route UI requests through an `app://` protocol handled directly by the host.
- **No local server:** There is no HTTP listener, localhost port, or socket-based runtime in the shipped app.
- **Portable app assets:** Keep the application layer in HTML, CSS, and JavaScript instead of coupling it to a platform-specific UI framework.

The host itself is a small platform adapter and must be compiled for its target operating system. The app assets and bridge contract are the portable part.

## How It Fits

```text
HTML / CSS / JavaScript
					|
			 app://
					|
Native Neutrino host
					|
Optional local backend component
```

The backend is replaceable. Phase 1 uses a Rust component through the WASI Component Model, but the central Neutrino concept is the local webview runtime and its direct bridge, not requiring every app to be written in Rust or WASM.

## Compared With Other Approaches

| Approach | UI assets | Runtime model | Localhost server | Bundled browser |
| --- | --- | --- | --- | --- |
| Neutrino | HTML, CSS, JavaScript | Native webview plus host bridge | No | No |
| Electron | HTML, CSS, JavaScript | Chromium plus Node.js | Usually available, not required | Yes |
| Tauri | HTML, CSS, JavaScript | Native webview plus generated Rust application | Usually no | No |

Neutrino explores a narrower runtime model than a full application framework: a reusable local web-app host and a direct `app://` bridge, without requiring each application to carry a Node runtime or a framework-generated platform shell.

## Phase 1 Status

The current proof of concept includes:

- A native macOS host using `wry` and `tao`.
- An `app://` custom protocol serving the UI and dispatching bridge requests.
- A small `ping` and `echo` backend component.
- WIT bindings for the host/component contract.

Run the host from the repository root:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
cargo build -p neutrino-core --target wasm32-wasip2 --release
NEUTRINO_COMPONENT="$PWD/target/wasm32-wasip2/release/neutrino_core.wasm" \
	cargo run -p neutrino-host
```

On Windows (PowerShell), with [Rust](https://rustup.rs) and the [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) installed:

```powershell
rustup target add wasm32-wasip2
cargo build -p neutrino-core --target wasm32-wasip2 --release
$env:NEUTRINO_COMPONENT = "$PWD\target\wasm32-wasip2\release\neutrino_core.wasm"
cargo run -p neutrino-host
```

Windows ships WebView2 with modern Windows 10/11, so no separate browser runtime install is needed.

## Packaging

### macOS

```sh
./scripts/package-macos.sh
```

This builds a `Neutrino.app` bundle under `target/Neutrino.app`, ad-hoc signs it, and prints the path to open it with.

### Windows

```powershell
.\scripts\package-windows.ps1
```

This builds a relocatable app folder under `target\Neutrino\` containing `neutrino.exe` and a `Resources\` folder with the UI assets and the `neutrino_core.wasm` component. Copy the whole `target\Neutrino\` folder anywhere and run `neutrino.exe` to launch it.

## Lightweight Runtime

The current macOS proof of concept produces an 18.68 MB native host binary and a 73.42 KiB backend component.

Neutrino stays lightweight by using the system webview instead of bundling Chromium and Node.js like Electron. It also keeps the app layer as ordinary HTML, CSS, and JavaScript rather than requiring a generated Tauri application shell for each project.
