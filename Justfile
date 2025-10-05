default: fmt lint test mutate

# This help screen
show-help:
	just --list

# Test it was built ok
test:
	RUST_BACKTRACE=1 cargo test

# Build release version
build:
	cargo build --release

# Check performance
bench:
	cargo bench

# Lint it
lint:
	cargo +nightly fmt --all -- --check
	cargo +nightly clippy --all-features
	cargo +nightly check
	cargo +nightly audit

# Format what can be formatted
fmt:
	cargo +nightly fix --allow-dirty --allow-staged
	cargo +nightly clippy --allow-dirty --allow-staged --fix -Z unstable-options --all-features
	cargo +nightly fmt --all
	npx prettier --write **.yml

# Clean the build directory
clean:
	cargo clean

# Benchmark staged vs. unstaged changes using hyperfine
bench-git:
	#!/usr/bin/env bash
	set -euo pipefail

	# Create temporary files
	STAGED_FILE=$(mktemp)
	UNSTAGED_FILE=$(mktemp)

	# Get staged changes
	git diff --staged > "$STAGED_FILE"

	# Get unstaged changes
	git diff > "$UNSTAGED_FILE"

	# Run hyperfine benchmark
	hyperfine --warmup 3 \
		"cat $STAGED_FILE | cargo run --quiet -- -" \
		"cat $UNSTAGED_FILE | cargo run --quiet -- -"

	# Clean up
	rm "$STAGED_FILE" "$UNSTAGED_FILE"

# Run mutation tests
mutate:
    cargo mutants --all-features --diff --shuffle

# Set the Concourse pipeline
set-pipeline:
	fly -t ci-mgmt set-pipeline -p termdiff -c ci/concourse.yaml

