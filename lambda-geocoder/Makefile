watch: 
	cargo lambda watch

format: 
	cargo fmt --quiet

lint:
	cargo clippy --quiet

run: 
	cargo run

invoke:
	cargo lambda invoke --data-ascii "{\"lat\": \"-42.564\", \"lon\": \"-73.35\"}"

release-arm: 
	cargo lambda build --release --arm64

deploy: 
	cargo lambda deploy

all:
	format lint 