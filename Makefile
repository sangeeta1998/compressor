.PHONY: all clean create_file run_native run_wasm run_script

all: create_file run_native run_wasm run_script

# Create a 1GB file filled with random data
create_file:
	dd if=/dev/urandom of=large_file.txt bs=1M count=1024

run_native:
	time docker run --rm --mount type=bind,source="$(pwd)",target=/app sangeetakakati/rust-compressor-native

run_wasm:
	time docker run --rm --runtime=io.containerd.wasmtime.v1 --platform=wasm --mount type=bind,source="$(pwd)",target=/app sangeetakakati/rust-compressor-wasm

run_script:
	./script.sh

clean:
	docker rmi sangeetakakati/rust-compressor-native
	docker rmi sangeetakakati/rust-compressor-wasm

