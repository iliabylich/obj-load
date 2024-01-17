build-deb:
    cargo build --target=x86_64-unknown-linux-musl --release -p obj-down-up-load-server
    strip target/x86_64-unknown-linux-musl/release/obj-down-up-load-server
    cargo deb --deb-revision="$(date +%s)" --target=x86_64-unknown-linux-musl -p obj-down-up-load-server
