//! Compile-time tests for imgui-sys bindings
//!
//! These tests verify that the bindings compile and link correctly
//! without actually executing any ImGui code.

use imgui_sys::*;

/// Test that all major function signatures are accessible
#[test]
fn test_function_signatures_exist() {
    // Context functions
    let _: unsafe extern "C" fn(*mut ImFontAtlas) -> *mut ImGuiContext = igCreateContext;
    let _: unsafe extern "C" fn(*mut ImGuiContext) = igDestroyContext;
    let _: unsafe extern "C" fn() -> *mut ImGuiContext = igGetCurrentContext;
    let _: unsafe extern "C" fn(*mut ImGuiContext) = igSetCurrentContext;

    // IO and Style
    let _: unsafe extern "C" fn() -> *mut ImGuiIO = igGetIO;
    let _: unsafe extern "C" fn() -> *mut ImGuiStyle = igGetStyle;

    // Frame functions
    let _: unsafe extern "C" fn() = igNewFrame;
    let _: unsafe extern "C" fn() = igEndFrame;
    let _: unsafe extern "C" fn() = igRender;
    let _: unsafe extern "C" fn() -> *mut ImDrawData = igGetDrawData;

    // Window functions
    let _: unsafe extern "C" fn(*const i8, *mut bool, i32) -> bool = igBegin;
    let _: unsafe extern "C" fn() = igEnd;

    // Widget functions - note: some are variadic or have different signatures than expected
    let _: unsafe extern "C" fn(*const i8) -> bool = igButton;
    let _: unsafe extern "C" fn(*const i8, *mut bool) -> bool = igCheckbox;

    // Demo/Debug windows
    let _: unsafe extern "C" fn(*mut bool) = igShowDemoWindow;
    let _: unsafe extern "C" fn(*mut bool) = igShowMetricsWindow;
}

/// Test that all major types are defined
#[test]
fn test_types_are_defined() {
    // Opaque pointer types
    let _: Option<*mut ImGuiContext> = None;
    let _: Option<*mut ImGuiIO> = None;
    let _: Option<*mut ImGuiStyle> = None;
    let _: Option<*mut ImDrawData> = None;
    let _: Option<*mut ImDrawList> = None;
    let _: Option<*mut ImFont> = None;
    let _: Option<*mut ImFontAtlas> = None;

    // Value types
    let _: ImVec2 = unsafe { core::mem::zeroed() };
    let _: ImVec4 = unsafe { core::mem::zeroed() };
    let _: ImDrawVert = unsafe { core::mem::zeroed() };
    let _: ImDrawCmd = unsafe { core::mem::zeroed() };

    // Type aliases
    let _: ImGuiID = 0u32;
    let _: ImDrawIdx = 0u16;
    let _: ImU32 = 0u32;
}

/// Test that constants are accessible
#[test]
fn test_constants_accessible() {
    let _ = IMGUI_VERSION;
    let _ = IMGUI_VERSION_NUM;
    let _ = IMGUI_PAYLOAD_TYPE_COLOR_3F;
    let _ = IMGUI_PAYLOAD_TYPE_COLOR_4F;
    let _ = IM_COL32_R_SHIFT;
    let _ = IM_COL32_G_SHIFT;
    let _ = IM_COL32_B_SHIFT;
    let _ = IM_COL32_A_SHIFT;
}

/// Test that we can take function pointers
#[test]
fn test_function_pointers() {
    let _create_ctx: unsafe extern "C" fn(*mut ImFontAtlas) -> *mut ImGuiContext =
        igCreateContext;
    let _destroy_ctx: unsafe extern "C" fn(*mut ImGuiContext) = igDestroyContext;
}

/// Test struct field access compiles
#[test]
fn test_struct_field_access() {
    let vec2 = ImVec2 { x: 1.0, y: 2.0 };
    assert_eq!(vec2.x, 1.0);
    assert_eq!(vec2.y, 2.0);

    let vec4 = ImVec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    assert_eq!(vec4.x, 1.0);
    assert_eq!(vec4.w, 4.0);
}
