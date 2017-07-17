###############################################################################
# Build Local
###############################################################################
init:
	cargo new book_teacher --bin

	cargo install cargo-outdated
	cargo install rustfmt

update:
	cargo outdated
	cargo update

fmt:
	cargo fmt

bld:
	cargo build
	#rustc main.rs

run:
	cargo run
	#./main

release:
	cargo build --release
