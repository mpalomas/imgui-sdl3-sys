# ImGui Backend C Wrappers

This directory contains C wrappers for the Dear ImGui C++ backends, providing a stable C ABI.

## Why C Wrappers?

The ImGui backends (SDL3 and SDL_GPU) are written in C++. Binding directly to C++ functions causes several problems:

### ❌ Problems with Direct C++ Bindings:

1. **C++ Name Mangling** - Function names are mangled differently by different compilers:
   ```rust
   // Old bindings with C++ mangling (UNSTABLE):
   #[link_name = "\u{1}_Z28ImGui_ImplSDL3_InitForSDLGPUP10SDL_Window"]
   pub fn ImGui_ImplSDL3_InitForSDLGPU(window: *mut SDL_Window) -> bool;
   ```

2. **Compiler-Specific** - Name mangling differs between:
   - GCC vs Clang vs MSVC
   - Different compiler versions
   - Different platforms (Linux, macOS, Windows)

3. **No ABI Stability** - C++ has no standard ABI, breaking compatibility across compilers

### ✅ Solution: C Wrappers with Stable ABI:

```rust
// New bindings with C ABI (STABLE):
unsafe extern "C" {
    pub fn cImGui_ImplSDL3_InitForSDLGPU(window: *mut SDL_Window) -> bool;
}
```

## Implementation

### Files:

- `cimgui_impl_sdl3.h` - C header for SDL3 backend wrapper
- `cimgui_impl_sdl3.cpp` - C++ implementation that wraps the C++ backend
- `cimgui_impl_sdlgpu3.h` - C header for SDL_GPU backend wrapper
- `cimgui_impl_sdlgpu3.cpp` - C++ implementation that wraps the C++ backend

### Build Process:

1. **Compile C++ Wrappers** - CMake compiles the C++ wrapper files as part of the imgui library
2. **Generate Rust Bindings** - `bindgen` generates Rust bindings from the C headers in `build.rs`
3. **Link** - The wrappers are included in the imgui library built by CMake

### Function Naming:

C wrapper functions are prefixed with `cImGui_` to distinguish them from the original C++ functions:

- `ImGui_ImplSDL3_InitForSDLGPU()` → `cImGui_ImplSDL3_InitForSDLGPU()`
- `ImGui_ImplSDLGPU3_Init()` → `cImGui_ImplSDLGPU3_Init()`

## Benefits

- ✅ **Stable ABI** - Works across all compilers and platforms
- ✅ **No Name Mangling** - Clean C ABI with `extern "C"`
- ✅ **Cross-Compiler** - GCC, Clang, MSVC all work
- ✅ **Future-Proof** - Compiler updates won't break compatibility

## Technical Details

### Enum Handling:

SDL_GPU enums are used directly in the C wrapper for better type safety and ergonomics:

```c
typedef struct {
    SDL_GPUDevice* Device;
    SDL_GPUTextureFormat ColorTargetFormat;
    SDL_GPUSampleCount MSAASamples;
    SDL_GPUSwapchainComposition SwapchainComposition;
    SDL_GPUPresentMode PresentMode;
} cImGui_ImplSDLGPU3_InitInfo;
```

This ensures:
- Type safety - can't accidentally pass wrong enum values
- Better IDE support and autocomplete
- Direct compatibility with SDL3 GPU types
- C ABI compatibility (C enums have well-defined ABI)
