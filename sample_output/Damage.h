#pragma once
// ============================================================
// SISTEMA DE DAÑO
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
    namespace EntityCpntBasicDamage
    {
        constexpr uintptr_t Constructor = 0x1CB300;

        namespace Offsets
        {
            constexpr size_t vtable                = 0x00;
            constexpr size_t signal_bool_1         = 0x18;
            constexpr size_t signal_bool_2         = 0x38;
            constexpr size_t signal_impact_pos     = 0x1A8; // Vec3 posición impacto
            constexpr size_t signal_damage_dir     = 0x1C8; // Vec3 dirección daño
            constexpr size_t signal_damage_amount  = 0x1E8; // float cantidad daño
            constexpr size_t damage_id             = 0x208;
            constexpr size_t damage_flags          = 0x218; // DWORD (-1)
        }
    }

    namespace DamageSettings
    {
        constexpr uintptr_t Constructor = 0x2E5AC0;
    }
}
