.PHONY: coverage build install_coverage_tool

build:
	@cargo build --release

install_coverage_tool:
	@cargo install cargo-tarpaulin

coverage: install_coverage_tool
	@cargo tarpaulin -v
