default:
    # List all available commands
    just --list
build:
    # Build the WebAssembly package
    wasm-pack build
clean:
    # Clean the build files
    cargo clean
    rm -rf pkg
