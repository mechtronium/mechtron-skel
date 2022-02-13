App {

  Set {
    +wasm.src=$(self.config.bundle):/wasm/my-mechtron.wasm,
    +mechtron.name=my-mechtron,
    +bind=$(self.config.bundle):/bind/my-mechtron.bind
  }

}
