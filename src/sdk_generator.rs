// ============================================================
// Generador automático de SDK
// Crea archivos .h con los offsets dumpeados
// ============================================================

use crate::dumper::{GameDumper, DumpedOffset, OffsetType};
use std::fs;
use std::io::Write;
use std::path::Path;

/// Un generador de archivo: nombre de salida + función que produce su contenido.
type FileGenerator = (&'static str, fn(&GameDumper) -> String);

/// Genera la carpeta SDK completa
/// Los offsets se guardan como RVA (Relative Virtual Address) para que sean
/// independientes de ASLR. Para obtener la dirección real en runtime:
///   uintptr_t base = (uintptr_t)GetModuleHandle(NULL);
///   uintptr_t addr = base + RVA;
pub fn generate_sdk(output_dir: &str, dumper: &GameDumper) -> Result<usize, String> {
    let path = Path::new(output_dir);
    fs::create_dir_all(path).map_err(|e| format!("No se pudo crear directorio: {}", e))?;

    let mut file_count = 0;

    let generators: Vec<FileGenerator> = vec![
        ("SDK.h", generate_main_header),
        ("Offsets.h", generate_offsets),
        ("Player.h", generate_player),
        ("Camera.h", generate_camera),
        ("Damage.h", generate_damage),
        ("Health.h", generate_health),
        ("Projectile.h", generate_projectile),
        ("Movement.h", generate_movement),
        ("Ability.h", generate_ability),
        ("Entity.h", generate_entity),
        ("Enemy.h", generate_enemy),
        ("Network.h", generate_network),
        ("Multiplayer.h", generate_multiplayer),
        ("Weapon.h", generate_weapon),
    ];

    for (filename, generator) in &generators {
        let content = generator(dumper);
        let filepath = path.join(filename);
        let mut file = fs::File::create(&filepath)
            .map_err(|e| format!("No se pudo crear {}: {}", filename, e))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("No se pudo escribir {}: {}", filename, e))?;

        println!("  [✓] Generado: {}", filename);
        file_count += 1;
    }

    Ok(file_count)
}

fn file_header(title: &str) -> String {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    format!(
        r#"#pragma once
// ============================================================
// {}
// Auto-generado por Game SDK Dumper
// Fecha: {}
// ============================================================
"#,
        title, now
    )
}

fn find_offset<'a>(dumper: &'a GameDumper, name: &str) -> Option<&'a DumpedOffset> {
    dumper.offsets.iter()
        .chain(dumper.vtables.iter())
        .chain(dumper.strings_found.iter())
        .find(|o| o.name == name)
}

fn addr_str(dumper: &GameDumper, name: &str) -> String {
    match find_offset(dumper, name) {
        Some(o) => format!("0x{:X}", o.rva),
        None => "0x0 /* NO ENCONTRADO */".to_string(),
    }
}

// ============================================================
// GENERADORES DE ARCHIVOS INDIVIDUALES
// ============================================================

fn generate_main_header(_dumper: &GameDumper) -> String {
    let mut s = file_header("SDK DEL JUEGO - Archivo Principal");
    s.push_str(r#"
#include "Offsets.h"
#include "Player.h"
#include "Camera.h"
#include "Damage.h"
#include "Health.h"
#include "Projectile.h"
#include "Movement.h"
#include "Ability.h"
#include "Entity.h"
#include "Enemy.h"
#include "Network.h"
#include "Multiplayer.h"
#include "Weapon.h"

namespace SDK
{
    // ============================================================
    // NOTA: Todos los offsets son RVA (Relative Virtual Address)
    // Para obtener la dirección real en runtime:
    //   uintptr_t base = (uintptr_t)GetModuleHandle(NULL);
    //   uintptr_t addr = base + RVA;
    // ============================================================

    inline uintptr_t GetBase() { return (uintptr_t)GetModuleHandleA(NULL); }
    inline uintptr_t Resolve(uintptr_t rva) { return GetBase() + rva; }

    struct Vec3 { float x, y, z; };
    struct Vec4 { float x, y, z, w; };
    struct Mat4x4 { float m[4][4]; };

    // ============================================================
    // RESUMEN DE OFFSETS CLAVE
    // ============================================================
    //
    // === POSICIÓN DEL JUGADOR ===
    // BarkPlayerEntityCpnt + 0x140 -> oCEntity3dLocator
    //   +0x148 -> X, +0x150 -> Y, +0x158 -> Z
    //
    // === POSICIÓN DE ENEMIGOS ===
    // oCEntityCpntBasicMove + 0xE0 -> oCEntity3dLocator
    //   +0xE8 -> X, +0xF0 -> Y, +0xF8 -> Z
    //
    // === CÁMARA ===
    // oCEntityCpntBasicCamera + 0x98 -> Posición cámara
    // oCEntityCpntBasicCamera + 0xB8 -> Look-at target
    // oCEntityCpntBasicCamera + 0x12C -> FOV (60.0f)
    // g_ViewProjMat -> renderer[82], matrix en +48
    //
    // === DAÑO ===
    // oCEntityCpntBasicDamage + 0x1A8 -> Posición impacto (Vec3)
    // oCEntityCpntBasicDamage + 0x1C8 -> Dirección daño (Vec3)
    // oCEntityCpntBasicDamage + 0x1E8 -> Cantidad daño (float)
    //
    // === HABILIDADES / COOLDOWNS ===
    // AbilityController + 0x258 -> Timer cooldown (float)
    // AbilityController + 0x278 -> Disponible (bool)
    // AbilityController + 0x2F8 -> Cargas (int)
    //
    // === ENTITY LIST ===
    // oCEntitySceneContext + 0x2B8 -> Lista entidades 1
    // oCEntitySceneContext + 0x2D0 -> Lista entidades 2
    // oCEntitySceneContext + 0x2E8 -> Lista entidades 3
    //
    // === WORLD TO SCREEN ===
    // 1. clipPos = ViewProjMat * Vec4(x, y, z, 1)
    // 2. ndcX = clipPos.x / clipPos.w
    // 3. ndcY = clipPos.y / clipPos.w
    // 4. screenX = (ndcX + 1) * 0.5 * width
    // 5. screenY = (1 - ndcY) * 0.5 * height
}
"#);
    s
}

fn generate_offsets(dumper: &GameDumper) -> String {
    let mut s = file_header("OFFSETS COMPLETOS");
    s.push_str("\nnamespace SDK\n{\n");

    // VTables
    s.push_str("    namespace VTables\n    {\n");
    for cat in dumper.get_categories() {
        let items: Vec<&DumpedOffset> = dumper.get_by_category(&cat)
            .into_iter()
            .filter(|o| matches!(o.offset_type, OffsetType::VTable))
            .collect();
        if items.is_empty() { continue; }
        s.push_str(&format!("        // --- {} ---\n", cat));
        for item in items {
            s.push_str(&format!(
                "        constexpr uintptr_t {:<45} = 0x{:X};\n",
                item.name, item.rva
            ));
        }
    }
    s.push_str("    }\n\n");

    // Functions
    s.push_str("    namespace Functions\n    {\n");
    for cat in dumper.get_categories() {
        let items: Vec<&DumpedOffset> = dumper.get_by_category(&cat)
            .into_iter()
            .filter(|o| matches!(o.offset_type, OffsetType::Function))
            .collect();
        if items.is_empty() { continue; }
        s.push_str(&format!("        // --- {} ---\n", cat));
        for item in items {
            s.push_str(&format!(
                "        constexpr uintptr_t {:<45} = 0x{:X}; // {}\n",
                item.name, item.rva, item.description
            ));
        }
    }
    s.push_str("    }\n\n");

    // Strings
    s.push_str("    namespace Strings\n    {\n");
    for cat in dumper.get_categories() {
        let items: Vec<&DumpedOffset> = dumper.get_by_category(&cat)
            .into_iter()
            .filter(|o| matches!(o.offset_type, OffsetType::StringRef))
            .collect();
        if items.is_empty() { continue; }
        s.push_str(&format!("        // --- {} ---\n", cat));
        for item in items {
            s.push_str(&format!(
                "        constexpr uintptr_t {:<45} = 0x{:X}; // \"{}\"\n",
                item.name, item.rva, item.description
            ));
        }
    }
    s.push_str("    }\n");

    s.push_str("}\n");
    s
}

fn generate_player(dumper: &GameDumper) -> String {
    let mut s = file_header("JUGADOR (BarkPlayerEntityCpnt)");
    s.push_str(&format!(r#"
namespace SDK
{{
    namespace BarkPlayerEntityCpnt
    {{
        constexpr uintptr_t VTable      = {};
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
            constexpr size_t vtable                = 0x00;
            constexpr size_t data_ptr_1            = 0x08;
            constexpr size_t data_ptr_2            = 0x10;
            constexpr size_t signal_bool_1         = 0x18;
            constexpr size_t signal_bool_2         = 0x38;
            constexpr size_t state_dword           = 0x60;
            constexpr size_t flags                 = 0x64;
            constexpr size_t fmod_event_3d         = 0x98;
            constexpr size_t entity_3d_locator     = 0x140; // POSICIÓN 3D
            constexpr size_t position_x            = 0x148;
            constexpr size_t position_y            = 0x150;
            constexpr size_t position_z            = 0x158;
        }}
    }}

    namespace HeroControllerNetworkData
    {{
        constexpr uintptr_t Constructor = {};
        constexpr size_t    Size        = 0xD0;
    }}
}}
"#,
        addr_str(dumper, "BarkPlayerEntityCpnt_VTable"),
        addr_str(dumper, "BarkPlayerEntityCpnt::Ctor"),
        addr_str(dumper, "oCHeroControllerNetworkData::Ctor"),
    ));
    s
}

fn generate_camera(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA DE CÁMARA");
    s.push_str(&format!(r#"
namespace SDK
{{
    namespace EntityCpntBasicCamera
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
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
        }}
    }}

    namespace EntityCpntTopDownCamera
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
            constexpr size_t vtable                = 0x00;
            constexpr size_t camera_locator         = 0x158;
            constexpr size_t pos_x                  = 0x160;
            constexpr size_t pos_y                  = 0x168;
            constexpr size_t pos_z                  = 0x170;
        }}
    }}

    namespace CameraShaderUniforms
    {{
        constexpr uintptr_t SetupFunction = {};

        namespace ArrayIndex
        {{
            constexpr size_t g_ViewMat      = 80;
            constexpr size_t g_ViewProjMat  = 82;   // MÁS IMPORTANTE PARA W2S
            constexpr size_t g_cameraNearFar = 84;
            constexpr size_t l_InvViewProj  = 12012;
            constexpr size_t l_CameraNearFar = 12014;
        }}

        constexpr size_t MatrixDataOffset = 48;
    }}

    namespace GameSceneCamera
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
            constexpr size_t camera_position   = 0x80;
            constexpr size_t fov               = 0xAC;  // float (60.0f)
            constexpr size_t camera_distance   = 0xB0;  // float (50.0f)
            constexpr size_t far_plane         = 0xB8;  // float (500.0f)
            constexpr size_t view_matrix       = 0xC0;  // Mat4x4
        }}
    }}
}}
"#,
        addr_str(dumper, "oCEntityCpntBasicCamera::Ctor"),
        addr_str(dumper, "oCEntityCpntTopDownCamera::Ctor"),
        addr_str(dumper, "SetupCameraShaderUniforms"),
        addr_str(dumper, "oIGameScene::Ctor"),
    ));
    s
}

fn generate_damage(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA DE DAÑO");
    s.push_str(&format!(r#"
namespace SDK
{{
    namespace EntityCpntBasicDamage
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
            constexpr size_t vtable                = 0x00;
            constexpr size_t signal_bool_1         = 0x18;
            constexpr size_t signal_bool_2         = 0x38;
            constexpr size_t signal_impact_pos     = 0x1A8; // Vec3 posición impacto
            constexpr size_t signal_damage_dir     = 0x1C8; // Vec3 dirección daño
            constexpr size_t signal_damage_amount  = 0x1E8; // float cantidad daño
            constexpr size_t damage_id             = 0x208;
            constexpr size_t damage_flags          = 0x218; // DWORD (-1)
        }}
    }}

    namespace DamageSettings
    {{
        constexpr uintptr_t Constructor = {};
    }}
}}
"#,
        addr_str(dumper, "oCEntityCpntBasicDamage::Ctor"),
        addr_str(dumper, "oCDtEntityCpntDamageSettings::Ctor"),
    ));
    s
}

fn generate_health(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA DE VIDA / SALUD");
    s.push_str(&format!(r#"
namespace SDK
{{
    namespace NamedEventGainHealth
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
            constexpr size_t vtable            = 0x00;
            constexpr size_t event_type        = 0x08;  // DWORD (2)
            constexpr size_t event_name        = 0x20;
            constexpr size_t network_id        = 0x38;  // QWORD (-1)
            constexpr size_t health_amount     = 0x50;  // DWORD cantidad de vida
            constexpr size_t health_flags      = 0x54;  // WORD (0x100)
        }}
    }}

    namespace HealthStrings
    {{
        constexpr uintptr_t MaxHealth          = {};
        constexpr uintptr_t HealthPercent      = {};
        constexpr uintptr_t HealthAbsolute     = {};
        constexpr uintptr_t GainHealth         = {};
    }}
}}
"#,
        addr_str(dumper, "oCNamedEventGainHealth::Ctor"),
        addr_str(dumper, "MaxHealth"),
        addr_str(dumper, "HealthPercent"),
        addr_str(dumper, "HealthAbsolute"),
        addr_str(dumper, "GainHealth"),
    ));
    s
}

fn generate_projectile(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA DE PROYECTILES");
    s.push_str(&format!(r#"
namespace SDK
{{
    namespace ProjectileAttack
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
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
        }}
    }}

    namespace ProjectileStrings
    {{
        constexpr uintptr_t Velocity       = {};
        constexpr uintptr_t VelocityScale  = {};
    }}
}}
"#,
        addr_str(dumper, "oCEntityCpntGpnProjectileAttack::Ctor"),
        addr_str(dumper, "Velocity"),
        addr_str(dumper, "VelocityScale"),
    ));
    s
}

fn generate_movement(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA DE MOVIMIENTO Y VELOCIDAD");
    s.push_str(&format!(r#"
namespace SDK
{{
    namespace EntityCpntBasicMove
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
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
        }}
    }}

    namespace EntityCpntGpnTargetComputer
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
            constexpr size_t vtable                = 0x00;
            constexpr size_t target_3d_locator      = 0x68;
            constexpr size_t target_pos_x           = 0x70;
            constexpr size_t target_pos_y           = 0x78;
            constexpr size_t target_pos_z           = 0x80;
            constexpr size_t signal_target_1        = 0xD8;
            constexpr size_t signal_target_2        = 0xF8;
            constexpr size_t signal_computed_target = 0x2C8;
        }}
    }}

    namespace MovementStrings
    {{
        constexpr uintptr_t MoveSpeedRatio = {};
        constexpr uintptr_t SpeedRatio     = {};
        constexpr uintptr_t AngleSpeed     = {};
    }}
}}
"#,
        addr_str(dumper, "oCEntityCpntBasicMove::Ctor"),
        addr_str(dumper, "oCEntityCpntGpnTargetComputer::Ctor"),
        addr_str(dumper, "MoveSpeedRatio"),
        addr_str(dumper, "SpeedRatio"),
        addr_str(dumper, "AngleSpeed"),
    ));
    s
}

fn generate_ability(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA DE HABILIDADES / COOLDOWNS");
    s.push_str(&format!(r#"
namespace SDK
{{
    namespace AbilityController
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
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
        }}
    }}

    namespace AbilityStrings
    {{
        constexpr uintptr_t Cooldown = {};
    }}
}}
"#,
        addr_str(dumper, "oCDtEntityCpntAbilityController::Ctor"),
        addr_str(dumper, "Cooldown"),
    ));
    s
}

fn generate_entity(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA DE ENTIDADES (Entity List)");
    s.push_str(&format!(r#"
namespace SDK
{{
    namespace GameObject
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
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
        }}
    }}

    namespace EntitySceneContext
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
            constexpr size_t vtable                = 0x00;
            constexpr size_t entity_spawner        = 0xA0;
            constexpr size_t entity_list_1         = 0x2B8; // Vector
            constexpr size_t entity_list_2         = 0x2D0; // Vector
            constexpr size_t entity_list_3         = 0x2E8; // Vector
            constexpr size_t entity_count          = 0x318; // DWORD
            constexpr size_t entity_limit          = 0x31C; // (1000)
            constexpr size_t entity_descriptors    = 0x4B0;
            constexpr size_t dynamic_entity_array  = 0x4F0;
        }}
    }}

    namespace GameScene
    {{
        constexpr uintptr_t Constructor = {};

        namespace Offsets
        {{
            constexpr size_t vtable                = 0x00;
            constexpr size_t game_scene_base       = 0x220;
            constexpr size_t time_scale            = 0x668; // float (1.0f)
        }}
    }}

    namespace Entity3dLocator
    {{
        namespace Offsets
        {{
            constexpr size_t vtable = 0x00;
            constexpr size_t pos_x  = 0x08;
            constexpr size_t pos_y  = 0x10;
            constexpr size_t pos_z  = 0x18;
        }}
    }}

    namespace EntityAroundPositionTraverser
    {{
        constexpr uintptr_t Factory = {};
        constexpr size_t    Size    = 0x90;

        namespace Offsets
        {{
            constexpr size_t vtable          = 0x00;
            constexpr size_t center_locator  = 0x28;
            constexpr size_t center_x        = 0x30;
            constexpr size_t center_y        = 0x38;
            constexpr size_t center_z        = 0x40;
            constexpr size_t search_radius   = 0x88; // float (10.0f)
        }}
    }}
}}
"#,
        addr_str(dumper, "oCGameObject::Ctor"),
        addr_str(dumper, "oCEntitySceneContext::Ctor"),
        addr_str(dumper, "oCGameScene::Ctor"),
        addr_str(dumper, "oCEntityGpnAroundPositionTraverser::Factory"),
    ));
    s
}

fn generate_enemy(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA DE ENEMIGOS");
    s.push_str(&format!(r#"
namespace SDK
{{
    namespace EnemyDefinition
    {{
        constexpr uintptr_t Constructor = {};
        constexpr size_t    Size        = 848;

        namespace Offsets
        {{
            constexpr size_t vtable                = 0x00;
            constexpr size_t enemy_resource_name   = 0x288;
            constexpr size_t state_flag            = 0x2B0;
            constexpr size_t custom_flag_list      = 0x2C0;
            constexpr size_t spawn_value_1         = 0x320; // float (-1.0f)
            constexpr size_t spawn_value_2         = 0x338; // float (-1.0f)
        }}
    }}

    // Los enemigos usan el mismo sistema ECS:
    // oCEntityCpntBasicMove + 0xE0 -> oCEntity3dLocator (posición)
    // oCEntityCpntGpnTargetComputer + 0x68 -> oCEntity3dLocator (target)
    //
    // Para iterar enemigos:
    // GameScene -> EntitySceneContext -> entity_list (0x2B8/0x2D0/0x2E8)
    //   -> oCGameObject[] -> components_array (0x30)
    //     -> Buscar oCEntity3dLocator -> pos_x, pos_y, pos_z
}}
"#,
        addr_str(dumper, "oCDtEnemyDefinition::Ctor"),
    ));
    s
}

fn generate_network(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA DE RED / NETWORK / ESTADÍSTICAS");
    s.push_str(&format!(r#"
namespace SDK
{{
    // ============================================================
    // STACK DE RED DEL JUEGO
    // ============================================================
    // Backend:     Stormancer (servidor de juegos)
    // P2P:         Epic Online Services (EOS P2P)
    // Sockets:     RakNet (RakPeer, RakNetSocket2)
    // Auth:        Steam + EOS Connect
    // HTTP:        WinHTTP / WinInet
    // ============================================================

    // ============================================================
    // oCEntityCpntNetwork - Componente de red de entidad
    // Constructor: {ctor_network}
    // ============================================================
    namespace EntityCpntNetwork
    {{
        constexpr uintptr_t Constructor = {ctor_network};
    }}

    // ============================================================
    // oCEntityCpntNetworkSettings
    // Constructor: {ctor_settings}
    // ============================================================
    namespace EntityCpntNetworkSettings
    {{
        constexpr uintptr_t Constructor = {ctor_settings};
    }}

    // ============================================================
    // oCEntityCpntHitPointNetworkData
    // Constructor: {ctor_hitpoint}
    // ============================================================
    namespace HitPointNetworkData
    {{
        constexpr uintptr_t Constructor = {ctor_hitpoint};
    }}

    // ============================================================
    // oCEntityCpntValueNetworkData
    // Constructor: {ctor_value}
    // ============================================================
    namespace ValueNetworkData
    {{
        constexpr uintptr_t Constructor = {ctor_value};
    }}

    // ============================================================
    // oCGlobalEntityValueNetworkData
    // Factory: {factory_global}
    // ============================================================
    namespace GlobalEntityValueNetworkData
    {{
        constexpr uintptr_t Factory = {factory_global};
    }}

    // ============================================================
    // URLs de Servidores
    // ============================================================
    namespace ServerUrls
    {{
        constexpr uintptr_t Live      = {url_live};    // "https://dt-live.passtechgames.com"
        constexpr uintptr_t Dev       = {url_dev};     // "http://dt-dev.passtechgames.com:8888"
        constexpr uintptr_t Live2     = {url_live2};   // "https://dt-live-2.passtechgames.com"
        constexpr uintptr_t Live3     = {url_live3};   // "https://dt-live-3.passtechgames.com"
        constexpr uintptr_t Localhost  = {url_local};   // "http://localhost"
    }}

    // ============================================================
    // VTables de Red conocidas (estáticas)
    // ============================================================
    namespace NetworkVTables
    {{
        constexpr uintptr_t oINetworkData                     = 0xEE65D8;
        constexpr uintptr_t oINetworkMessage                  = 0xEE8650;
        constexpr uintptr_t oIEntityCpntNetworkData           = 0xEE5538;
        constexpr uintptr_t oCNetworkData                     = 0xF52DD0;
        constexpr uintptr_t oCEntityCpntNetwork               = 0xF52778;
        constexpr uintptr_t oCEntityCpntNetworkSettings       = 0xF52E68;
        constexpr uintptr_t oCEntityCpntNetworkNetworkData    = 0xEECFC8;
        constexpr uintptr_t oCEntityCpntHitPointNetworkData   = 0xF00E88;
        constexpr uintptr_t oCEntityCpntValueNetworkData      = 0xF00958;
        constexpr uintptr_t oCGlobalEntityValueNetworkData    = 0xF4F900;
        constexpr uintptr_t oCGameNamedEventNetwork           = 0xEF2D28;
        constexpr uintptr_t oCGameNamedEventNetworkDamage     = 0xEEAED0;
        constexpr uintptr_t oCGameNamedEventNetworkDamageResp = 0xEE7430;
        constexpr uintptr_t oCNamedEventNetworkWithCpntId     = 0xEE4278;
        constexpr uintptr_t oCNamedEventNetworkWithData       = 0xEE8890;
        constexpr uintptr_t oCGameEventNetworkModifier        = 0xF0C168;
        constexpr uintptr_t oCDtEntityCpntHeroCtrlNetData     = 0xEEA6F8;
        constexpr uintptr_t oCDtEntityCpntCharCtrlNetData     = 0xEE8D80;
        constexpr uintptr_t oCDtEntityCpntTeleporterNetData   = 0xEE8230;
        constexpr uintptr_t oCDtEntityCpntAbilityCtrlNetData  = 0xEFF200;
        constexpr uintptr_t oCDtEntityCpntInteractionNetData  = 0xEFFFF8;
        constexpr uintptr_t oCDtEntityCpntGroupLevelNetData   = 0xEEB760;
        constexpr uintptr_t oCDtEntityCpntHittableListNetData = 0xF0CDA0;
        constexpr uintptr_t oCEntityCpntTimerNetworkData      = 0xF520C8;
        constexpr uintptr_t oCEntityCpntModifierHolderNetData = 0xEE5950;
        constexpr uintptr_t StackableValueCpntNetworkData     = 0xF4CD30;
        constexpr uintptr_t EntityListenerEntityCpntNetData   = 0xF4D818;
        constexpr uintptr_t oCCycleNetworkData_DayNight       = 0xEEA778;

        // RakNet
        constexpr uintptr_t RakPeerInterface                  = 0xF90D48;
        constexpr uintptr_t RakPeer                           = 0xF91030;
        constexpr uintptr_t RakNetSocket2                     = 0xF90008;
    }}

    // ============================================================
    // Imports de red (IAT)
    // ============================================================
    namespace NetworkImports
    {{
        constexpr uintptr_t EOS_Platform_GetNetworkStatus     = 0xE93228;
        constexpr uintptr_t EOS_Platform_SetNetworkStatus     = 0xE93328;
        constexpr uintptr_t EOS_P2P_AcceptConnection          = 0xE931D0;
        constexpr uintptr_t EOS_P2P_CloseConnection           = 0xE931F8;
        constexpr uintptr_t EOS_P2P_SendPacket                = 0xE93278;
        constexpr uintptr_t EOS_P2P_ReceivePacket             = 0xE932C0;
        constexpr uintptr_t EOS_Connect_Login                 = 0xE93270;
        constexpr uintptr_t EOS_Connect_Logout                = 0xE93180;
        constexpr uintptr_t EOS_Connect_CreateUser            = 0xE930C8;
        constexpr uintptr_t WinHttpConnect                    = 0xE93CE0;
        constexpr uintptr_t InternetConnectA                  = 0xE93D90;
        constexpr uintptr_t closesocket                       = 0xE93DC0;
        constexpr uintptr_t ioctlsocket                       = 0xE93DD0;
        constexpr uintptr_t socket_fn                         = 0xE93E30;
    }}

    // ============================================================
    // Strings de Red
    // ============================================================
    namespace NetworkStrings
    {{
        constexpr uintptr_t NetworkStats           = {s_netstats};
        constexpr uintptr_t NetworkStatsUI         = {s_netstatsui};
        constexpr uintptr_t NetworkSessionSize     = {s_sesssize};
        constexpr uintptr_t NetworkSettings        = {s_netsettings};
        constexpr uintptr_t IsConnectedToSession   = {s_connected};
        constexpr uintptr_t NoConnection           = {s_noconn};
        constexpr uintptr_t LoginFailed            = {s_loginfail};
        constexpr uintptr_t SessionId              = {s_sessid};
    }}

    // ============================================================
    // Strings de Estadísticas / Analytics
    // ============================================================
    namespace StatisticsStrings
    {{
        constexpr uintptr_t Statistics             = {s_stats};
        constexpr uintptr_t StatisticsCollection   = {s_statscoll};
        constexpr uintptr_t StatisticsSample       = {s_statssamp};
        constexpr uintptr_t StatisticsCollector    = {s_statscolr};
        constexpr uintptr_t StatsLogsEnable        = {s_statslogs};
        constexpr uintptr_t StatsLogInterval       = {s_statsint};
        constexpr uintptr_t Analytics              = {s_analytics};
    }}
}}
"#,
        ctor_network   = addr_str(dumper, "oCEntityCpntNetwork::Ctor"),
        ctor_settings  = addr_str(dumper, "oCEntityCpntNetworkSettings::Ctor"),
        ctor_hitpoint  = addr_str(dumper, "oCEntityCpntHitPointNetworkData::Ctor"),
        ctor_value     = addr_str(dumper, "oCEntityCpntValueNetworkData::Ctor"),
        factory_global = addr_str(dumper, "oCGlobalEntityValueNetworkData::Factory"),
        url_live       = addr_str(dumper, "ServerUrlLive"),
        url_dev        = addr_str(dumper, "ServerUrlDev"),
        url_live2      = addr_str(dumper, "ServerUrlLive2"),
        url_live3      = addr_str(dumper, "ServerUrlLive3"),
        url_local      = addr_str(dumper, "ServerUrlLocalhost"),
        s_netstats     = addr_str(dumper, "NetworkStats"),
        s_netstatsui   = addr_str(dumper, "NetworkStatsUI"),
        s_sesssize     = addr_str(dumper, "NetworkSessionSize"),
        s_netsettings  = addr_str(dumper, "NetworkSettings"),
        s_connected    = addr_str(dumper, "IsConnectedToSession"),
        s_noconn       = addr_str(dumper, "NoConnection"),
        s_loginfail    = addr_str(dumper, "LoginFailed"),
        s_sessid       = addr_str(dumper, "SessionId"),
        s_stats        = addr_str(dumper, "Statistics"),
        s_statscoll    = addr_str(dumper, "StatisticsCollection"),
        s_statssamp    = addr_str(dumper, "StatisticsSample"),
        s_statscolr    = addr_str(dumper, "StatisticsCollector"),
        s_statslogs    = addr_str(dumper, "StatsLogsEnable"),
        s_statsint     = addr_str(dumper, "StatsLogInterval"),
        s_analytics    = addr_str(dumper, "Analytics"),
    ));
    s
}

fn generate_multiplayer(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA MULTIJUGADOR / SESIONES / P2P / PARTY");
    s.push_str(&format!(r#"
namespace SDK
{{
    // ============================================================
    // ARQUITECTURA MULTIJUGADOR
    // ============================================================
    // Backend:        Stormancer (sesiones, matchmaking, party)
    // P2P Transport:  EOS P2P (Epic Online Services)
    // Low-level:      RakNet (RakPeer, sockets)
    // Auth:           Steam + EOS Connect
    //
    // Flujo de conexión:
    // 1. Login (Steam/EOS) -> Stormancer Auth
    // 2. CreateParty / JoinParty
    // 3. StartMatchmaking -> CreateSession
    // 4. ConnectToScene -> P2P handshake
    // 5. EntityReplication + NetworkData sync
    // ============================================================

    // ============================================================
    // Funciones Principales
    // ============================================================
    namespace MultiplayerFunctions
    {{
        constexpr uintptr_t StormancerMainHandler  = {fn_storm};   // Handler principal de escena Stormancer
        constexpr uintptr_t ConnectToScene          = {fn_connect}; // Conexión a escena/servidor
    }}

    // ============================================================
    // VTables Multiplayer (estáticas)
    // ============================================================
    namespace MultiplayerVTables
    {{
        // Stormancer
        constexpr uintptr_t oCStormancerSceneContext         = 0xEB0CA8;
        constexpr uintptr_t oCDtStormancerSteamSceneContext  = 0xEB0CA8;

        // Matchmaking
        constexpr uintptr_t oCMatchmakingOptionsSettings     = 0xEF3C40;

        // Session / Replication
        constexpr uintptr_t oCEntityReplicationCpnt          = 0xF52778;
        constexpr uintptr_t oCGameNamedEventNetwork          = 0xEF2D28;
    }}

    // ============================================================
    // Strings de Sesiones
    // ============================================================
    namespace SessionStrings
    {{
        constexpr uintptr_t MatchmakingStart       = {s_mm_start};
        constexpr uintptr_t MatchmakingEnd         = {s_mm_end};
        constexpr uintptr_t Matchmaking            = {s_mm};
        constexpr uintptr_t MatchmakingOptions     = {s_mm_opts};
        constexpr uintptr_t SessionEnd             = {s_sess_end};
        constexpr uintptr_t IsSessionHost          = {s_is_host};
        constexpr uintptr_t GameSessionSize        = {s_sess_size};
        constexpr uintptr_t GameSessionInfos       = {s_sess_infos};
        constexpr uintptr_t GameSession            = {s_game_sess};
        constexpr uintptr_t P2PSessionCreateFailed = {s_p2p_create_fail};
        constexpr uintptr_t P2PSessionJoinFailed   = {s_p2p_join_fail};
        constexpr uintptr_t SessionFromHost        = {s_sess_host};
        constexpr uintptr_t MultiplayerConnection  = {s_mp_conn};
        constexpr uintptr_t MultiplayerScore       = {s_mp_score};
        constexpr uintptr_t MultiplayerPng         = {s_mp_png};
    }}

    // ============================================================
    // Strings de Party / Lobby
    // ============================================================
    namespace PartyStrings
    {{
        constexpr uintptr_t CreateLobby            = {s_create_lobby};
        constexpr uintptr_t JoinLobby              = {s_join_lobby};
        constexpr uintptr_t Lobby                  = {s_lobby};
        constexpr uintptr_t Invite                 = {s_invite};
        constexpr uintptr_t PartyId                = {s_party_id};
        constexpr uintptr_t PartyDataToken         = {s_party_token};
        constexpr uintptr_t PartySceneIsNull       = {s_party_null};
        constexpr uintptr_t PartyDataBearerTokens  = {s_party_bearer};
        constexpr uintptr_t CreatePartyDataToken   = {s_party_create_tok};
    }}

    // ============================================================
    // Strings de P2P
    // ============================================================
    namespace P2PStrings
    {{
        constexpr uintptr_t P2PSessionSceneContext = {s_p2p_ctx};
        constexpr uintptr_t SendingJoinRequest     = {s_send_join};
    }}

    // ============================================================
    // Strings de UI Multiplayer
    // ============================================================
    namespace MultiplayerUiStrings
    {{
        constexpr uintptr_t ModalUiController      = {s_modal_ui};
        constexpr uintptr_t ModalUiSettings        = {s_modal_settings};
    }}

    // ============================================================
    // Strings de Replicación
    // ============================================================
    namespace ReplicationStrings
    {{
        constexpr uintptr_t ReplicationManagerScene    = {s_repl_mgr};
        constexpr uintptr_t ReplicationManagerUpdate   = {s_repl_update};
        constexpr uintptr_t ReplicationInterval        = {s_repl_interval};
        constexpr uintptr_t ReplicationPeriod          = {s_repl_period};
        constexpr uintptr_t ReplicationManagerSceneCtx = {s_repl_ctx};
        constexpr uintptr_t ReplicationManagerSceneLow = {s_repl_low};
    }}
}}
"#,
        fn_storm           = addr_str(dumper, "oCStormancerSceneContext::MainHandler"),
        fn_connect         = addr_str(dumper, "ConnectToStormancerScene"),
        s_mm_start         = addr_str(dumper, "MatchmakingStart"),
        s_mm_end           = addr_str(dumper, "MatchmakingEnd"),
        s_mm               = addr_str(dumper, "Matchmaking"),
        s_mm_opts          = addr_str(dumper, "MatchmakingOptions"),
        s_sess_end         = addr_str(dumper, "SessionEnd"),
        s_is_host          = addr_str(dumper, "IsSessionHost"),
        s_sess_size        = addr_str(dumper, "GameSessionSize"),
        s_sess_infos       = addr_str(dumper, "GameSessionInfos"),
        s_game_sess        = addr_str(dumper, "GameSession"),
        s_p2p_create_fail  = addr_str(dumper, "P2PSessionCreationFailed"),
        s_p2p_join_fail    = addr_str(dumper, "P2PSessionJoinFailed"),
        s_sess_host        = addr_str(dumper, "SessionFromHost"),
        s_mp_conn          = addr_str(dumper, "MultiplayerConnection"),
        s_mp_score         = addr_str(dumper, "MultiplayerScore"),
        s_mp_png           = addr_str(dumper, "MultiplayerPng"),
        s_create_lobby     = addr_str(dumper, "CreateLobby"),
        s_join_lobby       = addr_str(dumper, "JoinLobby"),
        s_lobby            = addr_str(dumper, "LobbyStr"),
        s_invite           = addr_str(dumper, "Invite"),
        s_party_id         = addr_str(dumper, "PartyId"),
        s_party_token      = addr_str(dumper, "PartyDataToken"),
        s_party_null       = addr_str(dumper, "PartySceneIsNull"),
        s_party_bearer     = addr_str(dumper, "PartyDataBearerTokens"),
        s_party_create_tok = addr_str(dumper, "CreatePartyDataToken"),
        s_p2p_ctx          = addr_str(dumper, "P2PSessionSceneContext"),
        s_send_join        = addr_str(dumper, "SendingSessionJoinRequest"),
        s_modal_ui         = addr_str(dumper, "MultiplayerModalUiCtrl"),
        s_modal_settings   = addr_str(dumper, "MultiplayerModalUiSettings"),
        s_repl_mgr         = addr_str(dumper, "ReplicationManagerSceneCtx"),
        s_repl_update      = addr_str(dumper, "ReplicationManagerCtxUpdate"),
        s_repl_interval    = addr_str(dumper, "ReplicationInterval"),
        s_repl_period      = addr_str(dumper, "ReplicationPeriod"),
        s_repl_ctx         = addr_str(dumper, "ReplicationManagerScene"),
        s_repl_low         = addr_str(dumper, "ReplicationManagerSceneStr"),
    ));
    s
}

fn generate_weapon(dumper: &GameDumper) -> String {
    let mut s = file_header("SISTEMA DE ARMAS / ATAQUES / ITEMS / INVENTARIO");
    s.push_str(&format!(r#"
namespace SDK
{{
    // ============================================================
    // SISTEMA DE ATAQUES
    // ============================================================
    // El juego usa un sistema de componentes para ataques:
    // - oIDtEntityCpntAttack (interfaz base)
    //   ├─ oCDtEntityCpntAttackAim (apuntado)
    //   ├─ oCDtEntityCpntChargeAttack (carga)
    //   ├─ oCDtEntityCpntAttackCombo (combos)
    //   ├─ oCDtEntityCpntContinuousAttack (continuo)
    //   ├─ oCEntityCpntZoneAttack (AoE)
    //   ├─ oCEntityCpntGpnTargetAttack (dirigido)
    //   └─ oCEntityCpntGpnProjectileAttack (proyectil)

    // ============================================================
    // Constructores de Ataques
    // ============================================================
    namespace AttackBase
    {{
        constexpr uintptr_t Constructor          = {fn_attack};     // oIDtEntityCpntAttack
        constexpr uintptr_t SettingsConstructor   = {fn_attack_set}; // oIDtEntityCpntAttackSettings
    }}

    namespace AttackAim
    {{
        constexpr uintptr_t Constructor = {fn_aim};     // oCDtEntityCpntAttackAim
    }}

    namespace ChargeAttack
    {{
        constexpr uintptr_t Constructor = {fn_charge};  // oCDtEntityCpntChargeAttack
    }}

    namespace AttackCombo
    {{
        constexpr uintptr_t Constructor = {fn_combo};   // oCDtEntityCpntAttackCombo
    }}

    namespace ZoneAttack
    {{
        constexpr uintptr_t Constructor = {fn_zone};    // oCEntityCpntZoneAttack (AoE)
    }}

    namespace TargetAttack
    {{
        constexpr uintptr_t Constructor = {fn_target};  // oCEntityCpntGpnTargetAttack
    }}

    // ============================================================
    // VTables de Ataques (estáticas)
    // ============================================================
    namespace AttackVTables
    {{
        constexpr uintptr_t oIDtEntityCpntAttack                  = 0xF0CF78;
        constexpr uintptr_t oIDtEntityCpntAttackSettings          = 0xF0D4F0;
        constexpr uintptr_t oCDtEntityCpntAttackAim               = 0xF0A9E8;
        constexpr uintptr_t oCDtEntityCpntAttackAimSettings       = 0xF0AF58;
        constexpr uintptr_t oCDtEntityCpntChargeAttack            = 0xF0EE00;
        constexpr uintptr_t oCDtEntityCpntChargeAttackSettings    = 0xF12350;
        constexpr uintptr_t oCDtChargeAttackStepSettings          = 0xF0F9A0;
        constexpr uintptr_t oCDtEntityCpntAttackCombo             = 0xF0F648;
        constexpr uintptr_t oCDtEntityCpntAttackComboSettings     = 0xF0F0A8;
        constexpr uintptr_t oCDtAttackComboStepSettings           = 0xF11B00;
        constexpr uintptr_t oCDtAttackComboStep                   = 0xF12F98;
        constexpr uintptr_t oCDtEntityCpntContinuousAttack        = 0xF113F0;
        constexpr uintptr_t oCDtEntityCpntContinuousAttackSettings = 0xF10008;
        constexpr uintptr_t oCEntityCpntZoneAttack                = 0xF4E458;
        constexpr uintptr_t oCEntityCpntGpnTargetAttack           = 0xF4E6C8;
        constexpr uintptr_t oCEntityGpnAttackSettings             = 0xF4DE88;
        constexpr uintptr_t oCEntityCpntAttackPosComputerSettings = 0xF6ADC8;
        constexpr uintptr_t EntityAttackEntityCpntSettings        = 0xF6AE08;
        constexpr uintptr_t oSDtAttackIdentifier                  = 0xEE3C68;
        constexpr uintptr_t oSAttackIdentifier                    = 0xEE6568;
    }}

    // ============================================================
    // Strings de Attack Power
    // ============================================================
    namespace AttackPowerStrings
    {{
        constexpr uintptr_t AttackPower            = {s_atk_power};
        constexpr uintptr_t AttackPowerBasic       = {s_atk_basic};
        constexpr uintptr_t AttackPowerSecondary   = {s_atk_sec};
        constexpr uintptr_t AttackPowerPrimary     = {s_atk_pri};
        constexpr uintptr_t AttackPowerTrait       = {s_atk_trait};
        constexpr uintptr_t AttackPowerDefensive   = {s_atk_def};
        constexpr uintptr_t AttackPowerDash        = {s_atk_dash};
        constexpr uintptr_t AttackPowerUltimate    = {s_atk_ult};
        constexpr uintptr_t CounterAttackDamage    = {s_counter};
        constexpr uintptr_t AttacksIgnoreResistance = {s_ignore_res};
        constexpr uintptr_t AttackPowerKey         = {s_atk_key};
    }}

    // ============================================================
    // Strings de Tipos de Ataque
    // ============================================================
    namespace AttackTypeStrings
    {{
        constexpr uintptr_t Attack                 = {s_attack};
        constexpr uintptr_t AttackSettings         = {s_atk_settings};
        constexpr uintptr_t DtAttackAim            = {s_dt_aim};
        constexpr uintptr_t DtChargeAttack         = {s_dt_charge};
        constexpr uintptr_t DtChargeAttackStep     = {s_dt_charge_step};
        constexpr uintptr_t DtAttackCombo          = {s_dt_combo};
        constexpr uintptr_t AttackComboStepSettings = {s_combo_step};
        constexpr uintptr_t AttackPosComputer      = {s_atk_pos};
        constexpr uintptr_t EntityAttack           = {s_entity_atk};
        constexpr uintptr_t Weapon                 = {s_weapon};
        constexpr uintptr_t ChargeToDamage         = {s_charge_dmg};
        constexpr uintptr_t Critical               = {s_critical};
        constexpr uintptr_t CriticalHealth         = {s_crit_health};
    }}

    // ============================================================
    // Strings de Skills
    // ============================================================
    namespace SkillStrings
    {{
        constexpr uintptr_t Skills                 = {s_skills};
        constexpr uintptr_t SkillPropose           = {s_skill_prop};
        constexpr uintptr_t SkillSelected          = {s_skill_sel};
        constexpr uintptr_t SkillBetterQuality     = {s_skill_quality};
        constexpr uintptr_t SkillChoice            = {s_skill_choice};
        constexpr uintptr_t SkillMenuOpen          = {s_skill_menu};
        constexpr uintptr_t SkillName              = {s_skill_name};
        constexpr uintptr_t SkillSelectedKey       = {s_skill_sel_key};
    }}

    // ============================================================
    // Strings de Items / Calidad / Loot
    // ============================================================
    namespace ItemStrings
    {{
        constexpr uintptr_t ItemQualityRare        = {s_rare};       // "Item_Quality_Rare"
        constexpr uintptr_t ItemQualityCommon      = {s_common};     // "Item_Quality_Common"
        constexpr uintptr_t ItemQualityLegendary   = {s_legendary};  // "Item_Quality_Legendary"
        constexpr uintptr_t ItemQualityEpic        = {s_epic};       // "Item_Quality_Epic"
        constexpr uintptr_t ItemQualityPowerUp     = {s_powerup};    // "Item_Quality_Power_Up"
        constexpr uintptr_t ItemQualityCurse       = {s_curse};      // "Item_Quality_Currse"
        constexpr uintptr_t ItemSelectable         = {s_selectable};
        constexpr uintptr_t Item                   = {s_item};
        constexpr uintptr_t LootCount              = {s_loot};
        constexpr uintptr_t Rarity                 = {s_rarity};
        constexpr uintptr_t DropRate               = {s_drop};
        constexpr uintptr_t Equiped                = {s_equip};
    }}

    // ============================================================
    // Strings de Inventario
    // ============================================================
    namespace InventoryStrings
    {{
        constexpr uintptr_t InventoryMenuOpen      = {s_inv_open};
        constexpr uintptr_t Inventory              = {s_inv};
        constexpr uintptr_t InventoryMenu          = {s_inv_menu};
        constexpr uintptr_t InventoryMenuUiCtrl    = {s_inv_ui};
        constexpr uintptr_t InventoryMenuUiSettings = {s_inv_settings};
    }}
}}
"#,
        fn_attack      = addr_str(dumper, "oIDtEntityCpntAttack::Ctor"),
        fn_attack_set  = addr_str(dumper, "oIDtEntityCpntAttackSettings::Ctor"),
        fn_aim         = addr_str(dumper, "oCDtEntityCpntAttackAim::Ctor"),
        fn_charge      = addr_str(dumper, "oCDtEntityCpntChargeAttack::Ctor"),
        fn_combo       = addr_str(dumper, "oCDtEntityCpntAttackCombo::Ctor"),
        fn_zone        = addr_str(dumper, "oCEntityCpntZoneAttack::Ctor"),
        fn_target      = addr_str(dumper, "oCEntityCpntGpnTargetAttack::Ctor"),
        s_atk_power    = addr_str(dumper, "AttackPower"),
        s_atk_basic    = addr_str(dumper, "AttackPowerBasic"),
        s_atk_sec      = addr_str(dumper, "AttackPowerSecondary"),
        s_atk_pri      = addr_str(dumper, "AttackPowerPrimary"),
        s_atk_trait    = addr_str(dumper, "AttackPowerTrait"),
        s_atk_def      = addr_str(dumper, "AttackPowerDefensive"),
        s_atk_dash     = addr_str(dumper, "AttackPowerDash"),
        s_atk_ult      = addr_str(dumper, "AttackPowerUltimate"),
        s_counter      = addr_str(dumper, "CounterAttackDamage"),
        s_ignore_res   = addr_str(dumper, "AttacksIgnoreResistance"),
        s_atk_key      = addr_str(dumper, "AttackPowerStr"),
        s_attack       = addr_str(dumper, "Attack"),
        s_atk_settings = addr_str(dumper, "AttackSettings"),
        s_dt_aim       = addr_str(dumper, "DtAttackAim"),
        s_dt_charge    = addr_str(dumper, "DtChargeAttack"),
        s_dt_charge_step = addr_str(dumper, "DtChargeAttackStep"),
        s_dt_combo     = addr_str(dumper, "DtAttackCombo"),
        s_combo_step   = addr_str(dumper, "AttackComboStepSettings"),
        s_atk_pos      = addr_str(dumper, "AttackPosComputer"),
        s_entity_atk   = addr_str(dumper, "EntityAttack"),
        s_weapon       = addr_str(dumper, "Weapon"),
        s_charge_dmg   = addr_str(dumper, "ChargeToDamage"),
        s_critical     = addr_str(dumper, "Critical"),
        s_crit_health  = addr_str(dumper, "CriticalHealth"),
        s_skills       = addr_str(dumper, "Skills"),
        s_skill_prop   = addr_str(dumper, "SkillPropose"),
        s_skill_sel    = addr_str(dumper, "SkillSelected"),
        s_skill_quality = addr_str(dumper, "SkillBetterQuality"),
        s_skill_choice = addr_str(dumper, "SkillChoice"),
        s_skill_menu   = addr_str(dumper, "SkillMenuOpen"),
        s_skill_name   = addr_str(dumper, "SkillName"),
        s_skill_sel_key = addr_str(dumper, "SkillSelectedKey"),
        s_rare         = addr_str(dumper, "ItemQualityRare"),
        s_common       = addr_str(dumper, "ItemQualityCommon"),
        s_legendary    = addr_str(dumper, "ItemQualityLegendary"),
        s_epic         = addr_str(dumper, "ItemQualityEpic"),
        s_powerup      = addr_str(dumper, "ItemQualityPowerUp"),
        s_curse        = addr_str(dumper, "ItemQualityCurse"),
        s_selectable   = addr_str(dumper, "ItemSelectable"),
        s_item         = addr_str(dumper, "Item"),
        s_loot         = addr_str(dumper, "LootCount"),
        s_rarity       = addr_str(dumper, "Rarity"),
        s_drop         = addr_str(dumper, "DropRate"),
        s_equip        = addr_str(dumper, "Equiped"),
        s_inv_open     = addr_str(dumper, "InventoryMenuOpen"),
        s_inv          = addr_str(dumper, "Inventory"),
        s_inv_menu     = addr_str(dumper, "InventoryMenu"),
        s_inv_ui       = addr_str(dumper, "InventoryMenuUiCtrl"),
        s_inv_settings = addr_str(dumper, "InventoryMenuUiSettings"),
    ));
    s
}
