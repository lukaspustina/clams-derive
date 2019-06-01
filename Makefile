all: check build test

todos:
	rg --vimgrep -g '!Makefile' -i todo 

check:
	cargo check

build:
	cargo build

test:
	cargo test

release: release-bump all
	git commit -am "Bump to version $$(cargo read-manifest | jq .version)"
	git tag v$$(cargo read-manifest | jq -r .version)

release-bump:
	cargo bump

clippy:
	rustup run nightly cargo clippy

fmt:
	rustup run nightly cargo fmt

duplicate_libs:
	cargo tree -d

_update-clippy_n_fmt:
	rustup update
	cargo install clippy --force
	cargo install rustfmt --force

_cargo_install:
	cargo install -f cargo-tree
	cargo install -f cargo-bump
