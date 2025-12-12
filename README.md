# imgui-sdl3-sys

# Build the main crate with static linking (default)
```sh
cargo build
```

# Build with explicit static features
```sh
cargo build --features build-from-source-static
```

# Build example with static linking (default)
```sh
cd examples/imgui-sdl3-sdlgpu3
cargo build
```

# Build the main crate with shared libraries
```sh
cargo build --no-default-features --features build-from-source
```

# Build example with shared libraries
```sh
cd examples/imgui-sdl3-sdlgpu3
cargo build --no-default-features --features imgui-sdl3-sys/build-from-source
```

# Release static
```sh
cargo build --release
```
# Release shared
```sh
cargo build --release --no-default-features --features build-from-source
```