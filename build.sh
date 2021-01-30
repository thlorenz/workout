#!/bin/sh

set -ex

wasm-pack build --target web
echo "http://localhost:3000" && python -m SimpleHTTPServer 3000 .
