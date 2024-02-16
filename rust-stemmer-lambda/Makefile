watch: 
	cargo lambda watch

format: 
	cargo fmt --quiet

lint:
	cargo clippy --quiet

run: 
	cargo run

invoke:
	cargo lambda invoke --data-ascii "{ \"phrase\": \"desesperadamente todo todisimo fue en vano\", \"language\": \"arab\" }"

release-arm: 
	cargo lambda build --release --arm64

deploy: 
	cargo lambda deploy

all:
	format lint 