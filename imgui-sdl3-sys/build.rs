#[cfg(feature = "build-from-source")]
const SOURCE_DIR: &str = imgui_src::SOURCE_DIR;

const LINK_FRAMEWORK: bool = cfg!(feature = "link-framework");

include!("build-common.rs");

fn main() -> Result<(), Box<dyn Error>> {
    build(|config| {
        let _ = config;
        #[cfg(feature = "build-from-source")]
        {
            if let Some(sdl3_cmake_dir) = env::var_os("DEP_SDL3_CMAKE_DIR") {
                config.define("SDL3_DIR", sdl3_cmake_dir);
            }

            if LINK_FRAMEWORK {
                // !!!FIXME
                panic!(
                    "imgui is currently missing a configuration option to build as a framework."
                );
            } else if cfg!(feature = "link-static") {
                config.define("BUILD_SHARED_LIBS", "OFF");
            } else {
                // Build shared library
                config.define("BUILD_SHARED_LIBS", "ON");
            }

            config.define("SDLTTF_SAMPLES", "OFF");

            // cmake_vars! { config =>
            //     SDLTTF_VENDORED,
            //     SDLTTF_HARFBUZZ,
            //     SDLTTF_PLUTOSVG,
            // }
        }
        Ok(())
    })?;

    // ImGui is C++ code, so we need to link the C++ standard library
    #[cfg(target_vendor = "apple")]
    println!("cargo::rustc-link-lib=c++");
    #[cfg(not(target_vendor = "apple"))]
    println!("cargo::rustc-link-lib=stdc++");

    #[cfg(feature = "build-from-source")]
    generate_bindings()?;

    Ok(())
}

#[cfg(feature = "build-from-source")]
fn generate_bindings() -> Result<(), Box<dyn Error>> {
    use std::path::PathBuf;

    eprintln!("generate_bindings: START");
    let _out_dir = env::var("OUT_DIR")?;
    let header_dir = PathBuf::from(SOURCE_DIR);
    let header_path = header_dir.join("cimgui_all.h");
    let backends_dir = header_dir.join("backends");
    let sdl3_backend_path = backends_dir.join("imgui_impl_sdl3.h");
    let sdlgpu3_backend_path = backends_dir.join("imgui_impl_sdlgpu3.h");

    println!("cargo::rerun-if-changed={}", header_path.display());
    println!("cargo::rerun-if-changed={}", sdl3_backend_path.display());
    println!("cargo::rerun-if-changed={}", sdlgpu3_backend_path.display());

    eprintln!("Header path: {}", header_path.display());
    eprintln!("Header exists: {}", header_path.exists());
    eprintln!("SDL3 backend path: {}", sdl3_backend_path.display());
    eprintln!("SDL3 backend exists: {}", sdl3_backend_path.exists());
    eprintln!("SDL3 GPU backend path: {}", sdlgpu3_backend_path.display());
    eprintln!("SDL3 GPU backend exists: {}", sdlgpu3_backend_path.exists());

    eprintln!("generate_bindings: Creating bindgen builder");
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .clang_arg(format!("-I{}", header_dir.display()))
        .clang_arg(format!("-I{}", backends_dir.display()))
        .clang_arg("-xc++")
        .clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS")
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
        .raw_line("#![allow(unsafe_op_in_unsafe_fn)]")
        .raw_line("#![allow(unnecessary_transmutes)]")
        .generate()?;

    eprintln!("generate_bindings: Generating SDL3 backend bindings from C wrapper");
    // C wrapper is now built by CMake in imgui-src, just generate bindings from the header
    let sdl3_wrapper_header = backends_dir.join("cimgui_impl_sdl3.h");
    let sdl3_bindings = bindgen::Builder::default()
        .header(sdl3_wrapper_header.to_str().unwrap())
        .clang_arg(format!("-I{}", backends_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("cImGui_ImplSDL3_.*")
        .allowlist_type("cImGui_ImplSDL3_.*")
        .allowlist_var("cImGui_ImplSDL3_.*")
        .opaque_type("SDL_Window")
        .opaque_type("SDL_Renderer")
        .opaque_type("SDL_Gamepad")
        .opaque_type("SDL_Event")
        .use_core()
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(clippy::all)]")
        .raw_line("#![allow(unnecessary_transmutes)]")
        .raw_line("use super::*;")
        .generate()?;

    eprintln!("generate_bindings: Generating SDL3 GPU backend bindings from C wrapper");
    // C wrapper is now built by CMake in imgui-src, just generate bindings from the header

    // Get SDL3 include directory for bindgen
    // Try multiple methods to find SDL3 headers for maximum cross-platform compatibility
    let sdl3_include_dir = {
        // Method 1: Check if DEP_SDL3_INCLUDE is set (some sdl3-sys versions might provide this)
        if let Some(include_path) = env::var_os("DEP_SDL3_INCLUDE") {
            let path = PathBuf::from(include_path);
            if path.join("SDL3").join("SDL_gpu.h").exists() {
                eprintln!("Found SDL3 headers via DEP_SDL3_INCLUDE: {}", path.display());
                Some(path)
            } else if path.join("SDL_gpu.h").exists() {
                eprintln!("Found SDL3 headers via DEP_SDL3_INCLUDE (parent): {}", path.parent().unwrap().display());
                path.parent().map(|p| p.to_path_buf())
            } else {
                None
            }
        } else if let Some(sdl3_cmake_dir) = env::var_os("DEP_SDL3_CMAKE_DIR") {
            // Method 2: Search relative to cmake directory
            let cmake_path = PathBuf::from(&sdl3_cmake_dir);
            eprintln!("DEP_SDL3_CMAKE_DIR: {}", cmake_path.display());

            // Search for SDL3 headers by walking up the directory tree and checking various locations
            // Common patterns:
            // - Windows/build-from-source: <out>/cmake -> <out>/include/SDL3/
            // - Linux/macOS build-from-source: <out>/lib/cmake/SDL3 -> <out>/include/SDL3/
            // - System installs: /usr/lib/cmake/SDL3 -> /usr/include/SDL3/

            fn find_sdl3_headers(start_path: &std::path::Path) -> Option<PathBuf> {
                let mut current = start_path;

                // Try up to 4 levels up from the cmake directory
                for level in 0..4 {
                    eprintln!("  Searching level {}: {}", level, current.display());

                    // Check <current>/include/SDL3/SDL_gpu.h
                    let candidate = current.join("include");
                    if candidate.join("SDL3").join("SDL_gpu.h").exists() {
                        eprintln!("Found SDL3 headers at: {}", candidate.display());
                        return Some(candidate);
                    }

                    // Check <current>/SDL3/SDL_gpu.h (for some system installs)
                    if current.join("SDL3").join("SDL_gpu.h").exists() {
                        eprintln!("Found SDL3 headers at: {}", current.display());
                        return Some(current.to_path_buf());
                    }

                    // Move up one directory
                    current = current.parent()?;
                }

                None
            }

            find_sdl3_headers(&cmake_path)
        } else {
            eprintln!("Neither DEP_SDL3_INCLUDE nor DEP_SDL3_CMAKE_DIR is set");
            None
        }
    };

    let sdlgpu3_wrapper_header = backends_dir.join("cimgui_impl_sdlgpu3.h");
    let mut sdlgpu3_builder = bindgen::Builder::default()
        .header(sdlgpu3_wrapper_header.to_str().unwrap())
        .clang_arg(format!("-I{}", backends_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("cImGui_ImplSDLGPU3_.*")
        .allowlist_type("cImGui_ImplSDLGPU3_.*")
        .allowlist_var("cImGui_ImplSDLGPU3_.*")
        .opaque_type("SDL_GPUDevice")
        .opaque_type("SDL_GPUCommandBuffer")
        .opaque_type("SDL_GPURenderPass")
        .opaque_type("SDL_GPUGraphicsPipeline")
        .opaque_type("SDL_GPUSampler")
        .opaque_type("SDL_GPUTextureFormat")
        .opaque_type("SDL_GPUSampleCount")
        .opaque_type("SDL_GPUSwapchainComposition")
        .opaque_type("SDL_GPUPresentMode")
        .opaque_type("ImDrawData")
        .opaque_type("ImTextureData")
        .use_core()
        .derive_debug(false)  // Don't derive Debug since SDL GPU types don't implement it
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(clippy::all)]")
        .raw_line("#![allow(unsafe_op_in_unsafe_fn)]")
        .raw_line("#![allow(unnecessary_transmutes)]")
        .raw_line("use super::*;");

    // Add SDL3 include directory if available
    if let Some(ref sdl3_inc) = sdl3_include_dir {
        eprintln!("Adding SDL3 include directory to bindgen: {}", sdl3_inc.display());
        sdlgpu3_builder = sdlgpu3_builder.clang_arg(format!("-I{}", sdl3_inc.display()));
    } else {
        eprintln!("WARNING: SDL3 include directory not found. SDL GPU backend bindings may fail.");
        eprintln!("This usually means SDL3 headers are not installed or DEP_SDL3_CMAKE_DIR is not set correctly.");
    }

    let sdlgpu3_bindings = sdlgpu3_builder.generate()?;

    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let bindings_dir = PathBuf::from(manifest_dir).join("src/bindings");
    std::fs::create_dir_all(&bindings_dir)?;

    // Write main imgui bindings
    let output_path = bindings_dir.join("imgui.rs");
    bindings.write_to_file(&output_path)?;

    // Write SDL3 backend bindings
    let sdl3_output_path = bindings_dir.join("sdl3_backend.rs");
    sdl3_bindings.write_to_file(&sdl3_output_path)?;

    // Write SDL3 GPU backend bindings
    let sdlgpu3_output_path = bindings_dir.join("sdlgpu3_backend.rs");
    sdlgpu3_bindings.write_to_file(&sdlgpu3_output_path)?;

    // Post-process: Add missing opaque type definitions
    let content = std::fs::read_to_string(&output_path)?;
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

    // Insert opaque definitions after #![allow...] lines
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();
    let mut allow_section_done = false;
    let mut inserted = false;

    for line in lines {
        // Skip lines until we're past the #![allow...] section
        if !allow_section_done {
            if line.starts_with("#![allow") {
                result.push_str(line);
                result.push('\n');
                continue;
            } else if !line.is_empty() && !line.starts_with("/*") {
                // We've found a non-allow, non-empty, non-comment line
                allow_section_done = true;
                if !inserted && !opaque_defs.is_empty() {
                    // Insert opaque definitions before this line
                    result.push_str(&opaque_defs);
                    result.push('\n');
                    inserted = true;
                }
            }
        }
        result.push_str(line);
        result.push('\n');
    }

    std::fs::write(&output_path, result)?;

    Ok(())
}
