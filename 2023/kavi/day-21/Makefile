run:
	@cargo run --release
.PHONY: run

test: coverage
	@cargo llvm-cov
.PHONY: test

coverage:
	@cargo llvm-cov --lcov --output-path ./lcov.info
.PHONY: coverage%