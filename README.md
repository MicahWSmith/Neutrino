# Neutrino
The subatomic desktop engine. Compile native WASI components once, run anywhere with an OS native webview UI, zero Node.js, and zero HTTP servers.

Why Neutrino?
Zero-Weight Architecture: Operates with a ~15MB idle RAM footprint—10x lighter than Electron and cleaner than traditional desktop bridges.

Universal WASI Core: Write backend logic in Rust, C++, or Go. Compile to .wasm once and deploy across Windows, macOS, Linux, iOS, and Android without native target compilers for each platform.

Zero-Socket Memory IPC: Intercepts UI calls via native custom URI schemes (app://) in direct memory. No local TCP ports, no background web servers, no latency.

Bring Your Own Frontend: Totally framework-agnostic. Works seamlessly with React, Vue, Svelte, HTMX, or pure vanilla HTML/JS.
