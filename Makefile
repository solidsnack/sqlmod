all:
	cargo fmt
	@find src/ -type f -name '*.bk' -delete
	cargo build --features codegen

test:
	cargo test

clean:
	cargo clean -p sqlmod
