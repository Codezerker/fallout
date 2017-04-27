all: build
	./target/debug/fallout ./samples/xcodebuild.log

build:
	cargo build

clean:
	rm -rf target/
	rm ./xcodebuild_warnings.json
