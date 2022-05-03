
get: get/image get/onnx

get/image:
	curl "https://upload.wikimedia.org/wikipedia/commons/e/ea/Kishida_Cabinet.jpg" \
		-o resource/cabinet.jpg --create-dirs

get/onnx:
	curl "https://raw.githubusercontent.com/Star-Clouds/CenterFace/master/models/onnx/centerface.onnx" \
	 	-o resource/centerface.onnx --create-dirs

detect:
	cargo run --package wasabi --bin detect $(FILE)

wasm/build:
	wasm-pack build
