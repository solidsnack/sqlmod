.PHONY: rs
rs:
	cd ../ && cargo build --release
	mkdir -p ../lib/query_selector
	cp ../target/release/lib*.* ../lib/query_selector/

.PHONY: install
install: rs
	@echo "Doing nothing for install..." >&2
