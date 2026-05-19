#pragma once
// ============================================================
// CAMERA SYSTEM
// ============================================================

namespace SDK::EntityCpntBasicCamera {
    constexpr uintptr_t Constructor = 0x1DCE60;
    constexpr uintptr_t VTable      = 0xF4F778;

    namespace Offsets {
        constexpr size_t vtable                = 0x00;
        constexpr size_t signal_active          = 0x18;
        constexpr size_t camera_pos_locator     = 0x98;  // oCEntity3dLocator
        constexpr size_t camera_pos_x           = 0xA0;
        constexpr size_t camera_pos_y           = 0xA8;
        constexpr size_t camera_pos_z           = 0xB0;
        constexpr size_t camera_target_locator  = 0xB8;  // Look-at target
        constexpr size_t target_x               = 0xC0;
        constexpr size_t target_y               = 0xC8;
        constexpr size_t target_z               = 0xD0;
        constexpr size_t fov                    = 0x12C; // float (60.0f)
        constexpr size_t near_far_modifier      = 0x130;
    }
}

namespace SDK::CameraShaderUniforms {
    constexpr uintptr_t SetupFunction = 0x57B940;

    namespace ArrayIndex {
        constexpr size_t g_ViewMat       = 80;
        constexpr size_t g_ViewProjMat   = 82;  // KEY FOR WorldToScreen
        constexpr size_t g_cameraNearFar = 84;
    }

    constexpr size_t MatrixDataOffset = 48;
}

namespace SDK::GameSceneCamera {
    constexpr uintptr_t Constructor = 0x6537C0;

    namespace Offsets {
        constexpr size_t camera_position = 0x80;
        constexpr size_t fov             = 0xAC;
        constexpr size_t camera_distance = 0xB0;
        constexpr size_t far_plane       = 0xB8;
        constexpr size_t view_matrix     = 0xC0;  // Mat4x4
    }
}
