# imgui-sdl3-sys

What is this? Another set of low level, unsafe Dear ImGui bindings for Rus, focused on SDL3 integration.

Should your use this? Probably no. This is mostly for my personal usage. Many things can, and will change and break you. Take it as a reference.

Then, why?
- There are several ImGui bindings crates, but unmaintained, or lacking what I need
- Specifically, I need the SDL3 backend, and SDL GPU. Nothing more
- I don´t want to depend on the high level SDL3 crate
- It's easier for me to control/bump ImGui version
- I simply re-export the whole SDL3 (sys), so I don´t need a specific dependency to sdl3-sys in consuming code

Non-goals:
- High level, safe, idiomatic bindings: I don´t care much, for my usage unsafe and conversions are acceptable
- Other backends: I don´t need them

Maybe goals:
- ImGui extensions if I need them
- ImGui docking branch if needed

Notes:

- I've used LLMs, specifically Claude, to help with build.rs, usage of bindgen, C API for ImGui backends, and translation of a example from C++ to Rust.
- imgui-sdl3-sys/docs contains details about what Claude did and issues faced

# Build

## Static linking (default)
```sh
cargo build
```

## Explicit static features
```sh
cargo build --features build-from-source-static
```

## Example with static linking (default)
```sh
cd examples/imgui-sdl3-sdlgpu3
cargo build
```

## Shared libraries
```sh
cargo build --no-default-features --features build-from-source
```

## Example with shared libraries
```sh
cd examples/imgui-sdl3-sdlgpu3
cargo build --no-default-features --features imgui-sdl3-sys/build-from-source
```

## Release static
```sh
cargo build --release
```
## Release shared
```sh
cargo build --release --no-default-features --features build-from-source
```