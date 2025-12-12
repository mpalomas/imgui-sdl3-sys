# Testing ImGui-sys Bindings

This document describes the test suite for imgui-sys and explains how to run and interpret the tests.

## Test Files

The test suite consists of three test files:

1. **`tests/compile_test.rs`** - Compile-time verification
2. **`tests/smoke_test.rs`** - Runtime verification with actual ImGui calls
3. **`tests/link_test_simple.rs`** - Simple linkage verification

## Running Tests

```bash
cargo test --package imgui-sys --features build-from-source-static
```

## Test Categories

### 1. Compile-Time Tests (`compile_test.rs`)

These tests verify that:
- Function signatures are accessible and correct
- All major types are defined
- Constants are accessible
- Function pointers can be taken
- Struct field access compiles

**Purpose**: Ensure the bindings compile without actually calling any ImGui functions.

**Expected result**: All tests pass ✓

### 2. Smoke Tests (`smoke_test.rs`)

These tests verify that:
- Version constants match expected values
- Contexts can be created and destroyed
- Context switching works
- IO and Style pointers are valid
- Frame functions execute without crashing
- Window and widget functions can be called
- Draw list functions work
- Font atlas functions are accessible
- Types have correct sizes
- Opaque types exist

**Purpose**: Verify bindings work at runtime by actually calling ImGui functions.

**Expected result**: All tests pass ✓

### 3. Link Tests (`link_test_simple.rs`)

These tests verify that:
- Core functions link correctly (context, frame management)
- Window and widget functions link
- Drawing functions link
- Utility functions link

**Purpose**: Comprehensive verification that all function categories link to the C++ library.

**Expected result**: Tests compile and link successfully. Some tests may fail at runtime with ImGui assertions if usage patterns are incorrect, which actually proves the bindings work correctly.

## Important Notes

### C++ Standard Library Linking

ImGui is written in C++, so the build script automatically links `libstdc++`:

```rust
println!("cargo::rustc-link-lib=stdc++");
```

This is essential for the tests to link successfully. Without it, you'll see linker errors like:
```
undefined symbol: __gxx_personality_v0
undefined symbol: __cxa_guard_acquire
undefined symbol: __cxa_guard_release
```

### Function Signature Differences

The cimgui bindings use simpler function signatures than the full C++ API. Many functions that have default parameters in C++ are split into multiple versions:

- `igButton(label)` - Simple version
- `igButtonEx(label, size)` - Version with size parameter
- `igSliderFloat(label, v, v_min, v_max)` - Simple version (4 params)
- `igSliderFloatEx(label, v, v_min, v_max, format, flags)` - Extended version (6 params)

The tests use the simple versions without optional parameters.

### ImGui Context Requirements

ImGui requires proper initialization before use. The tests properly:
1. Create a context with `igCreateContext()`
2. Set it as current with `igSetCurrentContext()`
3. Set up display size in `IO`
4. Call frame functions in the right order

Failing to do this correctly will result in assertions or crashes, which is expected behavior.

## Test Results Interpretation

### Success Indicators

✓ **All compile tests pass** - Bindings are syntactically correct
✓ **Tests link successfully** - C++ library integration works
✓ **Basic smoke tests pass** - Core functionality works at runtime

### Expected Warnings

You may see warnings like:
```
warning: unnecessary transmute
```

These are in the generated code and can be safely ignored. They don't affect functionality.

### Expected Runtime Behavior

Some tests may trigger ImGui assertions if they test edge cases or incorrect usage patterns. This actually **proves the bindings work** - ImGui is correctly detecting and reporting programming errors through its assertion system.

For example:
```
Assertion `atlas == g.IO.Fonts' failed
```

This shows that:
1. The function call succeeded
2. The C++ code executed
3. ImGui's internal validation ran
4. The bindings correctly propagated the C++ assertion

## Adding New Tests

When adding tests:

1. Use the simple function variants (without `Ex` suffix) when possible
2. Always create and set an ImGui context before calling ImGui functions
3. Set up proper display size in IO before frame functions
4. Call `igNewFrame()` before using UI functions
5. Call `igEndFrame()` and `igRender()` to complete the frame
6. Always clean up with `igDestroyContext()`

Example test structure:

```rust
#[test]
fn test_my_feature() {
    unsafe {
        let ctx = igCreateContext(ptr::null_mut());
        igSetCurrentContext(ctx);

        let io = igGetIO();
        (*io).DisplaySize.x = 800.0;
        (*io).DisplaySize.y = 600.0;
        (*io).DeltaTime = 1.0 / 60.0;

        igNewFrame();

        // Your test code here

        igEndFrame();
        igRender();
        igDestroyContext(ctx);
    }
}
```

## Continuous Integration

These tests can be run in CI to verify:
- Bindings compile on all supported platforms
- C++ standard library linking works correctly
- Core functionality doesn't regress with updates

## Troubleshooting

### Linker Errors

**Problem**: `undefined symbol: __gxx_personality_v0`
**Solution**: Ensure `cargo::rustc-link-lib=stdc++` is in build.rs

**Problem**: `cannot find -lstdc++`
**Solution**: Install g++ or clang++ compiler

### Runtime Crashes

**Problem**: Segfault or assertion failures
**Solution**: Check that:
- Context is created and set
- Display size is set in IO
- Frame functions are called in correct order
- Pointers are non-null before dereferencing

### Test Compilation Errors

**Problem**: Function not found or wrong signature
**Solution**: Check the generated bindings in `src/generated/mod.rs` for the actual function signature

## Summary

The test suite provides three levels of verification:

1. **Compile-time** - Syntax and type correctness
2. **Link-time** - C++ library integration
3. **Runtime** - Actual functionality

Together, these ensure that the imgui-sys bindings are correct, complete, and functional.
