#!/bin/bash

echo "Building Game Boy Emulator for WebAssembly..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WebAssembly module
wasm-pack build --target web --out-dir pkg

echo "Build complete! To run the emulator:"
echo "1. Start a local web server in the project directory"
echo "2. Open web/index.html in your browser"
echo ""
echo "Example using Python:"
echo "python3 -m http.server 8000"
echo "Then open http://localhost:8000/web/"