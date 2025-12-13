//! Smoke tests for imgui-sys bindings
//!
//! These tests verify that:
//! 1. The bindings compile correctly
//! 2. Functions can be called and linked successfully
//! 3. Basic ImGui functionality works

use imgui_sdl3_sys::imgui_sys::*;

#[test]
fn test_version_constants() {
    // Test that constants are accessible
    assert_eq!(IMGUI_VERSION_NUM, 19250);

    // Test version string
    let version = unsafe {
        std::ffi::CStr::from_ptr(IMGUI_VERSION.as_ptr() as *const i8)
    };
    assert_eq!(version.to_str().unwrap(), "1.92.5");
}

#[test]
fn test_context_creation_and_destruction() {
    unsafe {
        // Create a context
        let ctx = igCreateContext(core::ptr::null_mut());
        assert!(!ctx.is_null(), "Context creation failed");

        // Destroy the context
        igDestroyContext(ctx);
    }
}

#[test]
fn test_context_switching() {
    unsafe {
        let ctx1 = igCreateContext(core::ptr::null_mut());
        let ctx2 = igCreateContext(core::ptr::null_mut());

        igSetCurrentContext(ctx1);
        let current = igGetCurrentContext();
        assert_eq!(current, ctx1);

        igSetCurrentContext(ctx2);
        let current = igGetCurrentContext();
        assert_eq!(current, ctx2);

        igDestroyContext(ctx1);
        igDestroyContext(ctx2);
    }
}

#[test]
fn test_io_access() {
    unsafe {
        let ctx = igCreateContext(core::ptr::null_mut());
        igSetCurrentContext(ctx);

        // Get IO - this should not crash
        let io = igGetIO();
        assert!(!io.is_null(), "IO pointer should not be null");

        igDestroyContext(ctx);
    }
}

#[test]
fn test_style_access() {
    unsafe {
        let ctx = igCreateContext(core::ptr::null_mut());
        igSetCurrentContext(ctx);

        // Get Style - this should not crash
        let style = igGetStyle();
        assert!(!style.is_null(), "Style pointer should not be null");

        igDestroyContext(ctx);
    }
}

#[test]
fn test_frame_functions() {
    unsafe {
        let ctx = igCreateContext(core::ptr::null_mut());
        igSetCurrentContext(ctx);

        let io = igGetIO();
        (*io).DisplaySize.x = 800.0;
        (*io).DisplaySize.y = 600.0;
        (*io).DeltaTime = 1.0 / 60.0;

        // These should not crash
        igNewFrame();
        igEndFrame();
        igRender();

        let draw_data = igGetDrawData();
        assert!(!draw_data.is_null(), "Draw data should not be null after render");

        igDestroyContext(ctx);
    }
}

#[test]
fn test_window_functions_linkage() {
    unsafe {
        let ctx = igCreateContext(core::ptr::null_mut());
        igSetCurrentContext(ctx);

        let io = igGetIO();
        (*io).DisplaySize.x = 800.0;
        (*io).DisplaySize.y = 600.0;
        (*io).DeltaTime = 1.0 / 60.0;

        igNewFrame();

        // Test window functions - just verify they link and can be called
        let window_name = b"Test Window\0";
        let is_open = igBegin(
            window_name.as_ptr() as *const i8,
            core::ptr::null_mut(),
            0
        );

        if is_open {
            igText(b"Hello, world!\0".as_ptr() as *const i8);
            igEnd();
        }

        igEndFrame();
        igRender();

        igDestroyContext(ctx);
    }
}

#[test]
fn test_widget_functions_linkage() {
    unsafe {
        let ctx = igCreateContext(core::ptr::null_mut());
        igSetCurrentContext(ctx);

        let io = igGetIO();
        (*io).DisplaySize.x = 800.0;
        (*io).DisplaySize.y = 600.0;
        (*io).DeltaTime = 1.0 / 60.0;

        igNewFrame();

        let window_name = b"Widget Test\0";
        if igBegin(window_name.as_ptr() as *const i8, core::ptr::null_mut(), 0) {
            // Test various widget functions for linkage
            igButton(b"Click Me\0".as_ptr() as *const i8);

            let mut checkbox_value = false;
            igCheckbox(b"Check Me\0".as_ptr() as *const i8, &mut checkbox_value as *mut bool);

            let mut slider_value = 0.5f32;
            igSliderFloat(
                b"Slider\0".as_ptr() as *const i8,
                &mut slider_value as *mut f32,
                0.0,
                1.0
            );

            igEnd();
        }

        igEndFrame();
        igRender();

        igDestroyContext(ctx);
    }
}

#[test]
fn test_draw_list_functions() {
    unsafe {
        let ctx = igCreateContext(core::ptr::null_mut());
        igSetCurrentContext(ctx);

        let io = igGetIO();
        (*io).DisplaySize.x = 800.0;
        (*io).DisplaySize.y = 600.0;
        (*io).DeltaTime = 1.0 / 60.0;

        igNewFrame();

        if igBegin(b"Draw Test\0".as_ptr() as *const i8, core::ptr::null_mut(), 0) {
            let draw_list = igGetWindowDrawList();
            assert!(!draw_list.is_null(), "Window draw list should not be null");

            // Test draw functions linkage
            let p_min = ImVec2 { x: 10.0, y: 10.0 };
            let p_max = ImVec2 { x: 50.0, y: 50.0 };
            let col = 0xFF0000FFu32; // Red color

            ImDrawList_AddRect(draw_list, p_min, p_max, col);

            igEnd();
        }

        igEndFrame();
        igRender();

        igDestroyContext(ctx);
    }
}

#[test]
fn test_font_atlas_functions() {
    unsafe {
        let ctx = igCreateContext(core::ptr::null_mut());
        igSetCurrentContext(ctx);

        let io = igGetIO();
        let fonts = (*io).Fonts;
        assert!(!fonts.is_null(), "Font atlas should not be null");

        // Test font atlas functions linkage
        ImFontAtlas_GetTexDataAsRGBA32(
            fonts,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut()
        );

        igDestroyContext(ctx);
    }
}

#[test]
fn test_types_are_correct_size() {
    // Verify some key types have reasonable sizes
    assert!(core::mem::size_of::<ImVec2>() == 8); // 2 floats
    assert!(core::mem::size_of::<ImVec4>() == 16); // 4 floats
    assert!(core::mem::size_of::<ImDrawVert>() > 0);
    assert!(core::mem::size_of::<ImDrawCmd>() > 0);
}

#[test]
fn test_opaque_types_exist() {
    // Just verify that opaque types compile and can be referenced
    let _ctx_size = core::mem::size_of::<*mut ImGuiContext>();
    let _builder_size = core::mem::size_of::<*mut ImFontAtlasBuilder>();
    let _loader_size = core::mem::size_of::<*mut ImFontLoader>();
    let _shared_data_size = core::mem::size_of::<*mut ImDrawListSharedData>();
}
