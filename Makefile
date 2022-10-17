
clean:
	rm -rf target || true


all: 
	cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release --target wasm32-unknown-unknown
	mkdir -p bundle/wasm || true
	cp ./target/wasm32-unknown-unknown/release/{{crate_name}}.wasm  bundle/wasm/{{crate_name}}.wasm
	cd bundle && zip -r ../bundle.zip .




	

