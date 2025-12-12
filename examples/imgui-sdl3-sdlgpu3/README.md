# Dear ImGui SDL3 + SDL_GPU Example

This is a Rust translation of the Dear ImGui example application for SDL3 + SDL_GPU from the C++ version at `imgui/examples/example_sdl3_sdlgpu3`.

## Features

- Uses SDL3's GPU API for rendering
- Demonstrates basic Dear ImGui widgets and windows
- Shows the demo window with all ImGui features
- Custom window with interactive controls (slider, color picker, button, etc.)

## Important Notes

**CRITICAL:** Unlike other backends, you must call `ImGui_ImplSDLGPU3_PrepareDrawData()` BEFORE issuing a `SDL_GPURenderPass` containing `ImGui_ImplSDLGPU3_RenderDrawData()`. This function uploads the vertex and index buffers to the GPU. See the rendering section in `main.rs` for the correct usage.

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

The example will display:
1. The ImGui demo window (toggle with checkbox)
2. A "Hello, world!" window with various controls
3. An optional "Another Window" (toggle with checkbox)

## Code Structure

The example demonstrates:
- SDL3 initialization with GPU device creation
- ImGui context creation and configuration
- SDL3 backend initialization for SDLGPU
- Main event loop with event processing
- ImGui frame rendering
- Proper cleanup of all resources

## License

This example follows the same license as Dear ImGui (MIT License).
