FILE ?=

run:
	cargo run -- $(FILE)

run-web:
	cargo build --target wasm32-unknown-emscripten --release
	(sleep 1; xdg-open http://localhost:8080) &
	python3 -m http.server 8080
