build:
	docker run --rm -v .:/home/rust/src messense/rust-musl-cross:x86_64-musl cargo build --release