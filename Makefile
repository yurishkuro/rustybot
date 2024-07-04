.PHONY: test
test:
	cargo test

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: cover
cover:
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
	grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html
