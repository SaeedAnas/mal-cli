
.PHONY: debug build-release release-linux-musl test clippy clippy-pedantic install install-debug

build-release:
	cargo build --release

release-mac: build-release
	strip target/release/mal
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/gitui-mac.tar.gz ./mal
	ls -lisah ./release/mal-cli.tar.gz

release-win: build-release
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/mal-cli-win.tar.gz ./mal.exe

release-linux-musl: build-linux-musl-release
	strip target/x86_64-unknown-linux-musl/release/mal
	mkdir -p release
	tar -C ./target/x86_64-unknown-linux-musl/release/ -czvf ./release/mal-cli-linux-musl.tar.gz ./gitui

build-linux-musl-debug:
	cargo build --target=x86_64-unknown-linux-musl --no-default-features

build-linux-musl-release:
	cargo build --release --target=x86_64-unknown-linux-musl --no-default-features

fmt:
	cargo fmt -- --check

clippy:
	touch src/main.rs
	cargo clean -p mal-cli -p rmal
	cargo clippy --all-features

clippy-pedantic:
	cargo clean -p mal-cli -p rmal
	cargo clippy --all-features -- -W clippy::pedantic

install:
	cargo install --path "." --offline
