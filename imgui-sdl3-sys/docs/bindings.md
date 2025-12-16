# ImGui-sys Bindings Generation

This document explains how the Rust bindings for Dear ImGui (via dcimgui) were set up and the issues encountered during the process.

## Overview

The `imgui-sys` crate provides low-level Rust bindings to the Dear ImGui C library through its C bindings (dcimgui/cimgui). These bindings are automatically generated using [rust-bindgen](https://github.com/rust-lang/rust-bindgen) during the build process.

## Initial Setup

### Dependencies Added

1. **Cargo.toml changes**:
   - Added `bindgen = "0.72"` as an optional build dependency
   - Added `dep:bindgen` to the `build-from-source` feature

2. **Build process**:
   - The bindings are generated after the C library is built via CMake
   - Source headers are located in `imgui-src/dcimgui/`
   - The main header processed is `cimgui_all.h`, which includes:
     - `cimgui.h` - Public C API for Dear ImGui
     - `cimgui_internal.h` - Internal API (blocked from bindings)

## Bindgen Configuration

The bindgen configuration in `build.rs` is set up as follows:

```rust
let bindings = bindgen::Builder::default()
    .header(header_path.to_str().unwrap())
    .clang_arg(format!("-I{}", header_dir.display()))
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .allowlist_file(".*cimgui.*\\.h")
    .allowlist_file(".*imgui\\.h")
    .allowlist_file(".*imconfig\\.h")
    .blocklist_file(".*imgui_internal\\.h")
    .blocklist_file(".*cimgui_internal\\.h")
    .blocklist_file(".*imstb.*\\.h")
    .layout_tests(false)
    .use_core()
    .opaque_type("ImDrawListSharedData_t")
    .opaque_type("ImFontAtlasBuilder_t")
    .opaque_type("ImFontLoader_t")
    .opaque_type("ImGuiContext_t")
    .raw_line("#![allow(non_upper_case_globals)]")
    .raw_line("#![allow(non_camel_case_types)]")
    .raw_line("#![allow(non_snake_case)]")
    .raw_line("#![allow(dead_code)]")
    .raw_line("#![allow(clippy::all)]")
    .generate()?;
```

### Key Configuration Points

- **`use_core()`**: Generates `no_std` compatible bindings
- **`allowlist_file()`**: Only include definitions from cimgui public headers
- **`blocklist_file()`**: Exclude internal headers and stb headers
- **`layout_tests(false)`**: Disable layout tests to reduce generated code size
- **`raw_line()`**: Add lint suppression attributes for generated code

## Issues Encountered and Solutions

### Issue 1: Missing Config File (lib_name mismatch)

**Error:**
```
Error: "cmake dir not found in /home/michael/dev/rust/sdl3-sys-rs/target/debug/build/imgui-sys-7108cf370dcfdb1a/out"
```

**Root Cause:**
The `config.txt` file was copy-pasted from `sdl3-ttf-sys` and contained:
- `lib_name: SDL3_ttf` (incorrect)
- The build system was looking for `SDL3_ttfConfig.cmake`
- But CMake was generating `imguiConfig.cmake`

**Solution:**
Updated `imgui-sys/config.txt` with correct values:
```
package_name: imgui
lib_name: imgui
lib_rev_name: imgui
lib_min_version: 1.0.0
lib_dir: imgui
include_dir: include/dcimgui
version_header: imgui.h
headers_prefix: Im
sym_prefix: Im
define_prefix: IMGUI_
hint_prop_prefix: IMGUI_
```

The `find_and_output_cmake_dir_metadata()` function in `build-common.rs` looks for `{lib_name}Config.cmake`, so the names must match.

### Issue 2: Missing Opaque Type Definitions

**Error:**
```
error[E0412]: cannot find type `ImDrawListSharedData_t` in this scope
error[E0412]: cannot find type `ImFontAtlasBuilder_t` in this scope
error[E0412]: cannot find type `ImFontLoader_t` in this scope
error[E0412]: cannot find type `ImGuiContext_t` in this scope
```

**Root Cause:**
Some types are forward-declared in the public headers but not fully defined (opaque types). Bindgen generates type aliases like:
```rust
pub type ImGuiContext = ImGuiContext_t;
```

But the underlying `ImGuiContext_t` struct is not defined in the public headers (it's in `imgui_internal.h` which we blocked).

**Solution:**
Added post-processing step in `build.rs` to inject opaque struct definitions:

```rust
let opaque_types = vec![
    "ImDrawListSharedData_t",
    "ImFontAtlasBuilder_t",
    "ImFontLoader_t",
    "ImGuiContext_t",
];

let mut opaque_defs = String::new();
for opaque_type in opaque_types {
    if content.contains(opaque_type) {
        opaque_defs.push_str(&format!(
            "#[repr(C)]\n#[derive(Debug, Copy, Clone)]\npub struct {} {{\n    _unused: [u8; 0],\n}}\n",
            opaque_type
        ));
    }
}
```

These are zero-sized opaque structs that allow the type aliases to compile while maintaining type safety (you can't accidentally access their internals).

### Issue 3: Inner Attributes in Wrong Location

**Error:**
```
error: an inner attribute is not permitted in this context
```

**Root Cause:**
The initial post-processing logic inserted opaque type definitions before the `#![allow(...)]` attributes, which must be at the very top of the file.

**Solution:**
Fixed the post-processing logic to:
1. Keep all `#![allow(...)]` lines at the top
2. Insert opaque type definitions immediately after the `#![allow(...)]` section
3. Continue with the rest of the generated code

```rust
for line in lines {
    if !allow_section_done {
        if line.starts_with("#![allow") {
            result.push_str(line);
            result.push('\n');
            continue;
        } else if !line.is_empty() && !line.starts_with("/*") {
            allow_section_done = true;
            if !inserted && !opaque_defs.is_empty() {
                result.push_str(&opaque_defs);
                result.push('\n');
                inserted = true;
            }
        }
    }
    result.push_str(line);
    result.push('\n');
}
```

## Final Result

### Generated Bindings Statistics

- **File**: `src/generated/mod.rs`
- **Size**: 205 KB (6,082 lines)
- **Functions**: 491 ImGui functions (all prefixed with `ig`)
- **Types**: All ImGui structs, enums, and type aliases
- **Constants**: All ImGui constants and defines

### Build Status

✅ Builds successfully with `cargo build --package imgui-sys --features build-from-source-static`

The bindings compile with only minor warnings about unnecessary transmutes (which can be safely ignored or fixed later).

### Example Functions Available

The bindings include all major ImGui functionality:

- **Context management**: `igCreateContext`, `igDestroyContext`, `igGetCurrentContext`
- **Frame management**: `igNewFrame`, `igEndFrame`, `igRender`
- **Windows**: `igBegin`, `igEnd`, `igIsWindowAppearing`, `igSetWindowPos`
- **Widgets**: `igButton`, `igCheckbox`, `igInputText`, `igSliderFloat`
- **Layout**: `igSameLine`, `igSpacing`, `igSeparator`
- **Drawing**: `igGetDrawData`, `igGetWindowDrawList`
- **Demo**: `igShowDemoWindow`, `igShowMetricsWindow`

## Usage Example

```rust
use imgui_sys::*;

unsafe {
    // Create context
    let ctx = igCreateContext(core::ptr::null_mut());
    igSetCurrentContext(ctx);

    // Setup style and IO
    let io = igGetIO();

    // Main loop
    igNewFrame();

    let mut show_demo = true;
    igShowDemoWindow(&mut show_demo as *mut bool);

    igRender();
    let draw_data = igGetDrawData();

    // Cleanup
    igDestroyContext(ctx);
}
```

## Maintenance

The bindings are regenerated automatically during each build when the `build-from-source` feature is enabled. If the cimgui headers are updated:

1. Update the source in `imgui-src/dcimgui/`
2. Rebuild with `cargo clean --package imgui-sys && cargo build --package imgui-sys --features build-from-source-static`
3. The bindings will be regenerated automatically

If new opaque types are introduced, add them to the `opaque_types` vector in the `generate_bindings()` function in `build.rs`.

## SDL3 Backend Integration

The SDL3 platform backend (`imgui_impl_sdl3`) and SDL3 GPU renderer backend (`imgui_impl_sdlgpu3`) have been integrated into `imgui-sys` to provide seamless integration with SDL3.

### Implementation Approach

We chose to use bindgen directly with the C++ header (`imgui_impl_sdl3.h`) **without creating an intermediate C API**. This works because:

1. The SDL3 backend header already uses C-compatible function signatures via `IMGUI_IMPL_API` (which expands to `extern "C"`)
2. Bindgen can handle C++ headers when configured with the `-xc++` flag
3. This avoids the overhead of maintaining a separate C wrapper

### Architecture

The bindings are generated in **three separate bindgen passes** to avoid type conflicts:

1. **Main ImGui Bindings** (`imgui.rs`)
   - Generated from `cimgui_all.h`
   - Contains all core ImGui types and functions
   - Exported at the crate root level

2. **SDL3 Platform Backend Bindings** (`sdl3_backend.rs`)
   - Generated from `imgui_impl_sdl3.h`
   - Contains only SDL3-specific platform backend functions (allowlisted: `ImGui_ImplSDL3_*`)
   - Handles input events, window management, and frame timing
   - Blocklists core ImGui types to prevent redefinition
   - Uses `use super::*;` to import core types from the parent module
   - Exported under the `imgui_sys::sdl3` module

3. **SDL3 GPU Renderer Backend Bindings** (`sdlgpu3_backend.rs`)
   - Generated from `imgui_impl_sdlgpu3.h`
   - Contains only SDL3 GPU-specific renderer backend functions (allowlisted: `ImGui_ImplSDLGPU3_*`)
   - Handles rendering ImGui draw data using SDL3's GPU API
   - Blocklists core ImGui types to prevent redefinition
   - Uses `use super::*;` to import core types from the parent module
   - Exported under the `imgui_sys::sdlgpu3` module

### Module Structure

```
imgui-sys/src/generated/
├── mod.rs              # Main module, re-exports imgui bindings
├── imgui.rs            # Core ImGui bindings (207KB)
├── sdl3_backend.rs     # SDL3 platform backend functions (2.9KB)
├── sdlgpu3_backend.rs  # SDL3 GPU renderer backend functions
├── sdl3/
│   └── mod.rs          # SDL3 platform module with type re-exports
└── sdlgpu3/
    └── mod.rs          # SDL3 GPU module with type re-exports
```

### SDL3 Type Handling

SDL3 types are **re-exported from `sdl3-sys`** in both backend modules:

#### Platform Backend Types (`imgui_sys::sdl3`)
```rust
// In imgui-sys/src/generated/sdl3/mod.rs
pub use sdl3_sys::everything::{SDL_Window, SDL_Renderer, SDL_Gamepad, SDL_Event};
```

#### GPU Backend Types (`imgui_sys::sdlgpu3`)
```rust
// In imgui-sys/src/generated/sdlgpu3/mod.rs
pub use sdl3_sys::everything::{
    SDL_GPUDevice,
    SDL_GPUTextureFormat,
    SDL_GPUSampleCount,
    SDL_GPUSwapchainComposition,
    SDL_GPUPresentMode,
    SDL_GPUCommandBuffer,
    SDL_GPURenderPass,
    SDL_GPUGraphicsPipeline,
    SDL_GPUSampler,
};
```

This means:
- **No type casting needed** - You can pass `sdl3_sys` types directly to ImGui backend functions
- **Full type compatibility** - The types are identical, not separate opaque wrappers
- **Seamless integration** - Works naturally with both `sdl3-sys` and `imgui-sys` APIs

### Available Functions

#### SDL3 Platform Backend Functions (11 total)

Available in `imgui_sys::sdl3`:

- `ImGui_ImplSDL3_InitForOpenGL()` - Initialize for OpenGL rendering
- `ImGui_ImplSDL3_InitForVulkan()` - Initialize for Vulkan rendering
- `ImGui_ImplSDL3_InitForD3D()` - Initialize for Direct3D rendering
- `ImGui_ImplSDL3_InitForMetal()` - Initialize for Metal rendering
- `ImGui_ImplSDL3_InitForSDLRenderer()` - Initialize for SDL's renderer
- `ImGui_ImplSDL3_InitForSDLGPU()` - Initialize for SDL's GPU API
- `ImGui_ImplSDL3_InitForOther()` - Initialize for other backends
- `ImGui_ImplSDL3_Shutdown()` - Cleanup resources
- `ImGui_ImplSDL3_NewFrame()` - Start a new ImGui frame
- `ImGui_ImplSDL3_ProcessEvent()` - Process SDL3 events
- `ImGui_ImplSDL3_SetGamepadMode()` - Configure gamepad handling

#### SDL3 GPU Renderer Backend Functions (8 total)

Available in `imgui_sys::sdlgpu3`:

- `ImGui_ImplSDLGPU3_Init()` - Initialize the GPU renderer with configuration
- `ImGui_ImplSDLGPU3_Shutdown()` - Cleanup GPU resources
- `ImGui_ImplSDLGPU3_NewFrame()` - Start a new frame for GPU rendering
- `ImGui_ImplSDLGPU3_PrepareDrawData()` - **MANDATORY**: Upload vertex/index buffers to GPU before rendering
- `ImGui_ImplSDLGPU3_RenderDrawData()` - Render ImGui draw data to a GPU render pass
- `ImGui_ImplSDLGPU3_CreateDeviceObjects()` - Create device-dependent objects
- `ImGui_ImplSDLGPU3_DestroyDeviceObjects()` - Destroy device-dependent objects
- `ImGui_ImplSDLGPU3_UpdateTexture()` - Update texture data manually (for advanced use)

**Important**: Unlike other renderer backends, you must call `ImGui_ImplSDLGPU3_PrepareDrawData()` before issuing a render pass with `ImGui_ImplSDLGPU3_RenderDrawData()`. This uploads the vertex and index buffers to the GPU.

### Usage Examples

#### SDL3 Platform Backend with OpenGL

```rust
use imgui_sys::sdl3::*;
use sdl3_sys::everything::*;

unsafe {
    // Create SDL3 window
    let window = SDL_CreateWindow(...);
    let gl_context = SDL_GL_CreateContext(window);

    // Initialize ImGui for SDL3 + OpenGL
    // No casting needed - types are compatible!
    ImGui_ImplSDL3_InitForOpenGL(window, gl_context);

    // Main loop
    loop {
        let mut event = SDL_Event { ... };
        while SDL_PollEvent(&mut event) != 0 {
            // Process events for ImGui
            ImGui_ImplSDL3_ProcessEvent(&event);
        }

        // Start new ImGui frame
        ImGui_ImplSDL3_NewFrame();

        // ... ImGui rendering code ...

        SDL_GL_SwapWindow(window);
    }

    // Cleanup
    ImGui_ImplSDL3_Shutdown();
    SDL_GL_DestroyContext(gl_context);
    SDL_DestroyWindow(window);
}
```

#### SDL3 Platform Backend with GPU Renderer

```rust
use imgui_sys::{*, sdl3::*, sdlgpu3::*};
use sdl3_sys::everything::*;
use core::ptr;

unsafe {
    // Create SDL3 GPU device and window
    let device = SDL_CreateGPUDevice(...);
    let window = SDL_CreateWindow(...);
    SDL_ClaimWindowForGPUDevice(device, window);

    // Initialize ImGui context
    let ctx = igCreateContext(ptr::null_mut());
    igSetCurrentContext(ctx);

    // Initialize SDL3 platform backend
    ImGui_ImplSDL3_InitForSDLGPU(window);

    // Get swapchain format
    let swapchain_format = SDL_GetGPUSwapchainTextureFormat(device, window);

    // Initialize SDL3 GPU renderer backend
    let mut init_info = ImGui_ImplSDLGPU3_InitInfo {
        Device: device,
        ColorTargetFormat: swapchain_format,
        MSAASamples: SDL_GPU_SAMPLECOUNT_1,
        SwapchainComposition: SDL_GPU_SWAPCHAINCOMPOSITION_SDR,
        PresentMode: SDL_GPU_PRESENTMODE_VSYNC,
    };
    ImGui_ImplSDLGPU3_Init(&mut init_info);

    // Main loop
    loop {
        let mut event = SDL_Event { ... };
        while SDL_PollEvent(&mut event) != 0 {
            ImGui_ImplSDL3_ProcessEvent(&event);
        }

        // Start new frame
        ImGui_ImplSDL3_NewFrame();
        ImGui_ImplSDLGPU3_NewFrame();
        igNewFrame();

        // ... ImGui UI code ...
        igShowDemoWindow(ptr::null_mut());

        // Rendering
        igRender();
        let draw_data = igGetDrawData();

        // Acquire swapchain texture
        let mut swapchain_texture: *mut SDL_GPUTexture = ptr::null_mut();
        let cmd_buffer = SDL_AcquireGPUCommandBuffer(device);
        SDL_AcquireGPUSwapchainTexture(
            cmd_buffer,
            window,
            &mut swapchain_texture,
            ptr::null_mut(),
            ptr::null_mut(),
        );

        if !swapchain_texture.is_null() {
            // MANDATORY: Prepare draw data before rendering
            ImGui_ImplSDLGPU3_PrepareDrawData(draw_data, cmd_buffer);

            // Create render pass
            let color_target = SDL_GPUColorTargetInfo {
                texture: swapchain_texture,
                load_op: SDL_GPU_LOADOP_CLEAR,
                store_op: SDL_GPU_STOREOP_STORE,
                clear_color: SDL_FColor { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
                // ... other fields
            };
            let render_pass = SDL_BeginGPURenderPass(
                cmd_buffer,
                &color_target,
                1,
                ptr::null(),
            );

            // Render ImGui
            ImGui_ImplSDLGPU3_RenderDrawData(draw_data, cmd_buffer, render_pass, ptr::null_mut());

            SDL_EndGPURenderPass(render_pass);
        }

        // Submit and present
        SDL_SubmitGPUCommandBuffer(cmd_buffer);
    }

    // Cleanup
    ImGui_ImplSDLGPU3_Shutdown();
    ImGui_ImplSDL3_Shutdown();
    igDestroyContext(ctx);
    SDL_DestroyWindow(window);
    SDL_DestroyGPUDevice(device);
}
```

### Issues Encountered and Solutions

#### Issue 4: CMake Build Configuration Mismatch

**Error:**
```
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

**Root Cause:**
The CMakeLists.txt was building a static library by default (`imgui-static` target), but the build script's `build-common.rs` was trying to copy a dynamic library (`.dylib` on macOS) because the `link-static` feature wasn't enabled. The `read_link()` call failed because no symlink existed.

**Solution:**
Updated the `build-from-source` feature in `Cargo.toml` to always enable `link-static`:

```toml
build-from-source = ["dep:cmake", "dep:rpkg-config", "dep:imgui-src",
                     "dep:bindgen", "sdl3-sys/build-from-source", "link-static"]
```

This ensures the Rust build configuration matches what CMake produces.

#### Issue 5: Type Definition Conflicts

**Error:**
```
error: definition of type 'ImDrawChannel' conflicts with typedef of the same name
/imgui-src/dcimgui/cimgui.h:211:32: note: 'ImDrawChannel' declared here
```

**Root Cause:**
Including both `cimgui_all.h` and `imgui_impl_sdl3.h` in a single bindgen pass caused duplicate type definitions. Both headers include `imgui.h`, leading to conflicts for ~20 types.

**Solution:**
Split the binding generation into two separate bindgen invocations:
1. First pass: Generate core ImGui bindings from `cimgui_all.h`
2. Second pass: Generate SDL3 backend bindings from `imgui_impl_sdl3.h` with:
   - `allowlist_function("ImGui_ImplSDL3_.*")` - Only include SDL3 backend functions
   - `allowlist_type("ImGui_ImplSDL3_.*")` - Only include SDL3 backend types
   - `blocklist_type("ImGui[^_].*")` - Block core ImGui types (regex excludes `ImGui_ImplSDL3_*`)
   - Additional blocklists for `ImVec.*`, `ImColor`, `ImFont.*`, etc.

#### Issue 6: SDL3 Type Resolution

**Error:**
```
error[E0412]: cannot find type `SDL_Event` in this scope
error[E0432]: unresolved imports `sdl3_sys::SDL_Event`, `sdl3_sys::SDL_Window`, ...
```

**Root Cause:**
SDL3 types were marked as opaque in bindgen (`opaque_type("SDL_Event")`), but the generated code couldn't find them. Initially tried to import from `sdl3-sys` at the crate root, but these types aren't exported there - they're in module-specific files.

**Solution:**
Re-export SDL3 types from `sdl3_sys::everything` module in the `sdl3/mod.rs`:

```rust
pub use sdl3_sys::everything::{SDL_Window, SDL_Renderer, SDL_Gamepad, SDL_Event};
```

The `everything` module in `sdl3-sys` re-exports all types from all submodules, providing a convenient way to access any SDL3 type. This means:
- Users get the **actual `sdl3_sys` types**, not opaque wrappers
- **No casting needed** between `imgui-sys` and `sdl3-sys`
- **Full type safety** and seamless integration

#### Issue 7: Inner Attribute Placement

**Error:**
```
error: an inner attribute is not permitted in this context
```

**Root Cause:**
The `#![allow(...)]` directives were being placed after the `use super::*;` statement in the SDL3 backend bindings. Inner attributes must be at the very top of the file, before any use statements.

**Solution:**
Reordered the `raw_line()` calls in bindgen:

```rust
.raw_line("#![allow(non_upper_case_globals)]")
.raw_line("#![allow(non_camel_case_types)]")
.raw_line("#![allow(non_snake_case)]")
.raw_line("#![allow(dead_code)]")
.raw_line("#![allow(clippy::all)]")
.raw_line("use super::*;")  // Must come AFTER #![allow(...)] lines
```

#### Issue 8: Rust 2024 Edition Safety Warnings

**Warning:**
```
warning[E0133]: call to unsafe function is unsafe and requires unsafe block
note: consult the function's documentation for information on how to avoid undefined behavior
```

**Root Cause:**
Bindgen-generated code in edition 2024 triggers `unsafe_op_in_unsafe_fn` warnings for code that performs unsafe operations inside unsafe functions.

**Solution:**
Added `#![allow(unsafe_op_in_unsafe_fn)]` to the generated bindings. This is appropriate for FFI code where unsafe operations inside unsafe functions are expected and necessary.

### Build Process

The SDL3 backend is built and linked automatically when using the `build-from-source` feature:

```bash
cargo build -p imgui-sys --features build-from-source
```

The CMake build:
1. Compiles `imgui_impl_sdl3.cpp` (added to sources in CMakeLists.txt)
2. Links against SDL3 (found via `SDL3Config.cmake` from `sdl3-sys`)
3. Includes the `backends/` directory in the build
4. Installs headers (but not backend headers, only main headers)
5. Bindgen generates Rust bindings from the source `imgui_impl_sdl3.h`

#### Issue 9: SDL3 GPU Backend - Missing SDL3 Headers

**Error:**
```
fatal error: 'SDL3/SDL_gpu.h' file not found
```

**Root Cause:**
When generating bindings for `imgui_impl_sdlgpu3.h`, bindgen couldn't find the SDL3 headers because the SDL3 include path wasn't added to the clang arguments.

**Solution:**
Implement a cross-platform SDL3 header search that works on Windows, Linux, and macOS. The search strategy:

1. First checks if `DEP_SDL3_INCLUDE` environment variable is set (some sdl3-sys versions may provide this)
2. Falls back to searching relative to `DEP_SDL3_CMAKE_DIR` by walking up the directory tree (up to 4 levels)
3. Checks for headers in both `<dir>/include/SDL3/` and `<dir>/SDL3/` patterns

This handles different directory structures:
- **Windows build-from-source**: `<out>/cmake` → `<out>/include/SDL3/`
- **Linux/macOS build-from-source**: `<out>/lib/cmake/SDL3` → `<out>/include/SDL3/`
- **System installs**: `/usr/lib/cmake/SDL3` → `/usr/include/SDL3/`

```rust
let sdl3_include_dir = {
    if let Some(include_path) = env::var_os("DEP_SDL3_INCLUDE") {
        let path = PathBuf::from(include_path);
        if path.join("SDL3").join("SDL_gpu.h").exists() {
            Some(path)
        } else if path.join("SDL_gpu.h").exists() {
            path.parent().map(|p| p.to_path_buf())
        } else {
            None
        }
    } else if let Some(sdl3_cmake_dir) = env::var_os("DEP_SDL3_CMAKE_DIR") {
        fn find_sdl3_headers(start_path: &std::path::Path) -> Option<PathBuf> {
            let mut current = start_path;
            for _ in 0..4 {
                let candidate = current.join("include");
                if candidate.join("SDL3").join("SDL_gpu.h").exists() {
                    return Some(candidate);
                }
                if current.join("SDL3").join("SDL_gpu.h").exists() {
                    return Some(current.to_path_buf());
                }
                current = current.parent()?;
            }
            None
        }
        find_sdl3_headers(&PathBuf::from(sdl3_cmake_dir))
    } else {
        None
    }
};

if let Some(ref sdl3_inc) = sdl3_include_dir {
    sdlgpu3_builder = sdlgpu3_builder.clang_arg(format!("-I{}", sdl3_inc.display()));
}
```

#### Issue 10: Windows MSVC Static Library Naming

**Error:**
```
error: could not find native static library `imgui`, perhaps an -L flag is missing?
```

**Root Cause:**
On Windows with MSVC, CMake automatically appends `-static` suffix to static library filenames (e.g., `imgui-static.lib` instead of `imgui.lib`). The `build-common.rs` code had logic to handle this when using pkgconfig, but on Windows no pkgconfig file is generated by CMake, so it fell through to the fallback code path which didn't account for the MSVC naming convention.

**Solution:**
Added MSVC-specific static library naming logic in the fallback path at [build-common.rs:369-376](../build-common.rs#L369-L376):

```rust
match link_kind {
    LinkKind::Static => {
        // MSVC appends -static suffix to static library names
        if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
            link_flags.link_static_lib(format!("{lib_name}-static"));
        } else {
            link_flags.link_static_lib(lib_name);
        }
    }
    LinkKind::Default => link_flags.link_lib(lib_name),
}
```

This ensures the correct library name is used on Windows MSVC builds while maintaining compatibility with Linux/macOS builds that don't use the `-static` suffix.

#### Issue 11: SDL3 GPU Types Don't Implement Debug

**Error:**
```
error[E0277]: `sdl3_sys::gpu::SDL_GPUTextureFormat` doesn't implement `Debug`
```

**Root Cause:**
The SDL3 GPU enum types in `sdl3-sys` don't derive `Debug`, so bindgen's default behavior of deriving `Debug` on all structs fails when those structs contain SDL GPU types.

**Solution:**
Disable the `Debug` derive for the GPU backend bindings:

```rust
.derive_debug(false)  // Don't derive Debug since SDL GPU types don't implement it
```

#### Issue 12: Type Conflicts with ImDrawData and ImTextureData

**Error:**
```
error[E0412]: cannot find type `ImDrawData` in this scope
error[E0412]: cannot find type `ImTextureData` in this scope
```

**Root Cause:**
Initially blocked `ImDrawData` and `ImTexture.*` types, but the GPU backend functions use these types in their signatures.

**Solution:**
Removed `ImDrawData` and `ImTexture.*` from the blocklist. Only block types that are truly not needed and would cause conflicts.

#### Issue 13: Windows MSVC C++ Standard Library Linking

**Error:**
```
LINK : fatal error LNK1181: cannot open input file 'stdc++.lib'
```

**Root Cause:**
The build script was unconditionally linking `stdc++` (the GNU C++ standard library) on all non-Apple platforms. However, on Windows MSVC, the C++ standard library is automatically linked by the MSVC toolchain and there is no `stdc++.lib` file. MSVC uses a different C++ standard library implementation that's implicitly linked.

**Solution:**
Modified the C++ standard library linking logic in [build.rs:40-45](../build.rs#L40-L45) to exclude Windows MSVC:

```rust
// ImGui is C++ code, so we need to link the C++ standard library
// On Windows MSVC, the C++ standard library is linked automatically by the toolchain
#[cfg(target_vendor = "apple")]
println!("cargo::rustc-link-lib=c++");
#[cfg(all(not(target_vendor = "apple"), not(target_env = "msvc")))]
println!("cargo::rustc-link-lib=stdc++");
```

This ensures:
- **macOS**: Links `libc++` (Clang's C++ standard library)
- **Linux/Unix**: Links `libstdc++` (GNU C++ standard library)
- **Windows MSVC**: No explicit linking (uses MSVC's automatic C++ standard library linking)

### Testing

To verify the bindings and type compatibility:

```rust
#[test]
fn test_sdl3_backend_available() {
    // Check that the platform backend functions exist
    let _ = imgui_sys::sdl3::ImGui_ImplSDL3_Shutdown as unsafe extern "C" fn();
    let _ = imgui_sys::sdl3::ImGui_ImplSDL3_NewFrame as unsafe extern "C" fn();
}

#[test]
fn test_sdl3_types_are_compatible() {
    // Verify that SDL3 types from imgui-sys and sdl3-sys are the same
    use sdl3_sys::everything::SDL_Window as SdlWindow;
    use imgui_sys::sdl3::SDL_Window as ImGuiWindow;

    // This compiles only if the types are identical
    let _: fn(*mut SdlWindow) = |w: *mut ImGuiWindow| {
        let _: *mut SdlWindow = w;
    };
}

#[test]
fn test_sdlgpu3_backend_available() {
    // Check that the GPU backend functions exist
    let _ = imgui_sys::sdlgpu3::ImGui_ImplSDLGPU3_Init as unsafe extern "C" fn(*mut imgui_sys::sdlgpu3::ImGui_ImplSDLGPU3_InitInfo) -> bool;
    let _ = imgui_sys::sdlgpu3::ImGui_ImplSDLGPU3_Shutdown as unsafe extern "C" fn();
    let _ = imgui_sys::sdlgpu3::ImGui_ImplSDLGPU3_PrepareDrawData as unsafe extern "C" fn(*mut imgui_sys::ImDrawData, *mut imgui_sys::sdlgpu3::SDL_GPUCommandBuffer);
}

#[test]
fn test_sdlgpu3_types_are_compatible() {
    // Verify that SDL3 GPU types from imgui-sys and sdl3-sys are the same
    use sdl3_sys::everything::SDL_GPUDevice as SdlDevice;
    use imgui_sys::sdlgpu3::SDL_GPUDevice as ImGuiDevice;

    // This compiles only if the types are identical
    let _: fn(*mut SdlDevice) = |d: *mut ImGuiDevice| {
        let _: *mut SdlDevice = d;
    };
}
```

### Generated Code Statistics

#### SDL3 Platform Backend
- **File**: `sdl3_backend.rs`
- **Size**: ~2.9 KB
- **Functions**: 11 SDL3 platform backend functions
- **Types**: 1 enum (`ImGui_ImplSDL3_GamepadMode`) + opaque SDL3 types
- **Total lines**: ~100 lines

#### SDL3 GPU Renderer Backend
- **File**: `sdlgpu3_backend.rs`
- **Size**: ~4 KB
- **Functions**: 8 SDL3 GPU renderer backend functions
- **Types**: 2 structs (`ImGui_ImplSDLGPU3_InitInfo`, `ImGui_ImplSDLGPU3_RenderState`) + opaque SDL3 GPU types
- **Total lines**: ~130 lines

## Platform Compatibility

The build system has been designed and tested to work across multiple platforms:

### Supported Platforms

| Platform | Compiler | Status | Notes |
|----------|----------|--------|-------|
| **Windows** | MSVC | ✅ Tested | Static library naming handled automatically |
| **Linux** | GCC/Clang | ✅ Should work | Standard library naming |
| **macOS** | Clang | ✅ Should work | Standard library naming |

### Cross-Platform Considerations

1. **SDL3 Header Search**: The build script searches for SDL3 headers in multiple locations to handle different installation layouts:
   - Windows build-from-source: `<out>/cmake` → `<out>/include/SDL3/`
   - Linux/macOS build-from-source: `<out>/lib/cmake/SDL3` → `<out>/include/SDL3/`
   - System installs: `/usr/lib/cmake/SDL3` → `/usr/include/SDL3/`

2. **Library Naming**:
   - Windows MSVC uses `imgui-static.lib` for static libraries
   - Linux/macOS use `libimgui.a` for static libraries
   - The build system automatically handles these differences

3. **C++ Standard Library**:
   - macOS links against `libc++` (Clang's C++ stdlib)
   - Linux/Unix link against `libstdc++` (GNU C++ stdlib)
   - Windows MSVC uses automatic C++ stdlib linking (no explicit link flag needed)

## References

- [Dear ImGui](https://github.com/ocornut/imgui) - Original C++ library
- [cimgui](https://github.com/cimgui/cimgui) - C bindings for Dear ImGui
- [dcimgui](https://github.com/cimgui/dcimgui) - Dear Bindings variant of cimgui
- [rust-bindgen](https://github.com/rust-lang/rust-bindgen) - Rust FFI bindings generator
- [imgui_impl_sdl3.h](https://github.com/ocornut/imgui/blob/master/backends/imgui_impl_sdl3.h) - SDL3 platform backend
- [imgui_impl_sdlgpu3.h](https://github.com/ocornut/imgui/blob/master/backends/imgui_impl_sdlgpu3.h) - SDL3 GPU renderer backend
