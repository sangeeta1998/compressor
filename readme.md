
dd if=/dev/urandom of=large_file.txt bs=1M count=1024

#This command will create a 1GB file filled with base64-encoded zeros. 

```Native```

docker buildx build --output "type=image,push=true" -t sangeetakakati/rust-compressor-native -f Dockerfile.native --builder default .

time docker run --rm --mount type=bind,source="$(pwd)",target=/app sangeetakakati/rust-compressor-native

```Wasm```

docker buildx build --platform wasm --output "type=image,push=true" -t sangeetakakati/rust-compressor-wasm -f Dockerfile.wasm --builder default .

time docker run --rm --runtime=io.containerd.wasmtime.v1 --platform=wasm --mount type=bind,source="$(pwd)",target=/app sangeetakakati/rust-compressor-wasm


