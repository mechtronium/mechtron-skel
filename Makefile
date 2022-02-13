
clean:
	rm -rf target || true


release: 
	cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release --target wasm32-unknown-unknown
	mkdir bundle/wasm || true
	cp ./target/wasm32-unknown-unknown/release/{{crate_name}}.wasm  bundle/wasm/my-mechtron.wasm
	zip -r ./target/bundle.zip bundle


debug: 
	cargo build --target wasm32-unknown-unknown
	mkdir bundle/wasm || true
	cp ./target/wasm32-unknown-unknown/debug/{{crate_name}}.wasm  bundle/wasm/my-mechtron.wasm
	zip -r ./target/bundle.zip bundle




	

