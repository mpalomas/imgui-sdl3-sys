// SDL3 backend bindings

// Re-export SDL3 types from sdl3-sys for convenience
// Users can work with the same types across imgui and SDL3
pub use sdl3_sys::everything::{SDL_Window, SDL_Renderer, SDL_Gamepad, SDL_Event};

#[path = "bindings/sdl3_backend.rs"]
mod backend;
pub use backend::*;
