#!/bin/sh

set -e

wasm-pack test --headless --chrome
wasm-pack test --node
wasm-pack build
(cd pkg && npm publish --access=public)