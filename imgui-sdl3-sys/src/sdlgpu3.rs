// SDL3 GPU backend bindings

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

#[path = "bindings/sdlgpu3_backend.rs"]
mod backend;
pub use backend::*;
