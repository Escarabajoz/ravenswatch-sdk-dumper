#pragma once
// ============================================================
// SISTEMA DE VIDA / SALUD
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
    namespace NamedEventGainHealth
    {
        constexpr uintptr_t Constructor = 0x27F420;

        namespace Offsets
        {
            constexpr size_t vtable            = 0x00;
            constexpr size_t event_type        = 0x08;  // DWORD (2)
            constexpr size_t event_name        = 0x20;
            constexpr size_t network_id        = 0x38;  // QWORD (-1)
            constexpr size_t health_amount     = 0x50;  // DWORD cantidad de vida
            constexpr size_t health_flags      = 0x54;  // WORD (0x100)
        }
    }

    namespace HealthStrings
    {
        constexpr uintptr_t MaxHealth          = 0xEEF6E0;
        constexpr uintptr_t HealthPercent      = 0xEFBBD8;
        constexpr uintptr_t HealthAbsolute     = 0xEFBAB8;
        constexpr uintptr_t GainHealth         = 0xEE12B8;
    }
}
