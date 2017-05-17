all: build
	./target/debug/fallout ./samples/xcodebuild.log

build:
	cargo build

install:
	cargo build --release
	cp ./target/release/fallout /usr/local/bin/fallout

clean:
	rm -rf target/
	rm ./xcodebuild_warnings.json
