// Dear ImGui: standalone example application for SDL3 + SDL_GPU
// (SDL is a cross-platform general purpose library for handling windows, inputs, OpenGL/Vulkan/Metal graphics context creation, etc.)

// This is a Rust translation of the C++ example from imgui/examples/example_sdl3_sdlgpu3

use imgui_sdl3_sys::{self, sdl3, sdlgpu3};
use sdl3_sys::everything::*;
use std::ffi::CString;
use std::ptr;

fn main() {
    unsafe {
        // Setup SDL
        if !SDL_Init(SDL_INIT_VIDEO | SDL_INIT_GAMEPAD) {
            eprintln!("Error: SDL_Init(): {}",
                std::ffi::CStr::from_ptr(SDL_GetError()).to_string_lossy());
            std::process::exit(1);
        }

        // Create SDL window graphics context
        let main_scale = SDL_GetDisplayContentScale(SDL_GetPrimaryDisplay());
        let window_flags = SDL_WINDOW_RESIZABLE | SDL_WINDOW_HIDDEN | SDL_WINDOW_HIGH_PIXEL_DENSITY;
        let title = CString::new("Dear ImGui SDL3+SDL_GPU example").unwrap();
        let window = SDL_CreateWindow(
            title.as_ptr(),
            (1280.0 * main_scale) as i32,
            (800.0 * main_scale) as i32,
            window_flags,
        );
        if window.is_null() {
            eprintln!("Error: SDL_CreateWindow(): {}",
                std::ffi::CStr::from_ptr(SDL_GetError()).to_string_lossy());
            std::process::exit(1);
        }
        SDL_SetWindowPosition(window, SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED);
        SDL_ShowWindow(window);

        // Create GPU Device
        let gpu_device = SDL_CreateGPUDevice(
            SDL_GPU_SHADERFORMAT_SPIRV | SDL_GPU_SHADERFORMAT_DXIL |
            SDL_GPU_SHADERFORMAT_MSL | SDL_GPU_SHADERFORMAT_METALLIB,
            true,
            ptr::null(),
        );
        if gpu_device.is_null() {
            eprintln!("Error: SDL_CreateGPUDevice(): {}",
                std::ffi::CStr::from_ptr(SDL_GetError()).to_string_lossy());
            std::process::exit(1);
        }

        // Claim window for GPU Device
        if !SDL_ClaimWindowForGPUDevice(gpu_device, window) {
            eprintln!("Error: SDL_ClaimWindowForGPUDevice(): {}",
                std::ffi::CStr::from_ptr(SDL_GetError()).to_string_lossy());
            std::process::exit(1);
        }
        SDL_SetGPUSwapchainParameters(
            gpu_device,
            window,
            SDL_GPU_SWAPCHAINCOMPOSITION_SDR,
            SDL_GPU_PRESENTMODE_VSYNC,
        );

        // Setup Dear ImGui context
        imgui_sdl3_sys::igCreateContext(ptr::null_mut());
        let io = imgui_sdl3_sys::igGetIO();
        (*io).ConfigFlags |= imgui_sdl3_sys::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard as i32;
        (*io).ConfigFlags |= imgui_sdl3_sys::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad as i32;

        // Setup Dear ImGui style
        imgui_sdl3_sys::igStyleColorsDark(ptr::null_mut());

        // Setup scaling
        let style = imgui_sdl3_sys::igGetStyle();
        imgui_sdl3_sys::ImGuiStyle_ScaleAllSizes(style, main_scale);
        (*style).FontScaleDpi = main_scale;

        // Setup Platform/Renderer backends
        sdl3::cImGui_ImplSDL3_InitForSDLGPU(window as *mut _);
        let mut init_info = sdlgpu3::cImGui_ImplSDLGPU3_InitInfo {
            Device: gpu_device,
            ColorTargetFormat: SDL_GetGPUSwapchainTextureFormat(gpu_device, window),
            MSAASamples: SDL_GPU_SAMPLECOUNT_1,
            SwapchainComposition: SDL_GPU_SWAPCHAINCOMPOSITION_SDR,
            PresentMode: SDL_GPU_PRESENTMODE_VSYNC,
        };
        sdlgpu3::cImGui_ImplSDLGPU3_Init(&mut init_info);

        // Our state
        let mut show_demo_window = true;
        let mut show_another_window = false;
        let mut clear_color = imgui_sdl3_sys::ImVec4 {
            x: 0.45,
            y: 0.55,
            z: 0.60,
            w: 1.00,
        };

        // Main loop
        let mut done = false;
        let mut event = SDL_Event::default();

        while !done {
            // Poll and handle events
            while SDL_PollEvent(&mut event) {
                sdl3::cImGui_ImplSDL3_ProcessEvent(&event as *const _ as *const _);
                if event.r#type == SDL_EVENT_QUIT {
                    done = true;
                }
                if event.r#type == SDL_EVENT_WINDOW_CLOSE_REQUESTED
                    && event.window.windowID == SDL_GetWindowID(window) {
                    done = true;
                }
            }

            // Skip rendering when minimized
            if SDL_GetWindowFlags(window) & SDL_WINDOW_MINIMIZED != 0 {
                SDL_Delay(10);
                continue;
            }

            // Start the Dear ImGui frame
            sdlgpu3::cImGui_ImplSDLGPU3_NewFrame();
            sdl3::cImGui_ImplSDL3_NewFrame();
            imgui_sdl3_sys::igNewFrame();

            // 1. Show the big demo window
            if show_demo_window {
                imgui_sdl3_sys::igShowDemoWindow(&mut show_demo_window);
            }

            // 2. Show a simple window that we create ourselves
            {
                use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
                static F_BITS: AtomicU32 = AtomicU32::new(0);
                static COUNTER: AtomicI32 = AtomicI32::new(0);

                let mut f = f32::from_bits(F_BITS.load(Ordering::Relaxed));
                let mut counter = COUNTER.load(Ordering::Relaxed);

                let window_title = CString::new("Hello, world!").unwrap();
                imgui_sdl3_sys::igBegin(window_title.as_ptr(), ptr::null_mut(), 0);

                let text = CString::new("This is some useful text.").unwrap();
                imgui_sdl3_sys::igText(text.as_ptr());

                let demo_label = CString::new("Demo Window").unwrap();
                imgui_sdl3_sys::igCheckbox(demo_label.as_ptr(), &mut show_demo_window);

                let another_label = CString::new("Another Window").unwrap();
                imgui_sdl3_sys::igCheckbox(another_label.as_ptr(), &mut show_another_window);

                let slider_label = CString::new("float").unwrap();
                imgui_sdl3_sys::igSliderFloat(slider_label.as_ptr(), &mut f, 0.0, 1.0);

                let color_label = CString::new("clear color").unwrap();
                imgui_sdl3_sys::igColorEdit3(color_label.as_ptr(), &mut clear_color.x, 0);

                let button_label = CString::new("Button").unwrap();
                if imgui_sdl3_sys::igButton(button_label.as_ptr()) {
                    counter += 1;
                }
                imgui_sdl3_sys::igSameLine();
                let counter_text = CString::new(format!("counter = {}", counter)).unwrap();
                imgui_sdl3_sys::igText(counter_text.as_ptr());

                let fps_text = CString::new(format!(
                    "Application average {:.3} ms/frame ({:.1} FPS)",
                    1000.0 / (*io).Framerate,
                    (*io).Framerate
                )).unwrap();
                imgui_sdl3_sys::igText(fps_text.as_ptr());

                imgui_sdl3_sys::igEnd();

                F_BITS.store(f.to_bits(), Ordering::Relaxed);
                COUNTER.store(counter, Ordering::Relaxed);
            }

            // 3. Show another simple window
            if show_another_window {
                let another_title = CString::new("Another Window").unwrap();
                imgui_sdl3_sys::igBegin(another_title.as_ptr(), &mut show_another_window, 0);

                let hello_text = CString::new("Hello from another window!").unwrap();
                imgui_sdl3_sys::igText(hello_text.as_ptr());

                let close_label = CString::new("Close Me").unwrap();
                if imgui_sdl3_sys::igButton(close_label.as_ptr()) {
                    show_another_window = false;
                }

                imgui_sdl3_sys::igEnd();
            }

            // Rendering
            imgui_sdl3_sys::igRender();
            let draw_data = imgui_sdl3_sys::igGetDrawData();
            let is_minimized = (*draw_data).DisplaySize.x <= 0.0 || (*draw_data).DisplaySize.y <= 0.0;

            let command_buffer = SDL_AcquireGPUCommandBuffer(gpu_device);

            let mut swapchain_texture: *mut SDL_GPUTexture = ptr::null_mut();
            SDL_WaitAndAcquireGPUSwapchainTexture(
                command_buffer,
                window,
                &mut swapchain_texture,
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if !swapchain_texture.is_null() && !is_minimized {
                // This is mandatory: call cImGui_ImplSDLGPU3_PrepareDrawData() to upload the vertex/index buffer!
                sdlgpu3::cImGui_ImplSDLGPU3_PrepareDrawData(draw_data as *mut _, command_buffer);

                // Setup and start a render pass
                let target_info = SDL_GPUColorTargetInfo {
                    texture: swapchain_texture,
                    clear_color: SDL_FColor {
                        r: clear_color.x,
                        g: clear_color.y,
                        b: clear_color.z,
                        a: clear_color.w,
                    },
                    load_op: SDL_GPU_LOADOP_CLEAR,
                    store_op: SDL_GPU_STOREOP_STORE,
                    mip_level: 0,
                    layer_or_depth_plane: 0,
                    cycle: false,
                    ..Default::default()
                };
                let render_pass = SDL_BeginGPURenderPass(command_buffer, &target_info, 1, ptr::null());

                // Render ImGui
                sdlgpu3::cImGui_ImplSDLGPU3_RenderDrawData(
                    draw_data as *mut _,
                    command_buffer,
                    render_pass,
                    ptr::null_mut(),
                );

                SDL_EndGPURenderPass(render_pass);
            }

            // Submit the command buffer
            SDL_SubmitGPUCommandBuffer(command_buffer);
        }

        // Cleanup
        SDL_WaitForGPUIdle(gpu_device);
        sdl3::cImGui_ImplSDL3_Shutdown();
        sdlgpu3::cImGui_ImplSDLGPU3_Shutdown();
        imgui_sdl3_sys::igDestroyContext(ptr::null_mut());

        SDL_ReleaseWindowFromGPUDevice(gpu_device, window);
        SDL_DestroyGPUDevice(gpu_device);
        SDL_DestroyWindow(window);
        SDL_Quit();
    }
}
