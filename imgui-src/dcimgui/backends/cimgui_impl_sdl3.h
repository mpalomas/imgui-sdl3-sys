// C wrapper for Dear ImGui SDL3 backend
// This provides a stable C ABI for the ImGui SDL3 backend functions

#ifndef CIMGUI_IMPL_SDL3_H
#define CIMGUI_IMPL_SDL3_H

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations
struct SDL_Window;
struct SDL_Renderer;
struct SDL_Gamepad;
typedef union SDL_Event SDL_Event;

// C ABI wrapper functions for ImGui SDL3 backend
bool cImGui_ImplSDL3_InitForOpenGL(struct SDL_Window* window, void* sdl_gl_context);
bool cImGui_ImplSDL3_InitForVulkan(struct SDL_Window* window);
bool cImGui_ImplSDL3_InitForD3D(struct SDL_Window* window);
bool cImGui_ImplSDL3_InitForMetal(struct SDL_Window* window);
bool cImGui_ImplSDL3_InitForSDLRenderer(struct SDL_Window* window, struct SDL_Renderer* renderer);
bool cImGui_ImplSDL3_InitForSDLGPU(struct SDL_Window* window);
bool cImGui_ImplSDL3_InitForOther(struct SDL_Window* window);
void cImGui_ImplSDL3_Shutdown(void);
void cImGui_ImplSDL3_NewFrame(void);
bool cImGui_ImplSDL3_ProcessEvent(const SDL_Event* event);

// Gamepad mode enum
typedef enum {
    cImGui_ImplSDL3_GamepadMode_AutoFirst = 0,
    cImGui_ImplSDL3_GamepadMode_AutoAll = 1,
    cImGui_ImplSDL3_GamepadMode_Manual = 2
} cImGui_ImplSDL3_GamepadMode;

void cImGui_ImplSDL3_SetGamepadMode(cImGui_ImplSDL3_GamepadMode mode, struct SDL_Gamepad** manual_gamepads_array, int manual_gamepads_count);

#ifdef __cplusplus
}
#endif

#endif // CIMGUI_IMPL_SDL3_H
