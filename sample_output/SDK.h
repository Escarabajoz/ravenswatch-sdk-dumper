#pragma once
// ============================================================
// Ravenswatch SDK - Auto-generated
// Engine: Custom oC* (Passtech Games)
// Dump date: 2026-05-16
// ============================================================

#include "Offsets.h"
#include "Player.h"
#include "Camera.h"
#include "Damage.h"
#include "Health.h"
#include "Movement.h"
#include "Ability.h"
#include "Entity.h"
#include "Enemy.h"
#include "Network.h"
#include "Multiplayer.h"
#include "Weapon.h"
#include "Projectile.h"

namespace SDK {
    // All offsets are RVA. To resolve at runtime:
    //   uintptr_t base = (uintptr_t)GetModuleHandle(NULL);
    //   uintptr_t addr = base + RVA;

    inline uintptr_t GetBase() { return (uintptr_t)GetModuleHandleA(NULL); }
    inline uintptr_t Resolve(uintptr_t rva) { return GetBase() + rva; }

    struct Vec3 { float x, y, z; };
    struct Vec4 { float x, y, z, w; };
    struct Mat4x4 { float m[4][4]; };

    // === KEY POINTER CHAINS ===
    //
    // PLAYER POSITION:
    // BarkPlayerEntityCpnt + 0x140 -> oCEntity3dLocator
    //   +0x148 -> X, +0x150 -> Y, +0x158 -> Z
    //
    // ENEMY POSITION:
    // oCEntityCpntBasicMove + 0xE0 -> oCEntity3dLocator
    //   +0xE8 -> X, +0xF0 -> Y, +0xF8 -> Z
    //
    // CAMERA:
    // oCEntityCpntBasicCamera + 0x98 -> Camera position
    // oCEntityCpntBasicCamera + 0xB8 -> Look-at target
    // oCEntityCpntBasicCamera + 0x12C -> FOV (60.0f)
    // g_ViewProjMat -> renderer[82], matrix at +48
    //
    // ENTITY LIST:
    // oCEntitySceneContext + 0x2B8 / +0x2D0 / +0x2E8 (3 lists)
    //
    // WORLD-TO-SCREEN:
    // 1. clipPos = ViewProjMat * Vec4(x, y, z, 1)
    // 2. ndcX = clipPos.x / clipPos.w
    // 3. ndcY = clipPos.y / clipPos.w
    // 4. screenX = (ndcX + 1) * 0.5 * width
    // 5. screenY = (1 - ndcY) * 0.5 * height
}
