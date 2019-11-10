all: build run open
build: build-skylake

build-general:
	cargo build --release

build-skylake:
	RUSTFLAGS="-Ctarget-cpu=skylake" cargo build --release

run:
	./target/release/battlesnake-2020

run-debug:
	./target/debug/battlesnake-2020

clean:
	rm -rf target

.PHONY: build build-general build-skylake run run-debug clean
