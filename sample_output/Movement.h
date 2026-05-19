#pragma once
// ============================================================
// SISTEMA DE MOVIMIENTO Y VELOCIDAD
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
    namespace EntityCpntBasicMove
    {
        constexpr uintptr_t Constructor = 0x1DD4B0;

        namespace Offsets
        {
            constexpr size_t vtable                = 0x00;
            constexpr size_t vec3_modifier          = 0x68;
            constexpr size_t entity_3d_locator      = 0xE0;  // POSICIÓN
            constexpr size_t locator_pos_x          = 0xE8;
            constexpr size_t locator_pos_y          = 0xF0;
            constexpr size_t locator_pos_z          = 0xF8;
            constexpr size_t signal_current_pos     = 0x100; // Vec3 posición actual
            constexpr size_t signal_velocity        = 0x120; // Vec3 velocidad
            constexpr size_t signal_target_pos      = 0x140; // Vec3 destino
            constexpr size_t signal_speed           = 0x160; // float velocidad escalar
        }
    }

    namespace EntityCpntGpnTargetComputer
    {
        constexpr uintptr_t Constructor = 0x3C1F90;

        namespace Offsets
        {
            constexpr size_t vtable                = 0x00;
            constexpr size_t target_3d_locator      = 0x68;
            constexpr size_t target_pos_x           = 0x70;
            constexpr size_t target_pos_y           = 0x78;
            constexpr size_t target_pos_z           = 0x80;
            constexpr size_t signal_target_1        = 0xD8;
            constexpr size_t signal_target_2        = 0xF8;
            constexpr size_t signal_computed_target = 0x2C8;
        }
    }

    namespace MovementStrings
    {
        constexpr uintptr_t MoveSpeedRatio = 0xEEF6B0;
        constexpr uintptr_t SpeedRatio     = 0xEEF6B5;
        constexpr uintptr_t AngleSpeed     = 0xF1FB50;
    }
}
