// C wrapper implementation for Dear ImGui SDL3 backend
// This provides a stable C ABI by wrapping the C++ functions

#include "cimgui_impl_sdl3.h"
#include "imgui_impl_sdl3.h"

extern "C" {

bool cImGui_ImplSDL3_InitForOpenGL(SDL_Window* window, void* sdl_gl_context) {
    return ImGui_ImplSDL3_InitForOpenGL(window, sdl_gl_context);
}

bool cImGui_ImplSDL3_InitForVulkan(SDL_Window* window) {
    return ImGui_ImplSDL3_InitForVulkan(window);
}

bool cImGui_ImplSDL3_InitForD3D(SDL_Window* window) {
    return ImGui_ImplSDL3_InitForD3D(window);
}

bool cImGui_ImplSDL3_InitForMetal(SDL_Window* window) {
    return ImGui_ImplSDL3_InitForMetal(window);
}

bool cImGui_ImplSDL3_InitForSDLRenderer(SDL_Window* window, SDL_Renderer* renderer) {
    return ImGui_ImplSDL3_InitForSDLRenderer(window, renderer);
}

bool cImGui_ImplSDL3_InitForSDLGPU(SDL_Window* window) {
    return ImGui_ImplSDL3_InitForSDLGPU(window);
}

bool cImGui_ImplSDL3_InitForOther(SDL_Window* window) {
    return ImGui_ImplSDL3_InitForOther(window);
}

void cImGui_ImplSDL3_Shutdown(void) {
    ImGui_ImplSDL3_Shutdown();
}

void cImGui_ImplSDL3_NewFrame(void) {
    ImGui_ImplSDL3_NewFrame();
}

bool cImGui_ImplSDL3_ProcessEvent(const SDL_Event* event) {
    return ImGui_ImplSDL3_ProcessEvent(event);
}

void cImGui_ImplSDL3_SetGamepadMode(cImGui_ImplSDL3_GamepadMode mode, SDL_Gamepad** manual_gamepads_array, int manual_gamepads_count) {
    ImGui_ImplSDL3_SetGamepadMode(
        static_cast<ImGui_ImplSDL3_GamepadMode>(mode),
        manual_gamepads_array,
        manual_gamepads_count
    );
}

} // extern "C"
