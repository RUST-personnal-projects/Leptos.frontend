build:
    cargo build --target wasm32-unknown-unknown

serve OPTIONS="":
    trunk serve {{OPTIONS}}