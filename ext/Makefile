.PHONY: rs
rs:
	cd ../ && cargo build --release
	mkdir -p ../lib/sqlmod
	cp ../target/release/lib*.* ../lib/sqlmod/

.PHONY: install
install: rs
	@echo "Doing nothing for install..." >&2
