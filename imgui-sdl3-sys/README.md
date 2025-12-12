# imgui-sdl3-sys

Low-level Rust bindings for Dear ImGui (via dcimgui C bindings) with SDL3 backend.

This crate provides automatically generated bindings to the cimgui C API for Dear ImGui.

## Features

- `build-from-source`: Build and link Dear ImGui from source using cmake
- `build-from-source-static`: Build and statically link Dear ImGui from source
- `link-static`: Link against a static Dear ImGui library
- `no-link`: Don't link anything, provide linking flags via Cargo metadata

## Building

The bindings are generated automatically using [bindgen](https://github.com/rust-lang/rust-bindgen) when building with the `build-from-source` feature.

```bash
cargo build --features build-from-source-static
```

## Generated Bindings

The bindings are located in `src/generated/mod.rs` and include:

- All Dear ImGui functions (prefixed with `ig`, e.g., `igCreateContext`)
- All Dear ImGui types (e.g., `ImGuiContext`, `ImDrawData`, `ImVec2`)
- All Dear ImGui constants and enums

## Example Usage

```rust
use imgui_sys::*;

unsafe {
    let ctx = igCreateContext(core::ptr::null_mut());
    igSetCurrentContext(ctx);

    // Your ImGui code here

    igDestroyContext(ctx);
}
```

## License

This crate is licensed under the MIT license.
