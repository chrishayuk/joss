#!/bin/sh
# Builds the release binary natively and for WASI, replays every example and
# every original session through both, and checks that they type exactly the
# same page.
#
# Needs: rustup target add wasm32-wasip1, and wasmtime on the PATH.
set -eu
cd "$(dirname "$0")/.."

cargo build --release --quiet
cargo build --release --quiet --target wasm32-wasip1
native=target/release/joss
wasm=target/wasm32-wasip1/release/joss.wasm

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

total=0
failed=0
for sitting in examples/*.joss; do
    n=0
    "$native" "$sitting" > "$work/native.out" 2> "$work/native.err" || n=$?
    w=0
    wasmtime run --dir=. "$wasm" "$sitting" > "$work/wasm.out" 2> "$work/wasm.err" || w=$?
    total=$((total + 1))
    if [ "$n" != "$w" ] || ! cmp -s "$work/native.out" "$work/wasm.out" || ! cmp -s "$work/native.err" "$work/wasm.err"; then
        failed=$((failed + 1))
        echo "DIFFERS: $sitting"
    fi
done
"$native" --verify transcripts/*.txt > "$work/native.out"
wasmtime run --dir=. "$wasm" --verify transcripts/*.txt > "$work/wasm.out"
total=$((total + 1))
if ! cmp -s "$work/native.out" "$work/wasm.out"; then
    failed=$((failed + 1))
    echo "DIFFERS: the original sessions"
fi

echo "$total compared, $failed differ between native and WASI"
[ "$failed" -eq 0 ]
