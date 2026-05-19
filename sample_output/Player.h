#pragma once
// ============================================================
// PLAYER (BarkPlayerEntityCpnt)
// ============================================================

namespace SDK::BarkPlayerEntityCpnt {
    constexpr uintptr_t VTable      = 0xF4F778;
    constexpr uintptr_t Constructor = 0x1CCCF0;

    namespace Offsets {
        constexpr size_t vtable                = 0x00;
        constexpr size_t data_ptr_1            = 0x08;
        constexpr size_t data_ptr_2            = 0x10;
        constexpr size_t signal_bool_1         = 0x18;
        constexpr size_t signal_bool_2         = 0x38;
        constexpr size_t state_dword           = 0x60;
        constexpr size_t flags                 = 0x64;
        constexpr size_t fmod_event_3d         = 0x98;
        constexpr size_t entity_3d_locator     = 0x140; // 3D POSITION
        constexpr size_t position_x            = 0x148;
        constexpr size_t position_y            = 0x150;
        constexpr size_t position_z            = 0x158;
    }
}

namespace SDK::HeroControllerNetworkData {
    constexpr uintptr_t Constructor = 0x2457B0;
    constexpr size_t    Size        = 0xD0;
}
