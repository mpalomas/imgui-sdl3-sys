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
        .raw_line("use super::*;")
        .generate()?;

    eprintln!("generate_bindings: Generating SDL3 GPU backend bindings from C wrapper");
    // C wrapper is now built by CMake in imgui-src, just generate bindings from the header

    // Get SDL3 include directory for bindgen
    let sdl3_include_dir = if let Some(sdl3_cmake_dir) = env::var_os("DEP_SDL3_CMAKE_DIR") {
        PathBuf::from(sdl3_cmake_dir)
            .parent()  // Remove SDL3 -> .../lib/cmake
            .and_then(|p| p.parent())  // Remove cmake -> .../lib
            .and_then(|p| p.parent())  // Remove lib -> .../out
            .map(|p| p.join("include"))  // Add include -> .../out/include
    } else {
        None
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
        .raw_line("use super::*;");

    // Add SDL3 include directory if available
    if let Some(ref sdl3_inc) = sdl3_include_dir {
        sdlgpu3_builder = sdlgpu3_builder.clang_arg(format!("-I{}", sdl3_inc.display()));
    }

    let sdlgpu3_bindings = sdlgpu3_builder.generate()?;

    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let generated_dir = PathBuf::from(manifest_dir).join("src/generated");
    std::fs::create_dir_all(&generated_dir)?;

    // Write main imgui bindings
    let output_path = generated_dir.join("imgui.rs");
    bindings.write_to_file(&output_path)?;

    // Write SDL3 backend bindings
    let sdl3_output_path = generated_dir.join("sdl3_backend.rs");
    sdl3_bindings.write_to_file(&sdl3_output_path)?;

    // Write SDL3 GPU backend bindings
    let sdlgpu3_output_path = generated_dir.join("sdlgpu3_backend.rs");
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

    // Create mod.rs that includes all files
    let mod_content = r#"// Auto-generated bindings for Dear ImGui and SDL3 backends

#[allow(unused_imports)]
use core::*;

mod imgui;
pub use imgui::*;

pub mod sdl3;
pub mod sdlgpu3;
"#;
    let mod_rs_path = generated_dir.join("mod.rs");
    std::fs::write(&mod_rs_path, mod_content)?;

    // Create sdl3/mod.rs that re-exports SDL3 types from sdl3-sys
    let sdl3_mod_dir = generated_dir.join("sdl3");
    std::fs::create_dir_all(&sdl3_mod_dir)?;
    let sdl3_mod_content = r#"// SDL3 backend bindings

// Re-export SDL3 types from sdl3-sys for convenience
// Users can work with the same types across imgui and SDL3
pub use sdl3_sys::everything::{SDL_Window, SDL_Renderer, SDL_Gamepad, SDL_Event};

#[path = "../sdl3_backend.rs"]
mod backend;
pub use backend::*;
"#;
    let sdl3_mod_rs = sdl3_mod_dir.join("mod.rs");
    std::fs::write(&sdl3_mod_rs, sdl3_mod_content)?;

    // Create sdlgpu3/mod.rs that re-exports SDL3 GPU types from sdl3-sys
    let sdlgpu3_mod_dir = generated_dir.join("sdlgpu3");
    std::fs::create_dir_all(&sdlgpu3_mod_dir)?;
    let sdlgpu3_mod_content = r#"// SDL3 GPU backend bindings

// Re-export SDL3 GPU types from sdl3-sys for convenience
// Users can work with the same types across imgui and SDL3 GPU
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

// Re-export ImGui types needed by the backend
pub use crate::{ImDrawData, ImTextureData};

#[path = "../sdlgpu3_backend.rs"]
mod backend;
pub use backend::*;
"#;
    let sdlgpu3_mod_rs = sdlgpu3_mod_dir.join("mod.rs");
    std::fs::write(&sdlgpu3_mod_rs, sdlgpu3_mod_content)?;

    Ok(())
}
