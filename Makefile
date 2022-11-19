.PHONY: run
run: build
	echo "$(initrd)"

.PHONY: build
build:
	cargo build
	codesign --entitlements cruise.entitlements -s - target/debug/cruise

check:
	cargo check

clean:
	cargo clean
