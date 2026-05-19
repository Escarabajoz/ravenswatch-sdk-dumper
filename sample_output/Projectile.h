#pragma once
// ============================================================
// SISTEMA DE PROYECTILES
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
    namespace ProjectileAttack
    {
        constexpr uintptr_t Constructor = 0x260A70;

        namespace Offsets
        {
            constexpr size_t vtable                = 0x00;
            constexpr size_t signal_bool_1         = 0x18;
            constexpr size_t signal_bool_2         = 0x38;
            constexpr size_t line_tester            = 0xC0;
            constexpr size_t projectile_width       = 0xE8;  // float (1.0f)
            constexpr size_t tester_combiner_1      = 0x100;
            constexpr size_t tester_combiner_2      = 0x150;
            constexpr size_t signal_projectile_pos  = 0x1A0; // Vec3 posición
            constexpr size_t signal_projectile_dir  = 0x1C0; // Vec3 dirección
            constexpr size_t projectile_state       = 0x1E0;
            constexpr size_t projectile_id          = 0x1E8; // QWORD (-1)
        }
    }

    namespace ProjectileStrings
    {
        constexpr uintptr_t Velocity       = 0xF4C47B;
        constexpr uintptr_t VelocityScale  = 0xF1FBF3;
    }
}
