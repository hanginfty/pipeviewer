host=x86_64-unknown-linux-gnu

build-std:
	cargo +nightly build -Z build-std=std,panic_abort --target $(host) --release

panic-immediate:
	cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target $(host) --release

pack:
	upx --best --lzma target/$(host)/release/pipeviewer
