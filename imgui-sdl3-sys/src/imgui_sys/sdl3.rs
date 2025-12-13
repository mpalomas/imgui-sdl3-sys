// SDL3 backend bindings (both renderer and GPU)

// Re-export SDL3 types from sdl3-sys for convenience
// Users can work with the same types across imgui and SDL3
pub use crate::sdl3_sys::everything::{
    SDL_Window,
    SDL_Renderer,
    SDL_Gamepad,
    SDL_Event,
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

// Re-export ImGui types needed by the backends
pub use super::{ImDrawData, ImTextureData};

// SDL3 renderer backend bindings
#[path = "../bindings/sdl3_backend.rs"]
mod renderer_backend;
pub use renderer_backend::*;

// SDL3 GPU backend bindings
#[path = "../bindings/sdlgpu3_backend.rs"]
mod gpu_backend;
pub use gpu_backend::*;
