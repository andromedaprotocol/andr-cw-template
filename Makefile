.PHONY: test-unit test-integration test lint build

# Runs unit tests
test-unit:
	@echo "Running unit tests..."
	@cargo unit-test --workspace --quiet
	@echo "Unit tests complete! \033[0;32m\xE2\x9C\x94\033[0m"

# Runs integration tests
test-integration:
	@echo "Running integration tests..."
	@cargo test -p e2e-tests --quiet
	@echo "Integration tests complete! \033[0;32m\xE2\x9C\x94\033[0m"

# Runs all tests
test: test-unit test-integration
	@echo "All tests complete! \033[0;32m\xE2\x9C\x94\033[0m"

# Runs lint checks
lint:
	cargo fmt --all
	cargo clippy --workspace -- -D warnings

# Builds optimized WASM using Docker
build:
	docker run --rm -v "$(pwd)":/code \
		--mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/optimizer:0.16.0