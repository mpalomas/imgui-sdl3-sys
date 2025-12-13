// ImGui bindings module
// This module contains all Dear ImGui bindings including core and backends

#[allow(unused_imports)]
use core::*;

// Core ImGui bindings
#[path = "bindings/imgui.rs"]
mod imgui_bindings;
pub use imgui_bindings::*;

// SDL3 backend submodule (includes both renderer and GPU backends)
pub mod sdl3;
