#pragma once
// ============================================================
// SISTEMA DE HABILIDADES / COOLDOWNS
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
    namespace AbilityController
    {
        constexpr uintptr_t Constructor = 0x2B0990;

        namespace Offsets
        {
            constexpr size_t vtable                = 0x00;
            constexpr size_t signal_bool_active    = 0x18;
            constexpr size_t signal_bool_enabled   = 0x38;
            constexpr size_t abilities_array       = 0x70;
            constexpr size_t abilities_count       = 0x78;  // DWORD (32)
            constexpr size_t ability_flags         = 0x98;  // DWORD (0x10000)
            constexpr size_t ability_mode          = 0xD0;  // DWORD (9)
            constexpr size_t signal_cooldown_timer = 0x258; // float timer cooldown
            constexpr size_t signal_available      = 0x278; // bool disponible
            constexpr size_t signal_level          = 0x298; // int nivel
            constexpr size_t signal_is_casting     = 0x2B8; // bool casteando
            constexpr size_t signal_is_ready       = 0x2D8; // bool listo
            constexpr size_t signal_charges        = 0x2F8; // int cargas
            constexpr size_t signal_ability_id     = 0x318; // int ID
        }
    }

    namespace AbilityStrings
    {
        constexpr uintptr_t Cooldown = 0xEF02F5;
    }
}
