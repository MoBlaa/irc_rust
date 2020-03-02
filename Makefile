.PHONY: coverage build install_coverage_tool

build:
	@cargo build

install_coverage_tool:
	@cargo install cargo-tarpaulin

coverage: install_coverage_tool
	@cargo tarpaulin -v
