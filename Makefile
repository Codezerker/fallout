all: build
	./target/debug/fallout ./resources/xcodebuild.log

build:
	cargo build