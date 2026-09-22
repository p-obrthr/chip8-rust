FILE ?=

run:
	cargo run -- $(FILE)

run-web:
	export EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1"
	cargo build --target wasm32-unknown-emscripten --release
	(sleep 1; xdg-open http://localhost:8080) &
	python3 -m http.server 8080
