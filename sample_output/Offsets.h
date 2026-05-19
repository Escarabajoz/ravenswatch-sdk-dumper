#pragma once
// ============================================================
// ALL OFFSETS - VTables + Functions + Strings
// ============================================================

namespace SDK::VTables {
    // Player
    constexpr uintptr_t BarkPlayerEntityCpnt_VTable             = 0xF4F778;
    constexpr uintptr_t oCHeroControllerNetworkData_VTable      = 0xEEB7D0;
    // Camera
    constexpr uintptr_t oCEntityCpntBasicCamera_VTable          = 0xF4F778;
    constexpr uintptr_t oCEntityCpntTopDownCamera_VTable        = 0xF4F778;
    // Damage
    constexpr uintptr_t oCEntityCpntBasicDamage_VTable          = 0xF4F778;
    constexpr uintptr_t oCDtEntityCpntDamageSettings_VTable     = 0xEFDA50;
    // Entity
    constexpr uintptr_t oCEntitySceneContext_VTable             = 0xEAFCB8;
    constexpr uintptr_t oCGameObject_VTable                     = 0xEDD7C8;
    constexpr uintptr_t oCGameScene_VTable                      = 0xF27D30;
    // Movement
    constexpr uintptr_t oCEntityCpntBasicMove_VTable            = 0xF4F778;
    // Network
    constexpr uintptr_t oCEntityCpntNetwork_VTable              = 0xF4F778;
    constexpr uintptr_t oCEntityCpntHitPointNetworkData_VTable  = 0xEDF018;
    // Ability
    constexpr uintptr_t oCDtEntityCpntAbilityController_VTable  = 0xF4F778;
}

namespace SDK::Functions {
    // Player
    constexpr uintptr_t BarkPlayerEntityCpnt_Ctor              = 0x1CCCF0;
    constexpr uintptr_t PlayerDebugTelemetry                   = 0x1D6B10;
    // Camera
    constexpr uintptr_t oCEntityCpntBasicCamera_Ctor           = 0x1DCE60;
    constexpr uintptr_t oCEntityCpntTopDownCamera_Ctor         = 0xC64E0;
    constexpr uintptr_t CameraSceneContext_Ctor                = 0x1CC730;
    constexpr uintptr_t SetupCameraShaderUniforms              = 0x57B940;
    // Damage
    constexpr uintptr_t oCEntityCpntBasicDamage_Ctor           = 0x1CB300;
    constexpr uintptr_t oCDtEntityCpntDamageSettings_Ctor      = 0x2E5AC0;
    // Entity
    constexpr uintptr_t oCGameObject_Ctor                      = 0x243C80;
    constexpr uintptr_t oCGameScene_Ctor                       = 0x463860;
    constexpr uintptr_t oCEntitySceneContext_Ctor              = 0x6E5A40;
    // Movement
    constexpr uintptr_t oCEntityCpntBasicMove_Ctor             = 0x1DD4B0;
    constexpr uintptr_t oCEntityCpntGpnTargetComputer_Ctor     = 0x3C1F90;
    // Ability
    constexpr uintptr_t oCDtEntityCpntAbilityController_Ctor   = 0x2B0990;
    // Multiplayer
    constexpr uintptr_t ConnectToStormancerScene               = 0x176EF0;
    constexpr uintptr_t oCStormancerSceneContext_MainHandler   = 0x8DD890;
    // Network
    constexpr uintptr_t oCEntityCpntNetwork_Ctor               = 0x704210;
}

namespace SDK::Strings {
    constexpr uintptr_t PlayerLocation        = 0xEF0470;
    constexpr uintptr_t LocalPlayerPosition   = 0xEF0560;
    constexpr uintptr_t ViewProjMat           = 0xEF787D;
    constexpr uintptr_t g_ViewMat             = 0xEF7890;
    constexpr uintptr_t MaxHealth             = 0xEEF6E0;
    constexpr uintptr_t HealthPercent         = 0xEFBBD8;
    constexpr uintptr_t Cooldown              = 0xEF02F5;
    constexpr uintptr_t ServerUrlLive         = 0xEED7B0;
    constexpr uintptr_t ServerUrlDev          = 0xEED7D8;
    constexpr uintptr_t SteamApi64            = 0xE9D640;
}
