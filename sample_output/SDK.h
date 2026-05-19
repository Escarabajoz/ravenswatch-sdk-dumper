#pragma once
// ============================================================
// SDK DEL JUEGO - Archivo Principal
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

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
