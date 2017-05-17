all: build

build:
	cargo build

test:
	cargo test

install:
	cargo build --release
	cp ./target/release/fallout /usr/local/bin/fallout

clean:
	rm -rf target/
	rm -rf ./xcodebuild_warnings.json
