
clean:
	rm -rf target || true


release: 
	cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release --target wasm32-unknown-unknown
	cp ./target/wasm32-unknown-unknown/release/{{crate-name}}.wasm  bundle/wasm/my-mechtron.wasm
	zip -r ./target/bundle.zip bundle


debug: 
	cargo build --target wasm32-unknown-unknown
	cp ./target/wasm32-unknown-unknown/debug/{{crate-name}}.wasm  bundle/wasm/my-mechtron.wasm
	zip -r ./target/bundle.zip bundle




	

