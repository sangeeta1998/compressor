
dd if=/dev/urandom of=large_file.txt bs=1M count=1024
#This command will create a 1GB file filled with base64-encoded zeros, which should compress much better. After creating this file, you can try compressing it again and check the size of the compressed file. 
 docker buildx build --output "type=image,push=true" -t sangeetakakati/rust-compressor-native -f Dockerfile.native --builder default .

time docker run --rm --mount type=bind,source="$(pwd)",target=/app sangeetakakati/rust-compressor-native

docker buildx build --platform wasm --output "type=image,push=true" -t sangeetakakati/rust-compressor-wasm -f Dockerfile.wasm --builder default .

time docker run --rm --runtime=io.containerd.wasmtime.v1 --platform=wasm --mount type=bind,source="$(pwd)",target=/app sangeetakakati/rust-compressor-wasm


