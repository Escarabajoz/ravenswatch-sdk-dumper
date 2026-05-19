#pragma once
// ============================================================
// SISTEMA DE CÁMARA
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
    namespace EntityCpntBasicCamera
    {
        constexpr uintptr_t Constructor = 0x1DCE60;

        namespace Offsets
        {
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
            constexpr size_t position_modifier      = 0xD8;
            constexpr size_t fov                    = 0x12C; // float (60.0f)
            constexpr size_t near_far_modifier      = 0x130;
        }
    }

    namespace EntityCpntTopDownCamera
    {
        constexpr uintptr_t Constructor = 0xC64E0;

        namespace Offsets
        {
            constexpr size_t vtable                = 0x00;
            constexpr size_t camera_locator         = 0x158;
            constexpr size_t pos_x                  = 0x160;
            constexpr size_t pos_y                  = 0x168;
            constexpr size_t pos_z                  = 0x170;
        }
    }

    namespace CameraShaderUniforms
    {
        constexpr uintptr_t SetupFunction = 0x57B940;

        namespace ArrayIndex
        {
            constexpr size_t g_ViewMat      = 80;
            constexpr size_t g_ViewProjMat  = 82;   // MÁS IMPORTANTE PARA W2S
            constexpr size_t g_cameraNearFar = 84;
            constexpr size_t l_InvViewProj  = 12012;
            constexpr size_t l_CameraNearFar = 12014;
        }

        constexpr size_t MatrixDataOffset = 48;
    }

    namespace GameSceneCamera
    {
        constexpr uintptr_t Constructor = 0x6537C0;

        namespace Offsets
        {
            constexpr size_t camera_position   = 0x80;
            constexpr size_t fov               = 0xAC;  // float (60.0f)
            constexpr size_t camera_distance   = 0xB0;  // float (50.0f)
            constexpr size_t far_plane         = 0xB8;  // float (500.0f)
            constexpr size_t view_matrix       = 0xC0;  // Mat4x4
        }
    }
}
