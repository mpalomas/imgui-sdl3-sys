cmake_minimum_required(VERSION 3.20)
set(CMAKE_CXX_STANDARD 11)

set(MAJOR_VERSION 1)
set(MINOR_VERSION 92)
set(MICRO_VERSION 5)
set(SDL_REQUIRED_VERSION 3.2.6)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")

project(dcimgui VERSION "${MAJOR_VERSION}.${MINOR_VERSION}.${MICRO_VERSION}")

if(CMAKE_SOURCE_DIR STREQUAL PROJECT_SOURCE_DIR)
    set(IMGUI_ROOTPROJECT ON)
else()
    set(IMGUI_ROOTPROJECT OFF)
endif()

# Save BUILD_SHARED_LIBS variable
set(IMGUI_BUILD_SHARED_LIBS "${BUILD_SHARED_LIBS}")

set(sdl_required_components Headers)

if(IMGUI_BUILD_SHARED_LIBS)
    set(imgui_target_name imgui-shared)
    set(sdl3_target_name SDL3::SDL3-shared)

    list(APPEND sdl_required_components SDL3-shared)
else()
    set(imgui_target_name imgui-static)
    set(sdl3_target_name SDL3::SDL3)
endif()

if(NOT TARGET SDL3::Headers OR NOT TARGET ${sdl3_target_name})
    find_package(SDL3 ${SDL_REQUIRED_VERSION} REQUIRED COMPONENTS ${sdl_required_components})
endif()

set(imgui_install_enableable ON)
if((TARGET SDL3-shared OR TARGET SDL3-static) AND SDL_DISABLE_INSTALL)
    # Cannot install imgui when SDL3 is built in same built, and is not installed.
    set(imgui_install_enableable OFF)
endif()

include(CMakeDependentOption)
include(CMakePackageConfigHelpers)
include(GNUInstallDirs)
include(CheckSymbolExists)

cmake_dependent_option(IMGUI_INSTALL "Enable SDL3_image install target" ${IMGUI_ROOTPROJECT} "${imgui_install_enableable}" OFF)
# set(IMGUI_INSTALL ON)

add_library(${imgui_target_name}
        cimgui.cpp
        cimgui_internal.cpp
        imgui_demo.cpp
        imgui_draw.cpp
        imgui_tables.cpp
        imgui_widgets.cpp
        imgui.cpp
        backends/imgui_impl_sdl3.cpp
        backends/imgui_impl_sdlgpu3.cpp
        backends/cimgui_impl_sdl3.cpp
        backends/cimgui_impl_sdlgpu3.cpp
        )

# target_include_directories(imgui PUBLIC .)
if ((${CMAKE_CXX_COMPILER_ID} MATCHES "Clang") OR (${CMAKE_CXX_COMPILER_ID} MATCHES "GNU"))
    target_compile_options(${imgui_target_name} PUBLIC -Wno-return-type-c-linkage -Wno-unused-function)
elseif (MSVC)
    target_compile_options(${imgui_target_name} PUBLIC /wd4190)
endif()

target_include_directories(${imgui_target_name}
    PUBLIC
        "$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>"
        "$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/backends>"
        "$<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>"
)
target_link_libraries(${imgui_target_name} PUBLIC SDL3::Headers)
if(IMGUI_BUILD_SHARED_LIBS)
    target_link_libraries(${imgui_target_name} PRIVATE SDL3::SDL3-shared)
endif()

set_target_properties(${imgui_target_name} PROPERTIES
    OUTPUT_NAME imgui
    DEFINE_SYMBOL DLL_EXPORT
    EXPORT_NAME ${imgui_target_name}
    C_VISIBILITY_PRESET "hidden"
)
if(NOT ANDROID)
    set_target_properties(${imgui_target_name} PROPERTIES
        SOVERSION "${SO_VERSION_MAJOR}"
        VERSION "${SO_VERSION}"
    )
    if(APPLE)
        cmake_minimum_required(VERSION 3.17...3.28)
        set_target_properties(${imgui_target_name} PROPERTIES
            MACHO_COMPATIBILITY_VERSION "${DYLIB_COMPAT_VERSION}"
            MACHO_CURRENT_VERSION "${DYLIB_CURRENT_VERSION}"
        )
    endif()
endif()
if(IMGUI_BUILD_SHARED_LIBS)
    if(WIN32)
        set_target_properties(${imgui_target_name} PROPERTIES
            PREFIX ""
        )
    endif()
else()
    if(MSVC)
        set_target_properties(${imgui_target_name} PROPERTIES
            OUTPUT_NAME "imgui-static"
        )
    endif()
endif()

if(IMGUI_BUILD_SHARED_LIBS)
    # Use `Compatible Interface Properties` to ensure a shared SDL3_ttf is linked to a shared SDL3 library
    set_property(TARGET ${imgui_target_name} PROPERTY INTERFACE_SDL3_SHARED ${IMGUI_BUILD_SHARED_LIBS})
    set_property(TARGET ${imgui_target_name} APPEND PROPERTY COMPATIBLE_INTERFACE_BOOL SDL3_SHARED)
endif()

if(IMGUI_INSTALL)
    install(
        TARGETS ${imgui_target_name}
        EXPORT imguiTargets
        ARCHIVE DESTINATION "${CMAKE_INSTALL_LIBDIR}" COMPONENT devel
        LIBRARY DESTINATION "${CMAKE_INSTALL_LIBDIR}" COMPONENT library
        RUNTIME DESTINATION "${CMAKE_INSTALL_BINDIR}" COMPONENT library
    )
    install(
        FILES "${CMAKE_CURRENT_SOURCE_DIR}/cimgui.h"
        "${CMAKE_CURRENT_SOURCE_DIR}/cimgui_internal.h"
        "${CMAKE_CURRENT_SOURCE_DIR}/cimgui_all.h"
        "${CMAKE_CURRENT_SOURCE_DIR}/imgui.h"
        "${CMAKE_CURRENT_SOURCE_DIR}/imgui_internal.h"
        "${CMAKE_CURRENT_SOURCE_DIR}/imconfig.h"
        "${CMAKE_CURRENT_SOURCE_DIR}/backends/cimgui_impl_sdl3.h"
        "${CMAKE_CURRENT_SOURCE_DIR}/backends/cimgui_impl_sdlgpu3.h"
        DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}/dcimgui" COMPONENT DEVEL
    )

    if(BUILD_SHARED_LIBS)
        set(pdbdir "${CMAKE_INSTALL_BINDIR}")
    else()
        set(pdbdir "${CMAKE_INSTALL_LIBDIR}")
    endif()

    if(WIN32 AND NOT MINGW)
        set(IMGUI_INSTALL_CMAKEDIR_DEFAULT "cmake")
    else()
        set(IMGUI_INSTALL_CMAKEDIR_DEFAULT "${CMAKE_INSTALL_LIBDIR}/cmake")
    endif()
    set(IMGUI_INSTALL_CMAKEDIR_ROOT "${IMGUI_INSTALL_CMAKEDIR_DEFAULT}" CACHE STRING "Location where to install imguiConfig.cmake")
    set(IMGUI_PKGCONFIG_INSTALLDIR "${CMAKE_INSTALL_LIBDIR}/pkgconfig")

    if(WIN32 AND NOT MINGW)
        set(IMGUI_INSTALL_CMAKEDIR "${IMGUI_INSTALL_CMAKEDIR_ROOT}")
    else()
        set(IMGUI_INSTALL_CMAKEDIR "${IMGUI_INSTALL_CMAKEDIR_ROOT}/imgui")
    endif()

    configure_package_config_file(cmake/imguiConfig.cmake.in imguiConfig.cmake
        NO_SET_AND_CHECK_MACRO
        INSTALL_DESTINATION "${IMGUI_INSTALL_CMAKEDIR}"
    )
    write_basic_package_version_file("${PROJECT_BINARY_DIR}/imguiConfigVersion.cmake"
        COMPATIBILITY AnyNewerVersion
    )
    install(
        FILES
            "${CMAKE_CURRENT_BINARY_DIR}/imguiConfig.cmake"
            "${CMAKE_CURRENT_BINARY_DIR}/imguiConfigVersion.cmake"
        DESTINATION ${IMGUI_INSTALL_CMAKEDIR}
        COMPONENT devel
    )

    install(EXPORT imguiTargets
        FILE ${imgui_target_name}-targets.cmake
        NAMESPACE imgui::
        DESTINATION "${IMGUI_INSTALL_CMAKEDIR}"
        COMPONENT devel
    )

    export(TARGETS ${imgui_target_name} ${INSTALL_EXTRA_TARGETS} NAMESPACE "imgui::" FILE "${imgui_target_name}-targets.cmake")
endif()
