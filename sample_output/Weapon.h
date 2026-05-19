#pragma once
// ============================================================
// SISTEMA DE ARMAS / ATAQUES / ITEMS / INVENTARIO
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
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
    {
        constexpr uintptr_t Constructor          = 0x1E2E10;     // oIDtEntityCpntAttack
        constexpr uintptr_t SettingsConstructor   = 0x3AE880; // oIDtEntityCpntAttackSettings
    }

    namespace AttackAim
    {
        constexpr uintptr_t Constructor = 0x3AF4A0;     // oCDtEntityCpntAttackAim
    }

    namespace ChargeAttack
    {
        constexpr uintptr_t Constructor = 0x1DD400;  // oCDtEntityCpntChargeAttack
    }

    namespace AttackCombo
    {
        constexpr uintptr_t Constructor = 0x3C3110;   // oCDtEntityCpntAttackCombo
    }

    namespace ZoneAttack
    {
        constexpr uintptr_t Constructor = 0x1E3050;    // oCEntityCpntZoneAttack (AoE)
    }

    namespace TargetAttack
    {
        constexpr uintptr_t Constructor = 0xC9410;  // oCEntityCpntGpnTargetAttack
    }

    // ============================================================
    // VTables de Ataques (estáticas)
    // ============================================================
    namespace AttackVTables
    {
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
    }

    // ============================================================
    // Strings de Attack Power
    // ============================================================
    namespace AttackPowerStrings
    {
        constexpr uintptr_t AttackPower            = 0xEEF900;
        constexpr uintptr_t AttackPowerBasic       = 0xEEF900;
        constexpr uintptr_t AttackPowerSecondary   = 0xEEF928;
        constexpr uintptr_t AttackPowerPrimary     = 0xEEF940;
        constexpr uintptr_t AttackPowerTrait       = 0xEEF958;
        constexpr uintptr_t AttackPowerDefensive   = 0xEEF970;
        constexpr uintptr_t AttackPowerDash        = 0xEEF988;
        constexpr uintptr_t AttackPowerUltimate    = 0xEEF9A0;
        constexpr uintptr_t CounterAttackDamage    = 0xEEFF78;
        constexpr uintptr_t AttacksIgnoreResistance = 0xEF0150;
        constexpr uintptr_t AttackPowerKey         = 0xEED5D8;
    }

    // ============================================================
    // Strings de Tipos de Ataque
    // ============================================================
    namespace AttackTypeStrings
    {
        constexpr uintptr_t Attack                 = 0xEEF900;
        constexpr uintptr_t AttackSettings         = 0xF092C8;
        constexpr uintptr_t DtAttackAim            = 0xF09320;
        constexpr uintptr_t DtChargeAttack         = 0xF0E040;
        constexpr uintptr_t DtChargeAttackStep     = 0xF0E058;
        constexpr uintptr_t DtAttackCombo          = 0xF0E130;
        constexpr uintptr_t AttackComboStepSettings = 0xF0E140;
        constexpr uintptr_t AttackPosComputer      = 0xF6A5B8;
        constexpr uintptr_t EntityAttack           = 0xF6A830;
        constexpr uintptr_t Weapon                 = 0xF435FC;
        constexpr uintptr_t ChargeToDamage         = 0xEEFC9E;
        constexpr uintptr_t Critical               = 0xE9F11F;
        constexpr uintptr_t CriticalHealth         = 0xEEFF23;
    }

    // ============================================================
    // Strings de Skills
    // ============================================================
    namespace SkillStrings
    {
        constexpr uintptr_t Skills                 = 0xEEF40A;
        constexpr uintptr_t SkillPropose           = 0xEEF3C0;
        constexpr uintptr_t SkillSelected          = 0xEEF3F0;
        constexpr uintptr_t SkillBetterQuality     = 0xEF0036;
        constexpr uintptr_t SkillChoice            = 0xEF01FE;
        constexpr uintptr_t SkillMenuOpen          = 0xEF0593;
        constexpr uintptr_t SkillName              = 0xEF25F8;
        constexpr uintptr_t SkillSelectedKey       = 0xEF2648;
    }

    // ============================================================
    // Strings de Items / Calidad / Loot
    // ============================================================
    namespace ItemStrings
    {
        constexpr uintptr_t ItemQualityRare        = 0xEEE840;       // "Item_Quality_Rare"
        constexpr uintptr_t ItemQualityCommon      = 0xEEE858;     // "Item_Quality_Common"
        constexpr uintptr_t ItemQualityLegendary   = 0xEEE870;  // "Item_Quality_Legendary"
        constexpr uintptr_t ItemQualityEpic        = 0xEEE888;       // "Item_Quality_Epic"
        constexpr uintptr_t ItemQualityPowerUp     = 0xEEE8A0;    // "Item_Quality_Power_Up"
        constexpr uintptr_t ItemQualityCurse       = 0xEEE8B8;      // "Item_Quality_Currse"
        constexpr uintptr_t ItemSelectable         = 0xEFBE98;
        constexpr uintptr_t Item                   = 0xEE7700;
        constexpr uintptr_t LootCount              = 0xEF074E;
        constexpr uintptr_t Rarity                 = 0xEF2570;
        constexpr uintptr_t DropRate               = 0xEF0026;
        constexpr uintptr_t Equiped                = 0xEFB918;
    }

    // ============================================================
    // Strings de Inventario
    // ============================================================
    namespace InventoryStrings
    {
        constexpr uintptr_t InventoryMenuOpen      = 0xEF05E3;
        constexpr uintptr_t Inventory              = 0xEF05E3;
        constexpr uintptr_t InventoryMenu          = 0xEF30AE;
        constexpr uintptr_t InventoryMenuUiCtrl    = 0xF06EC0;
        constexpr uintptr_t InventoryMenuUiSettings = 0xF06EC0;
    }
}
