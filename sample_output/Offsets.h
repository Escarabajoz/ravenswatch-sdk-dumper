#pragma once
// ============================================================
// OFFSETS COMPLETOS
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
    namespace VTables
    {
        // --- Ability ---
        constexpr uintptr_t oCDtEntityCpntAbilityController_VTable        = 0xF4F778;
        // --- Camera ---
        constexpr uintptr_t CameraSceneContext_VTable                     = 0xEAFCB8;
        constexpr uintptr_t SetupCameraShaderUniforms_VTable              = 0xEF7890;
        constexpr uintptr_t oCEntityCpntBasicCamera_VTable                = 0xF4F778;
        constexpr uintptr_t oCEntityCpntTopDownCamera_VTable              = 0xF4F778;
        // --- Damage ---
        constexpr uintptr_t oCDtEntityCpntDamageSettings_VTable           = 0xEFDA50;
        constexpr uintptr_t oCEntityCpntBasicDamage_VTable                = 0xF4F778;
        // --- Enemy ---
        constexpr uintptr_t oCDtEnemyDefinition_VTable                    = 0xEDD7C8;
        // --- Entity ---
        constexpr uintptr_t oCEntitySceneContext_VTable                   = 0xEAFCB8;
        constexpr uintptr_t oCGameObject_VTable                           = 0xEDD7C8;
        constexpr uintptr_t oCGameScene_VTable                            = 0xF27D30;
        constexpr uintptr_t oIGameScene_VTable                            = 0xF3EC20;
        constexpr uintptr_t oCEntityGpnAroundPositionTraverser_VTable     = 0xF61DD8;
        // --- Health ---
        constexpr uintptr_t oCNamedEventGainHealth_VTable                 = 0xEDF1E0;
        // --- Movement ---
        constexpr uintptr_t oCEntityCpntBasicMove_VTable                  = 0xF4F778;
        constexpr uintptr_t oCEntityCpntGpnTargetComputer_VTable          = 0xF4F778;
        // --- Network ---
        constexpr uintptr_t oCEntityCpntNetworkSettings_VTable            = 0xEDEE60;
        constexpr uintptr_t oCEntityCpntHitPointNetworkData_VTable        = 0xEDF018;
        constexpr uintptr_t oCGlobalEntityValueNetworkData_VTable         = 0xEE6638;
        constexpr uintptr_t oCEntityCpntValueNetworkData_VTable           = 0xF007E8;
        constexpr uintptr_t oCEntityCpntNetwork_VTable                    = 0xF4F778;
        // --- Player ---
        constexpr uintptr_t PlayerDebugTelemetry_VTable                   = 0xEB46D0;
        constexpr uintptr_t oCHeroControllerNetworkData_VTable            = 0xEEB7D0;
        constexpr uintptr_t BarkPlayerEntityCpnt_VTable                   = 0xF4F778;
        // --- Projectile ---
        constexpr uintptr_t oCEntityCpntGpnProjectileAttack_VTable        = 0xF4F778;
        // --- Weapon ---
        constexpr uintptr_t oCEntityCpntGpnTargetAttack_VTable            = 0xEDA9C8;
        constexpr uintptr_t oCDtEntityCpntChargeAttack_VTable             = 0xEE3CC0;
        constexpr uintptr_t oIDtEntityCpntAttackSettings_VTable           = 0xF0D480;
        constexpr uintptr_t oCDtEntityCpntAttackCombo_VTable              = 0xF0F5D8;
        constexpr uintptr_t oIDtEntityCpntAttack_VTable                   = 0xF4F778;
        constexpr uintptr_t oCEntityCpntZoneAttack_VTable                 = 0xF4F778;
    }

    namespace Functions
    {
        // --- Ability ---
        constexpr uintptr_t oCDtEntityCpntAbilityController::Ctor         = 0x2B0990; // Constructor del controlador de habilidades
        // --- Camera ---
        constexpr uintptr_t oCEntityCpntTopDownCamera::Ctor               = 0xC64E0; // Constructor de cámara top-down
        constexpr uintptr_t CameraSceneContext::Ctor                      = 0x1CC730; // Constructor del contexto de escena de cámara
        constexpr uintptr_t oCEntityCpntBasicCamera::Ctor                 = 0x1DCE60; // Constructor de cámara básica
        constexpr uintptr_t SetupCameraShaderUniforms                     = 0x57B940; // Configura ViewMat, ViewProjMat y uniforms de cámara
        // --- Damage ---
        constexpr uintptr_t oCEntityCpntBasicDamage::Ctor                 = 0x1CB300; // Constructor del componente de daño
        constexpr uintptr_t oCDtEntityCpntDamageSettings::Ctor            = 0x2E5AC0; // Constructor de settings de daño
        // --- Enemy ---
        constexpr uintptr_t oCDtEnemyDefinition::Ctor                     = 0x1DB800; // Constructor de definición de enemigo
        // --- Entity ---
        constexpr uintptr_t oCGameObject::Ctor                            = 0x243C80; // Constructor de game object
        constexpr uintptr_t oCGameScene::Ctor                             = 0x463860; // Constructor de la escena principal
        constexpr uintptr_t oIGameScene::Ctor                             = 0x6537C0; // Constructor de la escena base (datos de cámara)
        constexpr uintptr_t oCEntitySceneContext::Ctor                    = 0x6E5A40; // Constructor del contexto de escena de entidades
        constexpr uintptr_t oCEntityGpnAroundPositionTraverser::Factory   = 0x7ADF80; // Factory del traverser de entidades por posición
        // --- Health ---
        constexpr uintptr_t oCNamedEventGainHealth::Ctor                  = 0x27F420; // Constructor del evento de curación
        // --- Movement ---
        constexpr uintptr_t oCEntityCpntBasicMove::Ctor                   = 0x1DD4B0; // Constructor del componente de movimiento
        constexpr uintptr_t oCEntityCpntGpnTargetComputer::Ctor           = 0x3C1F90; // Constructor del sistema de targeting
        // --- Multiplayer ---
        constexpr uintptr_t ConnectToStormancerScene                      = 0x176EF0; // Conexión a escena de Stormancer (sesión/servidor)
        constexpr uintptr_t oCStormancerSceneContext::MainHandler         = 0x8DD890; // Handler principal de escena Stormancer (sesiones, P2P, party)
        // --- Network ---
        constexpr uintptr_t oCEntityCpntNetworkSettings::Ctor             = 0xC3BC0; // Constructor de settings de red de entidad
        constexpr uintptr_t oCEntityCpntHitPointNetworkData::Ctor         = 0xC3C30; // Constructor de datos de red de hit points
        constexpr uintptr_t oCGlobalEntityValueNetworkData::Factory       = 0x211F10; // Factory de datos de red de valores globales
        constexpr uintptr_t oCEntityCpntValueNetworkData::Ctor            = 0x3005B0; // Constructor de datos de red de valores de entidad
        constexpr uintptr_t oCEntityCpntNetwork::Ctor                     = 0x704210; // Constructor del componente de red de entidad
        // --- Player ---
        constexpr uintptr_t BarkPlayerEntityCpnt::Ctor                    = 0x1CCCF0; // Constructor del componente principal del jugador
        constexpr uintptr_t PlayerDebugTelemetry                          = 0x1D6B10; // Función de debug/telemetría del jugador (refs Player location, etc.)
        constexpr uintptr_t oCHeroControllerNetworkData::Ctor             = 0x2457B0; // Constructor de datos de red del héroe
        // --- Projectile ---
        constexpr uintptr_t oCEntityCpntGpnProjectileAttack::Ctor         = 0x260A70; // Constructor de ataque con proyectil
        // --- Weapon ---
        constexpr uintptr_t oCEntityCpntGpnTargetAttack::Ctor             = 0xC9410; // Constructor de ataque dirigido a target
        constexpr uintptr_t oCDtEntityCpntChargeAttack::Ctor              = 0x1DD400; // Constructor de ataque cargado (ChargeAttack)
        constexpr uintptr_t oIDtEntityCpntAttack::Ctor                    = 0x1E2E10; // Constructor de ataque base (interfaz)
        constexpr uintptr_t oCEntityCpntZoneAttack::Ctor                  = 0x1E3050; // Constructor de ataque de zona (AoE)
        constexpr uintptr_t oIDtEntityCpntAttackSettings::Ctor            = 0x3AE880; // Constructor de settings de ataque base
        constexpr uintptr_t oCDtEntityCpntAttackAim::Ctor                 = 0x3AF4A0; // Constructor de apuntado de ataque (AttackAim)
        constexpr uintptr_t oCDtEntityCpntAttackCombo::Ctor               = 0x3C3110; // Constructor de combo de ataque
    }

    namespace Strings
    {
        // --- Ability ---
        constexpr uintptr_t Cooldown                                      = 0xEF02F5; // "Cooldown de habilidad"
        // --- Camera ---
        constexpr uintptr_t ViewProjMat                                   = 0xEF787D; // "Matriz View-Projection"
        constexpr uintptr_t g_ViewMat                                     = 0xEF7890; // "View Matrix global"
        constexpr uintptr_t viewProj                                      = 0xF304F8; // "View-Projection (uniform)"
        constexpr uintptr_t g_cameraNearFar                               = 0xF305C8; // "Near/Far planes de cámara"
        // --- Damage ---
        constexpr uintptr_t ShowDamageAndHealth                           = 0xEEDAA8; // "Mostrar daño y números de curación"
        constexpr uintptr_t DamageReceived                                = 0xEEF200; // "Daño recibido"
        constexpr uintptr_t DamageDealt                                   = 0xEEF210; // "Daño causado"
        constexpr uintptr_t FatalDamageReceived                           = 0xEEF230; // "Daño fatal recibido"
        constexpr uintptr_t CritDamage                                    = 0xEEFDB8; // "Daño crítico"
        constexpr uintptr_t IncreaseDamageToBoss                          = 0xEEFED0; // "Incremento de daño a boss"
        constexpr uintptr_t BleedOnDamageReceive                          = 0xEF00E0; // "Sangrado al recibir daño"
        constexpr uintptr_t DamageInterrupt                               = 0xEF0300; // "Interrupción de daño"
        // --- Enemy ---
        constexpr uintptr_t AllPlayers                                    = 0xEFBB50; // "Todos los jugadores"
        // --- Health ---
        constexpr uintptr_t GainHealth                                    = 0xEE12B8; // "Ganar salud"
        constexpr uintptr_t MaxHealth                                     = 0xEEF6E0; // "Salud máxima"
        constexpr uintptr_t IncreaseHealthDropRate                        = 0xEF0010; // "Incrementar tasa de drop de globos de salud"
        constexpr uintptr_t HealthPercentOfMax                            = 0xEFBA90; // "Salud porcentaje del máximo"
        constexpr uintptr_t HealthAbsolute                                = 0xEFBAB8; // "Salud absoluta"
        constexpr uintptr_t HealthPercent                                 = 0xEFBBD8; // "Porcentaje de salud"
        constexpr uintptr_t HealFullHealth                                = 0xF09098; // "Curar salud completa"
        // --- Movement ---
        constexpr uintptr_t MoveSpeedRatio                                = 0xEEF6B0; // "Ratio de velocidad de movimiento"
        constexpr uintptr_t SpeedRatio                                    = 0xEEF6B5; // "Ratio de velocidad"
        constexpr uintptr_t AngleSpeed                                    = 0xF1FB50; // "Velocidad angular"
        constexpr uintptr_t ConstantSpeed                                 = 0xF36B29; // "Velocidad constante"
        constexpr uintptr_t SpeedNorm                                     = 0xF37709; // "Velocidad normalizada"
        // --- Multiplayer ---
        constexpr uintptr_t GameSession                                   = 0xEC6CB8; // "Sesión de juego"
        constexpr uintptr_t GameSessionInfos                              = 0xEDB2F8; // "Info de sesión de juego"
        constexpr uintptr_t PartyId                                       = 0xEE1B3B; // "ID del party"
        constexpr uintptr_t LobbyStr                                      = 0xEE1D51; // "Lobby"
        constexpr uintptr_t PartyDataBearerTokens                         = 0xEE1E1C; // "Tokens de autenticación del party"
        constexpr uintptr_t CreatePartyDataToken                          = 0xEE1E55; // "Crear token de datos del party"
        constexpr uintptr_t CreateLobby                                   = 0xEE1EE6; // "Crear lobby"
        constexpr uintptr_t JoinLobby                                     = 0xEE1EFE; // "Unirse a lobby"
        constexpr uintptr_t Invite                                        = 0xEE1F46; // "Invitar"
        constexpr uintptr_t PartyDataToken                                = 0xEE2638; // "Token de datos del party"
        constexpr uintptr_t PartySceneIsNull                              = 0xEE26D0; // "Escena de party es null"
        constexpr uintptr_t ReplicationManagerSceneCtx                    = 0xEE98F1; // "Contexto de escena del replication manager"
        constexpr uintptr_t P2PSessionSceneContext                        = 0xEEAC70; // "Contexto de escena P2P"
        constexpr uintptr_t MultiplayerPng                                = 0xEED5AF; // "Icono multijugador"
        constexpr uintptr_t MultiplayerScore                              = 0xEEE703; // "Puntuación multijugador"
        constexpr uintptr_t SessionEnd                                    = 0xEEF168; // "Fin de sesión"
        constexpr uintptr_t GameSessionSize                               = 0xEF0B40; // "Tamaño de sesión"
        constexpr uintptr_t IsSessionHost                                 = 0xEF0B70; // "Es host de sesión"
        constexpr uintptr_t MatchmakingStart                              = 0xEF2340; // "Inicio de matchmaking"
        constexpr uintptr_t Matchmaking                                   = 0xEF2340; // "Matchmaking"
        constexpr uintptr_t MatchmakingEnd                                = 0xEF2398; // "Fin de matchmaking"
        constexpr uintptr_t SessionFromHost                               = 0xEF3D96; // "Sesión desde host"
        constexpr uintptr_t P2PSessionCreationFailed                      = 0xEF3EF8; // "Creación de sesión P2P fallida"
        constexpr uintptr_t MultiplayerConnection                         = 0xEF3F81; // "Conexión multijugador"
        constexpr uintptr_t P2PSessionJoinFailed                          = 0xEF3F98; // "Unión a sesión P2P fallida"
        constexpr uintptr_t MultiplayerModalUiCtrl                        = 0xEF5610; // "Controlador UI modal multijugador"
        constexpr uintptr_t MatchmakingOptions                            = 0xEF5770; // "Opciones de matchmaking"
        constexpr uintptr_t MultiplayerModalUiSettings                    = 0xEF57C0; // "Settings UI modal multijugador"
        constexpr uintptr_t SendingSessionJoinRequest                     = 0xEF80F0; // "Enviando solicitud de unión a sesión"
        constexpr uintptr_t ReplicationInterval                           = 0xF0EB00; // "Intervalo de replicación"
        constexpr uintptr_t ReplicationPeriod                             = 0xF51353; // "Período de replicación"
        constexpr uintptr_t ReplicationManagerScene                       = 0xF73AA0; // "Contexto de escena del replication manager"
        constexpr uintptr_t ReplicationManagerCtxUpdate                   = 0xF7D761; // "Update del contexto de replication manager"
        constexpr uintptr_t ReplicationManagerSceneStr                    = 0xF7D791; // "Contexto de escena del replication manager (lowercase)"
        // --- Network ---
        constexpr uintptr_t SteamApi64                                    = 0xE9D640; // "Steam API DLL"
        constexpr uintptr_t NetworkDamage                                 = 0xEE1260; // "Daño de red"
        constexpr uintptr_t LoginFailed                                   = 0xEE15C0; // "Login fallido"
        constexpr uintptr_t LoginErrorRecoverable                         = 0xEE1720; // "Login fallido con error recuperable"
        constexpr uintptr_t LoginErrorUnrecoverable                       = 0xEE1778; // "Login fallido con error irrecuperable"
        constexpr uintptr_t AutoReconnection                              = 0xEE1AE0; // "Reconexión automática deshabilitada"
        constexpr uintptr_t NoConnection                                  = 0xEE1D30; // "Sin conexión"
        constexpr uintptr_t NetworkSettings                               = 0xEE4A1C; // "Configuración de red"
        constexpr uintptr_t ServerUrlLive                                 = 0xEED7B0; // "URL servidor live"
        constexpr uintptr_t ServerUrlDev                                  = 0xEED7D8; // "URL servidor dev"
        constexpr uintptr_t ServerUrlLive2                                = 0xEED800; // "URL servidor live 2"
        constexpr uintptr_t ServerUrlLocalhost                            = 0xEED828; // "URL servidor localhost"
        constexpr uintptr_t ServerUrlLive3                                = 0xEED840; // "URL servidor live 3"
        constexpr uintptr_t SessionConnectionError                        = 0xEEDC30; // "Error de conexión de sesión"
        constexpr uintptr_t SessionConnectionFail                         = 0xEEDCC8; // "Fallo de conexión de sesión"
        constexpr uintptr_t PartyConnectionError                          = 0xEEDD60; // "Error de conexión de party"
        constexpr uintptr_t IsConnectedToSession                          = 0xEF0B80; // "Conectado a sesión"
        constexpr uintptr_t NetworkStats                                  = 0xEF28C0; // "Estadísticas de red"
        constexpr uintptr_t SessionId                                     = 0xEF29B8; // "ID de sesión"
        constexpr uintptr_t NetworkSessionSize                            = 0xEF7330; // "Tamaño de sesión de red"
        constexpr uintptr_t NetworkStatsUI                                = 0xEF8168; // "UI de estadísticas de red"
        constexpr uintptr_t OnlineId                                      = 0xF829C8; // "ID online"
        constexpr uintptr_t EpicOnlineService                             = 0xF8DD60; // "Epic Online Services"
        constexpr uintptr_t RakPeerInterface                              = 0xF92F59; // "Interfaz RakNet Peer"
        // --- Player ---
        constexpr uintptr_t PlayerId                                      = 0xEED8E8; // "ID del jugador"
        constexpr uintptr_t PlayerName                                    = 0xEED930; // "Nombre del jugador"
        constexpr uintptr_t PlayerLocation                                = 0xEF0470; // "Ubicación del jugador"
        constexpr uintptr_t LocalPlayerPosition                           = 0xEF0560; // "Posición local del jugador"
        constexpr uintptr_t PlayerWatchingPosition                        = 0xEF05A8; // "Posición de observación del jugador"
        // --- Projectile ---
        constexpr uintptr_t VelocityScale                                 = 0xF1FBF3; // "Escala de velocidad"
        constexpr uintptr_t Velocity                                      = 0xF4C47B; // "Velocidad del proyectil"
        // --- Statistics ---
        constexpr uintptr_t StatsLogsEnable                               = 0xEED97B; // "Habilitar logs de estadísticas"
        constexpr uintptr_t StatsLogInterval                              = 0xEED9A3; // "Intervalo de logs de estadísticas"
        constexpr uintptr_t Analytics                                     = 0xEEED23; // "Analytics del juego"
        constexpr uintptr_t StatsUiControllerSettings                     = 0xEF9DCB; // "Settings del controlador UI de estadísticas"
        constexpr uintptr_t StatsUiController                             = 0xEF9EB3; // "Controlador UI de estadísticas"
        constexpr uintptr_t Statistics                                    = 0xF0E478; // "Estadísticas"
        constexpr uintptr_t StatisticsCollection                          = 0xF461D2; // "Colección de estadísticas"
        constexpr uintptr_t StatisticsSample                              = 0xF461EA; // "Muestra de estadísticas"
        constexpr uintptr_t StatisticsCollector                           = 0xF464E8; // "Colector de estadísticas"
        // --- Weapon ---
        constexpr uintptr_t Critical                                      = 0xE9F11F; // "Crítico"
        constexpr uintptr_t Item                                          = 0xEE7700; // "Item"
        constexpr uintptr_t AttackPowerStr                                = 0xEED5D8; // "Poder de ataque (key)"
        constexpr uintptr_t ItemQualityRare                               = 0xEEE840; // "Calidad: Rara"
        constexpr uintptr_t ItemQualityCommon                             = 0xEEE858; // "Calidad: Común"
        constexpr uintptr_t ItemQualityLegendary                          = 0xEEE870; // "Calidad: Legendaria"
        constexpr uintptr_t ItemQualityEpic                               = 0xEEE888; // "Calidad: Épica"
        constexpr uintptr_t ItemQualityPowerUp                            = 0xEEE8A0; // "Calidad: Power Up"
        constexpr uintptr_t ItemQualityCurse                              = 0xEEE8B8; // "Calidad: Maldición"
        constexpr uintptr_t SkillPropose                                  = 0xEEF3C0; // "Propuesta de habilidad"
        constexpr uintptr_t SkillSelected                                 = 0xEEF3F0; // "Habilidad seleccionada"
        constexpr uintptr_t Skills                                        = 0xEEF40A; // "Habilidades"
        constexpr uintptr_t AttackPower                                   = 0xEEF900; // "Poder de ataque"
        constexpr uintptr_t AttackPowerBasic                              = 0xEEF900; // "Poder de ataque básico"
        constexpr uintptr_t Attack                                        = 0xEEF900; // "Ataque"
        constexpr uintptr_t AttackPowerSecondary                          = 0xEEF928; // "Poder de ataque secundario"
        constexpr uintptr_t AttackPowerPrimary                            = 0xEEF940; // "Poder de ataque primario"
        constexpr uintptr_t AttackPowerTrait                              = 0xEEF958; // "Poder de ataque rasgo"
        constexpr uintptr_t AttackPowerDefensive                          = 0xEEF970; // "Poder de ataque defensivo"
        constexpr uintptr_t AttackPowerDash                               = 0xEEF988; // "Poder de ataque dash"
        constexpr uintptr_t AttackPowerUltimate                           = 0xEEF9A0; // "Poder de ataque ultimate"
        constexpr uintptr_t ChargeToDamage                                = 0xEEFC9E; // "Carga a daño"
        constexpr uintptr_t CriticalHealth                                = 0xEEFF23; // "Salud crítica"
        constexpr uintptr_t CounterAttackDamage                           = 0xEEFF78; // "Daño de contraataque"
        constexpr uintptr_t DropRate                                      = 0xEF0026; // "Tasa de drop"
        constexpr uintptr_t SkillBetterQuality                            = 0xEF0036; // "Probabilidad de mejor calidad de habilidad"
        constexpr uintptr_t AttacksIgnoreResistance                       = 0xEF0150; // "Ataques ignoran resistencia"
        constexpr uintptr_t SkillChoice                                   = 0xEF01FE; // "Elección de habilidad"
        constexpr uintptr_t SkillMenuOpen                                 = 0xEF0593; // "Menú de habilidad abierto"
        constexpr uintptr_t InventoryMenuOpen                             = 0xEF05E3; // "Menú de inventario abierto"
        constexpr uintptr_t Inventory                                     = 0xEF05E3; // "Inventario"
        constexpr uintptr_t LootCount                                     = 0xEF074E; // "Cantidad de loot"
        constexpr uintptr_t Rarity                                        = 0xEF2570; // "Rareza"
        constexpr uintptr_t SkillName                                     = 0xEF25F8; // "Nombre de habilidad"
        constexpr uintptr_t SkillSelectedKey                              = 0xEF2648; // "Habilidad seleccionada (key)"
        constexpr uintptr_t InventoryMenu                                 = 0xEF30AE; // "Menú de inventario"
        constexpr uintptr_t Equiped                                       = 0xEFB918; // "Equipado"
        constexpr uintptr_t ItemSelectable                                = 0xEFBE98; // "Item seleccionable"
        constexpr uintptr_t InventoryMenuUiCtrl                           = 0xF06EC0; // "Controlador UI del inventario"
        constexpr uintptr_t InventoryMenuUiSettings                       = 0xF06EC0; // "Settings UI del inventario"
        constexpr uintptr_t AttackSettings                                = 0xF092C8; // "Settings de ataque"
        constexpr uintptr_t DtAttackAim                                   = 0xF09320; // "Apuntado de ataque"
        constexpr uintptr_t DtChargeAttack                                = 0xF0E040; // "Ataque cargado"
        constexpr uintptr_t DtChargeAttackStep                            = 0xF0E058; // "Paso de ataque cargado"
        constexpr uintptr_t DtAttackCombo                                 = 0xF0E130; // "Combo de ataque"
        constexpr uintptr_t AttackComboStepSettings                       = 0xF0E140; // "Settings de paso de combo"
        constexpr uintptr_t Weapon                                        = 0xF435FC; // "Arma"
        constexpr uintptr_t AttackPosComputer                             = 0xF6A5B8; // "Computador de posición de ataque"
        constexpr uintptr_t EntityAttack                                  = 0xF6A830; // "Ataque de entidad"
    }
}
