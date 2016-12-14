all:
	cargo fmt
	@find src/ -type f -name '*.bk' -delete
	cargo build

test:
	cargo test

clean:
	cargo clean -p qselect


src/peg.rs: parser.rustpeg
	 peg < $^ > $@
