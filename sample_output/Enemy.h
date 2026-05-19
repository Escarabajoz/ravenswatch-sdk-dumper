#pragma once
// ============================================================
// SISTEMA DE ENEMIGOS
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
    namespace EnemyDefinition
    {
        constexpr uintptr_t Constructor = 0x1DB800;
        constexpr size_t    Size        = 848;

        namespace Offsets
        {
            constexpr size_t vtable                = 0x00;
            constexpr size_t enemy_resource_name   = 0x288;
            constexpr size_t state_flag            = 0x2B0;
            constexpr size_t custom_flag_list      = 0x2C0;
            constexpr size_t spawn_value_1         = 0x320; // float (-1.0f)
            constexpr size_t spawn_value_2         = 0x338; // float (-1.0f)
        }
    }

    // Los enemigos usan el mismo sistema ECS:
    // oCEntityCpntBasicMove + 0xE0 -> oCEntity3dLocator (posición)
    // oCEntityCpntGpnTargetComputer + 0x68 -> oCEntity3dLocator (target)
    //
    // Para iterar enemigos:
    // GameScene -> EntitySceneContext -> entity_list (0x2B8/0x2D0/0x2E8)
    //   -> oCGameObject[] -> components_array (0x30)
    //     -> Buscar oCEntity3dLocator -> pos_x, pos_y, pos_z
}
