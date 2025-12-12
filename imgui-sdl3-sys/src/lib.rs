#![no_std]
#![cfg_attr(all(feature = "nightly", doc), feature(doc_cfg))]

mod generated;
pub use generated::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdl3_backend_functions_exist() {
        // Verify that SDL3 backend functions are available
        let _ = sdl3::ImGui_ImplSDL3_Shutdown as unsafe extern "C" fn();
        let _ = sdl3::ImGui_ImplSDL3_NewFrame as unsafe extern "C" fn();
    }

    #[test]
    fn test_sdl3_types_are_from_sdl3_sys() {
        // Verify that SDL3 types are the actual sdl3_sys types, not opaque wrappers
        // This is a compile-time check - if types are compatible, this will compile

        use sdl3_sys::everything::SDL_Window as SdlWindow;
        use sdl3::SDL_Window as ImGuiWindow;

        // These should be the same type
        let _: fn(*mut SdlWindow) = |w: *mut ImGuiWindow| {
            let _: *mut SdlWindow = w;
        };
    }
}
