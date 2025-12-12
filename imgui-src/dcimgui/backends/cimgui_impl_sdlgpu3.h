// C wrapper for Dear ImGui SDL_GPU backend
// This provides a stable C ABI for the ImGui SDL_GPU backend functions

#ifndef CIMGUI_IMPL_SDLGPU3_H
#define CIMGUI_IMPL_SDLGPU3_H

#include <stdbool.h>
#include <stdint.h>
#include <SDL3/SDL_gpu.h>

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations for ImGui types
typedef struct ImDrawData ImDrawData;
typedef struct ImTextureData ImTextureData;

// Initialization data for C wrapper
// Using actual SDL enum types for better ergonomics
typedef struct {
    SDL_GPUDevice* Device;
    SDL_GPUTextureFormat ColorTargetFormat;
    SDL_GPUSampleCount MSAASamples;
    SDL_GPUSwapchainComposition SwapchainComposition;
    SDL_GPUPresentMode PresentMode;
} cImGui_ImplSDLGPU3_InitInfo;

// C ABI wrapper functions for ImGui SDL_GPU backend
bool cImGui_ImplSDLGPU3_Init(cImGui_ImplSDLGPU3_InitInfo* info);
void cImGui_ImplSDLGPU3_Shutdown(void);
void cImGui_ImplSDLGPU3_NewFrame(void);
void cImGui_ImplSDLGPU3_PrepareDrawData(ImDrawData* draw_data, SDL_GPUCommandBuffer* command_buffer);
void cImGui_ImplSDLGPU3_RenderDrawData(ImDrawData* draw_data, SDL_GPUCommandBuffer* command_buffer, SDL_GPURenderPass* render_pass, SDL_GPUGraphicsPipeline* pipeline);
void cImGui_ImplSDLGPU3_CreateDeviceObjects(void);
void cImGui_ImplSDLGPU3_DestroyDeviceObjects(void);
void cImGui_ImplSDLGPU3_UpdateTexture(ImTextureData* tex);

// Render state struct
typedef struct {
    SDL_GPUDevice* Device;
    SDL_GPUSampler* SamplerDefault;
    SDL_GPUSampler* SamplerCurrent;
} cImGui_ImplSDLGPU3_RenderState;

#ifdef __cplusplus
}
#endif

#endif // CIMGUI_IMPL_SDLGPU3_H
