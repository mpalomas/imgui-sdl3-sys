//! Simplified linkage verification test
//!
//! This test verifies that core ImGui functions link correctly
//! without dealing with complex overloads and variadic functions.

use imgui_sdl3_sys::*;
use core::ptr;

/// Test core functionality links correctly
#[test]
fn test_core_functions_link() {
    unsafe {
        // Create and destroy context
        let ctx = igCreateContext(ptr::null_mut());
        assert!(!ctx.is_null());

        igSetCurrentContext(ctx);
        assert_eq!(igGetCurrentContext(), ctx);

        // Get IO and Style
        let io = igGetIO();
        let style = igGetStyle();
        assert!(!io.is_null());
        assert!(!style.is_null());

        // Setup display size
        (*io).DisplaySize.x = 800.0;
        (*io).DisplaySize.y = 600.0;
        (*io).DeltaTime = 1.0 / 60.0;

        // Frame cycle
        igNewFrame();
        igEndFrame();
        igRender();

        let draw_data = igGetDrawData();
        assert!(!draw_data.is_null());

        igDestroyContext(ctx);
    }
}

/// Test basic window and widget functions
#[test]
fn test_window_and_widgets_link() {
    unsafe {
        let ctx = igCreateContext(ptr::null_mut());
        igSetCurrentContext(ctx);

        let io = igGetIO();
        (*io).DisplaySize.x = 800.0;
        (*io).DisplaySize.y = 600.0;
        (*io).DeltaTime = 1.0 / 60.0;

        igNewFrame();

        // Window
        if igBegin(b"Test\0".as_ptr() as *const i8, ptr::null_mut(), 0) {
            // Text
            igText(b"Hello\0".as_ptr() as *const i8);

            // Button
            let _ = igButton(b"Click\0".as_ptr() as *const i8);

            // Checkbox
            let mut checked = false;
            igCheckbox(b"Check\0".as_ptr() as *const i8, &mut checked);

            // Slider
            let mut value = 0.5f32;
            igSliderFloat(
                b"Slider\0".as_ptr() as *const i8,
                &mut value,
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

/// Test drawing functions
#[test]
fn test_drawing_functions_link() {
    unsafe {
        let ctx = igCreateContext(ptr::null_mut());
        igSetCurrentContext(ctx);

        let io = igGetIO();
        (*io).DisplaySize.x = 800.0;
        (*io).DisplaySize.y = 600.0;
        (*io).DeltaTime = 1.0 / 60.0;

        igNewFrame();

        if igBegin(b"Draw\0".as_ptr() as *const i8, ptr::null_mut(), 0) {
            let draw_list = igGetWindowDrawList();
            assert!(!draw_list.is_null());

            // Draw rectangle
            ImDrawList_AddRect(
                draw_list,
                ImVec2 { x: 10.0, y: 10.0 },
                ImVec2 { x: 100.0, y: 100.0 },
                0xFFFFFFFF
            );

            // Draw line
            ImDrawList_AddLine(
                draw_list,
                ImVec2 { x: 0.0, y: 0.0 },
                ImVec2 { x: 50.0, y: 50.0 },
                0xFF0000FF
            );

            igEnd();
        }

        igEndFrame();
        igRender();
        igDestroyContext(ctx);
    }
}

/// Test utility functions
#[test]
fn test_utility_functions_link() {
    unsafe {
        let ctx = igCreateContext(ptr::null_mut());
        igSetCurrentContext(ctx);

        // Version
        let _ = igGetVersion();

        // Time/Frame
        let _ = igGetTime();
        let _ = igGetFrameCount();

        // Clipboard
        igSetClipboardText(b"test\0".as_ptr() as *const i8);
        let _ = igGetClipboardText();

        igDestroyContext(ctx);
    }
}
