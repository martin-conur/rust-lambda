watch: 
	cargo lambda watch

format: 
	cargo fmt --quiet

lint:
	cargo clippy --quiet

run: 
	cargo run

invoke:
	cargo lambda invoke --data-ascii "{ \"command\": \"Marco\" }"

release-arm: 
	cargo lambda build --release --arm64

deploy: 
	cargo lambda deploy

all:
	format lint 