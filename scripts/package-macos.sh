#!/bin/sh
set -eu

if [ $# -lt 1 ]; then
    printf 'usage: %s <ui-dir>\n' "$0" >&2
    exit 1
fi
ui_dir=$(CDPATH= cd -- "$1" && pwd)

export PATH="$HOME/.cargo/bin:$PATH"
root_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root_dir"

cargo build -p neutrino-core --target wasm32-wasip2 --release
cargo build -p neutrino-host --release

app_dir="$root_dir/target/Neutrino.app"
contents_dir="$app_dir/Contents"
resources_dir="$contents_dir/Resources"

rm -rf "$app_dir"
mkdir -p "$contents_dir/MacOS" "$resources_dir"
cp target/release/neutrino "$contents_dir/MacOS/neutrino"
cp target/wasm32-wasip2/release/neutrino_core.wasm "$resources_dir/neutrino_core.wasm"
cp -R "$ui_dir" "$resources_dir/ui"
cp packaging/macos/Info.plist "$contents_dir/Info.plist"

if command -v codesign >/dev/null 2>&1; then
    codesign --force --deep --sign - "$app_dir"
fi

printf 'Created %s\n' "$app_dir"
printf 'Open it with: open %s\n' "$app_dir"
