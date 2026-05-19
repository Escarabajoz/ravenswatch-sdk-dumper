#pragma once
// ============================================================
// SISTEMA DE ENTIDADES (Entity List)
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
    namespace GameObject
    {
        constexpr uintptr_t Constructor = 0x243C80;

        namespace Offsets
        {
            constexpr size_t vtable_1              = 0x00;
            constexpr size_t flags                 = 0x08;
            constexpr size_t vtable_2              = 0x10;
            constexpr size_t components_array      = 0x30;  // ptr* array componentes
            constexpr size_t components_capacity   = 0x38;
            constexpr size_t components_count      = 0x3C;  // DWORD
            constexpr size_t children_array        = 0x40;
            constexpr size_t children_count        = 0x48;
            constexpr size_t name_ptr              = 0x50;
            constexpr size_t name_length           = 0x58;
            constexpr size_t tag_ptr                = 0x60;
            constexpr size_t object_flags          = 0x70;
        }
    }

    namespace EntitySceneContext
    {
        constexpr uintptr_t Constructor = 0x6E5A40;

        namespace Offsets
        {
            constexpr size_t vtable                = 0x00;
            constexpr size_t entity_spawner        = 0xA0;
            constexpr size_t entity_list_1         = 0x2B8; // Vector
            constexpr size_t entity_list_2         = 0x2D0; // Vector
            constexpr size_t entity_list_3         = 0x2E8; // Vector
            constexpr size_t entity_count          = 0x318; // DWORD
            constexpr size_t entity_limit          = 0x31C; // (1000)
            constexpr size_t entity_descriptors    = 0x4B0;
            constexpr size_t dynamic_entity_array  = 0x4F0;
        }
    }

    namespace GameScene
    {
        constexpr uintptr_t Constructor = 0x463860;

        namespace Offsets
        {
            constexpr size_t vtable                = 0x00;
            constexpr size_t game_scene_base       = 0x220;
            constexpr size_t time_scale            = 0x668; // float (1.0f)
        }
    }

    namespace Entity3dLocator
    {
        namespace Offsets
        {
            constexpr size_t vtable = 0x00;
            constexpr size_t pos_x  = 0x08;
            constexpr size_t pos_y  = 0x10;
            constexpr size_t pos_z  = 0x18;
        }
    }

    namespace EntityAroundPositionTraverser
    {
        constexpr uintptr_t Factory = 0x7ADF80;
        constexpr size_t    Size    = 0x90;

        namespace Offsets
        {
            constexpr size_t vtable          = 0x00;
            constexpr size_t center_locator  = 0x28;
            constexpr size_t center_x        = 0x30;
            constexpr size_t center_y        = 0x38;
            constexpr size_t center_z        = 0x40;
            constexpr size_t search_radius   = 0x88; // float (10.0f)
        }
    }
}
