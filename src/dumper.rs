// ============================================================
// Módulo de Dumping - Escanea y extrae offsets del juego
// ============================================================

use crate::process::GameProcess;
use crate::scanner;
use colored::*;
use std::collections::HashMap;

/// Offset encontrado por el dumper
#[derive(Debug, Clone)]
pub struct DumpedOffset {
    pub name: String,
    pub category: String,
    pub address: usize,
    pub rva: usize, // Relative Virtual Address
    pub offset_type: OffsetType,
    pub description: String,
    /// Offset (en bytes desde el inicio del constructor) del `LEA` que carga la
    /// VTable. Solo relevante para funciones; `None` para strings/vtables o
    /// cuando no se conoce y hay que localizarlo por escaneo.
    pub vtable_lea_hint: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum OffsetType {
    Function,
    VTable,
    StringRef,
}

/// Definición de un patrón a buscar
struct PatternDef {
    name: &'static str,
    category: &'static str,
    pattern: &'static str,
    offset_type: OffsetType,
    description: &'static str,
    /// Offset para leer VTable desde el constructor (LEA instrucción)
    vtable_lea_offset: Option<usize>,
}

/// Definición de un string a buscar
struct StringDef {
    name: &'static str,
    category: &'static str,
    target: &'static str,
    description: &'static str,
}

/// Dumper principal del juego
pub struct GameDumper<'a> {
    process: &'a GameProcess,
    memory: &'a [u8],
    pub offsets: Vec<DumpedOffset>,
    pub strings_found: Vec<DumpedOffset>,
    pub vtables: Vec<DumpedOffset>,
}

impl<'a> GameDumper<'a> {
    pub fn new(process: &'a GameProcess, memory: &'a [u8]) -> Self {
        GameDumper {
            process,
            memory,
            offsets: Vec::new(),
            strings_found: Vec::new(),
            vtables: Vec::new(),
        }
    }

    /// Define todos los patrones de funciones a buscar
    fn get_patterns() -> Vec<PatternDef> {
        vec![
            // ==================== JUGADOR ====================
            PatternDef {
                name: "BarkPlayerEntityCpnt::Ctor",
                category: "Player",
                pattern: "48 89 4C 24 08 48 83 EC 38 4C 8B C1 48 8D 05 ?? ?? ?? ?? 48 89 01 45 33 DB 4C 89 59 08 4C 89 59",
                offset_type: OffsetType::Function,
                description: "Constructor del componente principal del jugador",
                vtable_lea_offset: Some(12), // LEA en offset 12 de la función
            },
            // ==================== DAÑO ====================
            PatternDef {
                name: "oCEntityCpntBasicDamage::Ctor",
                category: "Damage",
                pattern: "48 89 4C 24 08 48 83 EC 48 4C 8B C1 48 8D 05 ?? ?? ?? ?? 48 89 01 45 33 DB 4C 89 59 08 4C 89 59",
                offset_type: OffsetType::Function,
                description: "Constructor del componente de daño",
                vtable_lea_offset: Some(12),
            },
            PatternDef {
                name: "oCDtEntityCpntDamageSettings::Ctor",
                category: "Damage",
                pattern: "48 89 4C 24 08 53 55 56 57 41 54 41 55 41 56 41 57 48 83 EC 58 48 8B D9 E8 ?? ?? ?? ?? 90 48 8D",
                offset_type: OffsetType::Function,
                description: "Constructor de settings de daño",
                vtable_lea_offset: None,
            },
            // ==================== CÁMARA ====================
            PatternDef {
                name: "oCEntityCpntBasicCamera::Ctor",
                category: "Camera",
                pattern: "48 89 4C 24 08 48 83 EC 38 4C 8B C1 48 8D 05 ?? ?? ?? ?? 48 89 01 45 33 DB 4C 89 59 08 4C 89 59",
                offset_type: OffsetType::Function,
                description: "Constructor de cámara básica",
                vtable_lea_offset: Some(12),
            },
            PatternDef {
                name: "oCEntityCpntTopDownCamera::Ctor",
                category: "Camera",
                pattern: "48 89 4C 24 08 48 83 EC 18 4C 8B C1 48 8D 05 ?? ?? ?? ?? 48 89 01 45 33 DB 4C 89 59 08 4C 89 59",
                offset_type: OffsetType::Function,
                description: "Constructor de cámara top-down",
                vtable_lea_offset: Some(12),
            },
            PatternDef {
                name: "SetupCameraShaderUniforms",
                category: "Camera",
                pattern: "48 89 5C 24 18 55 56 57 41 54 41 55 41 56 41 57 48 8D AC 24 10 FF FF FF 48 81 EC F0 01 00 00 4C",
                offset_type: OffsetType::Function,
                description: "Configura ViewMat, ViewProjMat y uniforms de cámara",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "CameraSceneContext::Ctor",
                category: "Camera",
                pattern: "48 89 4C 24 08 48 8D 05 ?? ?? ?? ?? 48 89 01 66 C7 41 08 00 00 45 33 C0 4C 89 41 10 4C 89 41 18",
                offset_type: OffsetType::Function,
                description: "Constructor del contexto de escena de cámara",
                vtable_lea_offset: Some(5),
            },
            // ==================== MOVIMIENTO ====================
            PatternDef {
                name: "oCEntityCpntBasicMove::Ctor",
                category: "Movement",
                pattern: "48 89 4C 24 08 48 83 EC 68 4C 8B C1 48 8D 05 ?? ?? ?? ?? 48 89 01 45 33 DB 4C 89 59 08 4C 89 59",
                offset_type: OffsetType::Function,
                description: "Constructor del componente de movimiento",
                vtable_lea_offset: Some(12),
            },
            // ==================== TARGETING ====================
            PatternDef {
                name: "oCEntityCpntGpnTargetComputer::Ctor",
                category: "Movement",
                pattern: "48 89 4C 24 08 55 53 57 48 8D 6C 24 B0 48 81 EC 50 01 00 00 4C 8B C1 48 8D 05 ?? ?? ?? ?? 48 89",
                offset_type: OffsetType::Function,
                description: "Constructor del sistema de targeting",
                vtable_lea_offset: None,
            },
            // ==================== PROYECTILES ====================
            PatternDef {
                name: "oCEntityCpntGpnProjectileAttack::Ctor",
                category: "Projectile",
                pattern: "48 89 4C 24 08 48 83 EC 48 4C 8B C1 48 8D 05 ?? ?? ?? ?? 48 89 01 45 33 DB 4C 89 59 08 4C 89 59",
                offset_type: OffsetType::Function,
                description: "Constructor de ataque con proyectil",
                vtable_lea_offset: Some(12),
            },
            // ==================== HABILIDADES ====================
            PatternDef {
                name: "oCDtEntityCpntAbilityController::Ctor",
                category: "Ability",
                pattern: "4C 8B DC 49 89 4B 08 53 55 56 57 41 54 41 56 41 57 48 81 EC 90 00 00 00 48 8B D9 48 8D 05 ?? ??",
                offset_type: OffsetType::Function,
                description: "Constructor del controlador de habilidades",
                vtable_lea_offset: None,
            },
            // ==================== SALUD ====================
            PatternDef {
                name: "oCNamedEventGainHealth::Ctor",
                category: "Health",
                pattern: "48 89 5C 24 10 48 89 4C 24 08 57 48 83 EC 30 48 8B D9 48 8D 05 ?? ?? ?? ?? 48 89 01 C7 41 08 02",
                offset_type: OffsetType::Function,
                description: "Constructor del evento de curación",
                vtable_lea_offset: Some(18),
            },
            // ==================== ENEMIGOS ====================
            PatternDef {
                name: "oCDtEnemyDefinition::Ctor",
                category: "Enemy",
                pattern: "4C 8B DC 49 89 4B 08 48 83 EC 28 48 8B D1 4C 8D 0D ?? ?? ?? ?? 4C 89 09 45 33 D2 4C 89 51 08 4C",
                offset_type: OffsetType::Function,
                description: "Constructor de definición de enemigo",
                vtable_lea_offset: None,
            },
            // ==================== ENTIDADES ====================
            PatternDef {
                name: "oCGameObject::Ctor",
                category: "Entity",
                pattern: "48 89 4C 24 08 48 8D 05 ?? ?? ?? ?? 48 89 01 48 8D 05 ?? ?? ?? ?? 48 89 01 66 C7 41 08 00 00 48",
                offset_type: OffsetType::Function,
                description: "Constructor de game object",
                vtable_lea_offset: Some(5),
            },
            PatternDef {
                name: "oCEntitySceneContext::Ctor",
                category: "Entity",
                pattern: "48 89 4C 24 08 55 53 56 57 41 54 41 56 41 57 48 8D 6C 24 D9 48 81 EC D0 00 00 00 48 8B D9 48 8D",
                offset_type: OffsetType::Function,
                description: "Constructor del contexto de escena de entidades",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "oCGameScene::Ctor",
                category: "Entity",
                pattern: "48 89 4C 24 08 53 55 56 57 41 56 48 83 EC 70 48 8B F2 4C 8B F1 E8 ?? ?? ?? ?? 90 48 8D 05 ?? ??",
                offset_type: OffsetType::Function,
                description: "Constructor de la escena principal",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "oIGameScene::Ctor",
                category: "Entity",
                pattern: "48 89 4C 24 08 53 56 57 48 83 EC 20 48 8B F9 48 8D 05 ?? ?? ?? ?? 48 89 01 33 F6 48 89 71 10 48",
                offset_type: OffsetType::Function,
                description: "Constructor de la escena base (datos de cámara)",
                vtable_lea_offset: Some(15),
            },
            // ==================== MISC ====================
            PatternDef {
                name: "oCHeroControllerNetworkData::Ctor",
                category: "Player",
                pattern: "40 53 48 83 EC 20 48 8B D9 48 89 4C 24 30 33 D2 41 B8 D0 00 00 00 E8 ?? ?? ?? ?? 48 8B CB E8 ??",
                offset_type: OffsetType::Function,
                description: "Constructor de datos de red del héroe",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "oCEntityGpnAroundPositionTraverser::Factory",
                category: "Entity",
                pattern: "40 53 48 83 EC 20 F0 FF 05 ?? ?? ?? ?? B9 90 00 00 00 E8 ?? ?? ?? ?? 48 8B D8 48 89 44 24 38 33",
                offset_type: OffsetType::Function,
                description: "Factory del traverser de entidades por posición",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "PlayerDebugTelemetry",
                category: "Player",
                pattern: "48 89 4C 24 08 55 53 56 57 41 54 41 55 41 56 41 57 48 8D AC 24 38 EE FF FF B8 C8 12 00 00 E8 ??",
                offset_type: OffsetType::Function,
                description: "Función de debug/telemetría del jugador (refs Player location, etc.)",
                vtable_lea_offset: None,
            },
            // ==================== RED / NETWORK ====================
            PatternDef {
                name: "oCEntityCpntNetwork::Ctor",
                category: "Network",
                pattern: "4C 8B DC 49 89 4B 08 53 57 48 81 EC 88 00 00 00 48 8B D9 48 8D 05 ?? ?? ?? ?? 48 89 01 33 FF 48",
                offset_type: OffsetType::Function,
                description: "Constructor del componente de red de entidad",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "oCEntityCpntNetworkSettings::Ctor",
                category: "Network",
                pattern: "48 89 5C 24 10 48 89 4C 24 08 57 48 83 EC 20 8B DA 48 8B F9 48 8D 05 ?? ?? ?? ?? 48 89 01 48 8D",
                offset_type: OffsetType::Function,
                description: "Constructor de settings de red de entidad",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "oCEntityCpntHitPointNetworkData::Ctor",
                category: "Network",
                pattern: "48 89 5C 24 10 48 89 4C 24 08 57 48 83 EC 20 8B DA 48 8B F9 48 8D 05 ?? ?? ?? ?? 48 89 01 48 8D",
                offset_type: OffsetType::Function,
                description: "Constructor de datos de red de hit points",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "oCEntityCpntValueNetworkData::Ctor",
                category: "Network",
                pattern: "40 53 48 83 EC 20 48 8B D9 48 89 4C 24 30 33 D2 41 B8 08 01 00 00 E8 ?? ?? ?? ?? 48 8B CB E8 ??",
                offset_type: OffsetType::Function,
                description: "Constructor de datos de red de valores de entidad",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "oCGlobalEntityValueNetworkData::Factory",
                category: "Network",
                pattern: "48 89 5C 24 08 57 48 83 EC 20 48 8B FA F0 FF 05 ?? ?? ?? ?? B9 30 00 00 00 E8 ?? ?? ?? ?? 48 8B",
                offset_type: OffsetType::Function,
                description: "Factory de datos de red de valores globales",
                vtable_lea_offset: None,
            },
            // ==================== ARMAS / ATAQUES ====================
            PatternDef {
                name: "oIDtEntityCpntAttack::Ctor",
                category: "Weapon",
                pattern: "48 89 4C 24 08 48 83 EC 38 4C 8B C1 48 8D 05 ?? ?? ?? ?? 48 89 01 45 33 DB 4C 89 59 08 4C 89 59",
                offset_type: OffsetType::Function,
                description: "Constructor de ataque base (interfaz)",
                vtable_lea_offset: Some(12),
            },
            PatternDef {
                name: "oIDtEntityCpntAttackSettings::Ctor",
                category: "Weapon",
                pattern: "48 89 4C 24 08 48 83 EC 28 48 8D 05 ?? ?? ?? ?? 48 89 01 48 8D 05 ?? ?? ?? ?? 48 89 41 08 48 8D",
                offset_type: OffsetType::Function,
                description: "Constructor de settings de ataque base",
                vtable_lea_offset: Some(9),
            },
            PatternDef {
                name: "oCDtEntityCpntAttackAim::Ctor",
                category: "Weapon",
                pattern: "48 89 4C 24 08 53 48 83 EC 30 48 8B D9 E8 ?? ?? ?? ?? 90 48 8D 83 D0 00 00 00 48 89 44 24 48 33",
                offset_type: OffsetType::Function,
                description: "Constructor de apuntado de ataque (AttackAim)",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "oCDtEntityCpntChargeAttack::Ctor",
                category: "Weapon",
                pattern: "48 89 5C 24 10 48 89 4C 24 08 57 48 83 EC 20 8B FA 48 8B D9 48 8D 05 ?? ?? ?? ?? 48 89 01 83 B9",
                offset_type: OffsetType::Function,
                description: "Constructor de ataque cargado (ChargeAttack)",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "oCDtEntityCpntAttackCombo::Ctor",
                category: "Weapon",
                pattern: "48 89 4C 24 08 53 48 83 EC 30 48 8B D9 E8 ?? ?? ?? ?? 90 48 8D 05 ?? ?? ?? ?? 48 89 03 45 33 C9",
                offset_type: OffsetType::Function,
                description: "Constructor de combo de ataque",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "oCEntityCpntZoneAttack::Ctor",
                category: "Weapon",
                pattern: "48 89 4C 24 08 48 83 EC 38 4C 8B C1 48 8D 05 ?? ?? ?? ?? 48 89 01 45 33 DB 4C 89 59 08 4C 89 59",
                offset_type: OffsetType::Function,
                description: "Constructor de ataque de zona (AoE)",
                vtable_lea_offset: Some(12),
            },
            PatternDef {
                name: "oCEntityCpntGpnTargetAttack::Ctor",
                category: "Weapon",
                pattern: "48 89 5C 24 10 48 89 4C 24 08 57 48 83 EC 20 8B DA 48 8B F9 48 8D 05 ?? ?? ?? ?? 48 89 01 48 81",
                offset_type: OffsetType::Function,
                description: "Constructor de ataque dirigido a target",
                vtable_lea_offset: None,
            },
            // ==================== MULTIJUGADOR ====================
            PatternDef {
                name: "oCStormancerSceneContext::MainHandler",
                category: "Multiplayer",
                pattern: "4C 8B DC 45 88 4B 20 4D 89 43 18 49 89 53 10 53 56 57 41 54 41 55 41 56 41 57 48 81 EC D0 05 00",
                offset_type: OffsetType::Function,
                description: "Handler principal de escena Stormancer (sesiones, P2P, party)",
                vtable_lea_offset: None,
            },
            PatternDef {
                name: "ConnectToStormancerScene",
                category: "Multiplayer",
                pattern: "48 83 EC 68 48 83 3D ?? ?? ?? ?? 00 0F 85 ?? ?? ?? ?? 80 3D ?? ?? ?? ?? 00 75 2B E8 ?? ?? ?? ??",
                offset_type: OffsetType::Function,
                description: "Conexión a escena de Stormancer (sesión/servidor)",
                vtable_lea_offset: None,
            },
        ]
    }

    /// Define todos los strings a buscar
    fn get_string_defs() -> Vec<StringDef> {
        vec![
            // Jugador
            StringDef { name: "LocalPlayerPosition", category: "Player", target: "Local player position", description: "Posición local del jugador" },
            StringDef { name: "PlayerLocation", category: "Player", target: "Player location", description: "Ubicación del jugador" },
            StringDef { name: "PlayerWatchingPosition", category: "Player", target: "Player watching position", description: "Posición de observación del jugador" },
            StringDef { name: "PlayerId", category: "Player", target: "Player id", description: "ID del jugador" },
            StringDef { name: "PlayerName", category: "Player", target: "PlayerName", description: "Nombre del jugador" },

            // Daño
            StringDef { name: "DamageReceived", category: "Damage", target: "Damage received", description: "Daño recibido" },
            StringDef { name: "DamageDealt", category: "Damage", target: "Damage dealt", description: "Daño causado" },
            StringDef { name: "FatalDamageReceived", category: "Damage", target: "Fatal damage received", description: "Daño fatal recibido" },
            StringDef { name: "CritDamage", category: "Damage", target: "Crit damage", description: "Daño crítico" },
            StringDef { name: "IncreaseDamageToBoss", category: "Damage", target: "Increase damage to boss", description: "Incremento de daño a boss" },
            StringDef { name: "BleedOnDamageReceive", category: "Damage", target: "Bleed on damage receive", description: "Sangrado al recibir daño" },
            StringDef { name: "DamageInterrupt", category: "Damage", target: "Damage interrupt", description: "Interrupción de daño" },
            StringDef { name: "ShowDamageAndHealth", category: "Damage", target: "Show damage and healing numbers", description: "Mostrar daño y números de curación" },

            // Salud
            StringDef { name: "MaxHealth", category: "Health", target: "Max health", description: "Salud máxima" },
            StringDef { name: "HealthPercent", category: "Health", target: "Health percent", description: "Porcentaje de salud" },
            StringDef { name: "HealthAbsolute", category: "Health", target: "Health (absolute)", description: "Salud absoluta" },
            StringDef { name: "HealthPercentOfMax", category: "Health", target: "Health (% of max)", description: "Salud porcentaje del máximo" },
            StringDef { name: "GainHealth", category: "Health", target: "GAIN_HEALTH", description: "Ganar salud" },
            StringDef { name: "HealFullHealth", category: "Health", target: "HEAL_FULL_HEALTH", description: "Curar salud completa" },
            StringDef { name: "IncreaseHealthDropRate", category: "Health", target: "Increase health globe drop rate", description: "Incrementar tasa de drop de globos de salud" },

            // Velocidad / Movimiento
            StringDef { name: "MoveSpeedRatio", category: "Movement", target: "Move Speed Ratio", description: "Ratio de velocidad de movimiento" },
            StringDef { name: "SpeedRatio", category: "Movement", target: "Speed Ratio", description: "Ratio de velocidad" },
            StringDef { name: "AngleSpeed", category: "Movement", target: "Angle speed", description: "Velocidad angular" },
            StringDef { name: "ConstantSpeed", category: "Movement", target: "ConstantSpeed", description: "Velocidad constante" },
            StringDef { name: "SpeedNorm", category: "Movement", target: "SpeedNorm", description: "Velocidad normalizada" },

            // Proyectiles
            StringDef { name: "Velocity", category: "Projectile", target: "Velocity", description: "Velocidad del proyectil" },
            StringDef { name: "VelocityScale", category: "Projectile", target: "VelocityScale", description: "Escala de velocidad" },

            // Cámara
            StringDef { name: "ViewProjMat", category: "Camera", target: "ViewProjMat", description: "Matriz View-Projection" },
            StringDef { name: "viewProj", category: "Camera", target: "viewProj", description: "View-Projection (uniform)" },
            StringDef { name: "g_ViewMat", category: "Camera", target: "g_ViewMat", description: "View Matrix global" },
            StringDef { name: "g_cameraNearFar", category: "Camera", target: "g_cameraNearFar", description: "Near/Far planes de cámara" },

            // Cooldowns / Habilidades
            StringDef { name: "Cooldown", category: "Ability", target: "Cooldown", description: "Cooldown de habilidad" },

            // Enemigos
            StringDef { name: "AllPlayers", category: "Enemy", target: "AllPlayers", description: "Todos los jugadores" },

            // Red / Network
            StringDef { name: "NetworkStats", category: "Network", target: "network_stats", description: "Estadísticas de red" },
            StringDef { name: "NetworkStatsUI", category: "Network", target: "Network Stats", description: "UI de estadísticas de red" },
            StringDef { name: "NetworkSessionSize", category: "Network", target: "network.session.size", description: "Tamaño de sesión de red" },
            StringDef { name: "NetworkSettings", category: "Network", target: "NetworkSettings", description: "Configuración de red" },
            StringDef { name: "NetworkDamage", category: "Network", target: "NETWORK_DAMAGE", description: "Daño de red" },
            StringDef { name: "IsConnectedToSession", category: "Network", target: "Is Connected to Session", description: "Conectado a sesión" },
            StringDef { name: "NoConnection", category: "Network", target: "NoConnection", description: "Sin conexión" },
            StringDef { name: "AutoReconnection", category: "Network", target: "Auto recconnection is disabled", description: "Reconexión automática deshabilitada" },
            StringDef { name: "SessionConnectionFail", category: "Network", target: "Session_Connection_Fail", description: "Fallo de conexión de sesión" },
            StringDef { name: "SessionConnectionError", category: "Network", target: "Session_Connection_Error_Title", description: "Error de conexión de sesión" },
            StringDef { name: "PartyConnectionError", category: "Network", target: "Party_Connection_Error_Title", description: "Error de conexión de party" },
            StringDef { name: "LoginFailed", category: "Network", target: "Login failed : ", description: "Login fallido" },
            StringDef { name: "LoginErrorRecoverable", category: "Network", target: "Login failed with recoverable error", description: "Login fallido con error recuperable" },
            StringDef { name: "LoginErrorUnrecoverable", category: "Network", target: "Login failed with unrecoverable error", description: "Login fallido con error irrecuperable" },
            StringDef { name: "SessionId", category: "Network", target: "session_id", description: "ID de sesión" },

            // URLs de servidores
            StringDef { name: "ServerUrlLive", category: "Network", target: "https://dt-live.passtechgames.com", description: "URL servidor live" },
            StringDef { name: "ServerUrlDev", category: "Network", target: "http://dt-dev.passtechgames.com:8888", description: "URL servidor dev" },
            StringDef { name: "ServerUrlLive2", category: "Network", target: "https://dt-live-2.passtechgames.com", description: "URL servidor live 2" },
            StringDef { name: "ServerUrlLive3", category: "Network", target: "https://dt-live-3.passtechgames.com", description: "URL servidor live 3" },
            StringDef { name: "ServerUrlLocalhost", category: "Network", target: "http://localhost", description: "URL servidor localhost" },

            // Estadísticas
            StringDef { name: "Statistics", category: "Statistics", target: "Statistics", description: "Estadísticas" },
            StringDef { name: "StatisticsCollection", category: "Statistics", target: "StatisticsCollection", description: "Colección de estadísticas" },
            StringDef { name: "StatisticsSample", category: "Statistics", target: "StatisticsSample", description: "Muestra de estadísticas" },
            StringDef { name: "StatisticsCollector", category: "Statistics", target: "Statistics collector", description: "Colector de estadísticas" },
            StringDef { name: "StatsLogsEnable", category: "Statistics", target: "StatsLogsEnable", description: "Habilitar logs de estadísticas" },
            StringDef { name: "StatsLogInterval", category: "Statistics", target: "StatsLogInterval", description: "Intervalo de logs de estadísticas" },
            StringDef { name: "StatsUiControllerSettings", category: "Statistics", target: "StatsUiControllerEntityCpntSettings", description: "Settings del controlador UI de estadísticas" },
            StringDef { name: "StatsUiController", category: "Statistics", target: "StatsUiControllerEntityCpnt", description: "Controlador UI de estadísticas" },
            StringDef { name: "Analytics", category: "Statistics", target: "analytics", description: "Analytics del juego" },

            // EOS / Steam / RakNet
            StringDef { name: "OnlineId", category: "Network", target: "OnlineId", description: "ID online" },
            StringDef { name: "EpicOnlineService", category: "Network", target: "epiconlineserv", description: "Epic Online Services" },
            StringDef { name: "SteamApi64", category: "Network", target: "steam_api64.dll", description: "Steam API DLL" },
            StringDef { name: "RakPeerInterface", category: "Network", target: "RakPeerInterface", description: "Interfaz RakNet Peer" },

            // ==================== ARMAS / ATAQUES / ITEMS ====================
            // Attack Power
            StringDef { name: "AttackPower", category: "Weapon", target: "Attack power", description: "Poder de ataque" },
            StringDef { name: "AttackPowerBasic", category: "Weapon", target: "Attack power basic", description: "Poder de ataque básico" },
            StringDef { name: "AttackPowerSecondary", category: "Weapon", target: "Attack power secondary", description: "Poder de ataque secundario" },
            StringDef { name: "AttackPowerPrimary", category: "Weapon", target: "Attack power primary", description: "Poder de ataque primario" },
            StringDef { name: "AttackPowerTrait", category: "Weapon", target: "Attack power trait", description: "Poder de ataque rasgo" },
            StringDef { name: "AttackPowerDefensive", category: "Weapon", target: "Attack power defensive", description: "Poder de ataque defensivo" },
            StringDef { name: "AttackPowerDash", category: "Weapon", target: "Attack power dash", description: "Poder de ataque dash" },
            StringDef { name: "AttackPowerUltimate", category: "Weapon", target: "Attack power ultimate", description: "Poder de ataque ultimate" },
            StringDef { name: "CounterAttackDamage", category: "Weapon", target: "Counter attack damage", description: "Daño de contraataque" },
            StringDef { name: "AttacksIgnoreResistance", category: "Weapon", target: "Attacks ignore resistance", description: "Ataques ignoran resistencia" },
            StringDef { name: "AttackPowerStr", category: "Weapon", target: "attack_power", description: "Poder de ataque (key)" },

            // Attack Types
            StringDef { name: "Attack", category: "Weapon", target: "Attack", description: "Ataque" },
            StringDef { name: "AttackSettings", category: "Weapon", target: "Attack Settings", description: "Settings de ataque" },
            StringDef { name: "DtAttackAim", category: "Weapon", target: "Dt Attack Aim", description: "Apuntado de ataque" },
            StringDef { name: "DtChargeAttack", category: "Weapon", target: "Dt Charge Attack", description: "Ataque cargado" },
            StringDef { name: "DtChargeAttackStep", category: "Weapon", target: "Dt Charge Attack Step", description: "Paso de ataque cargado" },
            StringDef { name: "DtAttackCombo", category: "Weapon", target: "Dt Attack Combo", description: "Combo de ataque" },
            StringDef { name: "AttackComboStepSettings", category: "Weapon", target: "Attack Combo Step Settings", description: "Settings de paso de combo" },
            StringDef { name: "AttackPosComputer", category: "Weapon", target: "Attack Pos Computer", description: "Computador de posición de ataque" },
            StringDef { name: "EntityAttack", category: "Weapon", target: "Entity Attack", description: "Ataque de entidad" },
            StringDef { name: "Weapon", category: "Weapon", target: "Weapon", description: "Arma" },

            // Charge
            StringDef { name: "ChargeToDamage", category: "Weapon", target: "charge to damage", description: "Carga a daño" },

            // Critical
            StringDef { name: "Critical", category: "Weapon", target: "critical", description: "Crítico" },
            StringDef { name: "CriticalHealth", category: "Weapon", target: "critical health", description: "Salud crítica" },

            // Skills
            StringDef { name: "Skills", category: "Weapon", target: "skills", description: "Habilidades" },
            StringDef { name: "SkillPropose", category: "Weapon", target: "Skill propose", description: "Propuesta de habilidad" },
            StringDef { name: "SkillSelected", category: "Weapon", target: "Skill selected", description: "Habilidad seleccionada" },
            StringDef { name: "SkillBetterQuality", category: "Weapon", target: "skill better quality chance", description: "Probabilidad de mejor calidad de habilidad" },
            StringDef { name: "SkillChoice", category: "Weapon", target: "skill choice", description: "Elección de habilidad" },
            StringDef { name: "SkillMenuOpen", category: "Weapon", target: "skill menu open", description: "Menú de habilidad abierto" },
            StringDef { name: "SkillName", category: "Weapon", target: "skill_name", description: "Nombre de habilidad" },
            StringDef { name: "SkillSelectedKey", category: "Weapon", target: "skill_selected", description: "Habilidad seleccionada (key)" },

            // Items / Calidad / Loot
            StringDef { name: "ItemQualityRare", category: "Weapon", target: "Item_Quality_Rare", description: "Calidad: Rara" },
            StringDef { name: "ItemQualityCommon", category: "Weapon", target: "Item_Quality_Common", description: "Calidad: Común" },
            StringDef { name: "ItemQualityLegendary", category: "Weapon", target: "Item_Quality_Legendary", description: "Calidad: Legendaria" },
            StringDef { name: "ItemQualityEpic", category: "Weapon", target: "Item_Quality_Epic", description: "Calidad: Épica" },
            StringDef { name: "ItemQualityPowerUp", category: "Weapon", target: "Item_Quality_Power_Up", description: "Calidad: Power Up" },
            StringDef { name: "ItemQualityCurse", category: "Weapon", target: "Item_Quality_Currse", description: "Calidad: Maldición" },
            StringDef { name: "ItemSelectable", category: "Weapon", target: "ItemSelectable", description: "Item seleccionable" },
            StringDef { name: "Item", category: "Weapon", target: "Item", description: "Item" },
            StringDef { name: "LootCount", category: "Weapon", target: "Loot Count", description: "Cantidad de loot" },
            StringDef { name: "Rarity", category: "Weapon", target: "rarity", description: "Rareza" },
            StringDef { name: "DropRate", category: "Weapon", target: "drop rate", description: "Tasa de drop" },
            StringDef { name: "Equiped", category: "Weapon", target: "Equiped", description: "Equipado" },

            // Inventario
            StringDef { name: "InventoryMenuOpen", category: "Weapon", target: "inventory menu open", description: "Menú de inventario abierto" },
            StringDef { name: "Inventory", category: "Weapon", target: "inventory", description: "Inventario" },
            StringDef { name: "InventoryMenu", category: "Weapon", target: "InventoryMenu", description: "Menú de inventario" },
            StringDef { name: "InventoryMenuUiCtrl", category: "Weapon", target: "InventoryMenuUiControllerEntityCpnt", description: "Controlador UI del inventario" },
            StringDef { name: "InventoryMenuUiSettings", category: "Weapon", target: "InventoryMenuUiControllerEntityCpntSettings", description: "Settings UI del inventario" },

            // ==================== MULTIJUGADOR ====================
            // Sesiones
            StringDef { name: "MatchmakingStart", category: "Multiplayer", target: "matchmaking_start", description: "Inicio de matchmaking" },
            StringDef { name: "MatchmakingEnd", category: "Multiplayer", target: "matchmaking_end", description: "Fin de matchmaking" },
            StringDef { name: "Matchmaking", category: "Multiplayer", target: "matchmaking", description: "Matchmaking" },
            StringDef { name: "MatchmakingOptions", category: "Multiplayer", target: "MatchmakingOptionsSettings", description: "Opciones de matchmaking" },
            StringDef { name: "SessionEnd", category: "Multiplayer", target: "Session end", description: "Fin de sesión" },
            StringDef { name: "IsSessionHost", category: "Multiplayer", target: "Is Session Host", description: "Es host de sesión" },
            StringDef { name: "GameSessionSize", category: "Multiplayer", target: "Game Session Size", description: "Tamaño de sesión" },
            StringDef { name: "GameSessionInfos", category: "Multiplayer", target: "GameSessionInfos", description: "Info de sesión de juego" },
            StringDef { name: "GameSession", category: "Multiplayer", target: "GameSession", description: "Sesión de juego" },
            StringDef { name: "P2PSessionCreationFailed", category: "Multiplayer", target: "P2P session creation failed", description: "Creación de sesión P2P fallida" },
            StringDef { name: "P2PSessionJoinFailed", category: "Multiplayer", target: "P2P session join failed", description: "Unión a sesión P2P fallida" },
            StringDef { name: "SessionFromHost", category: "Multiplayer", target: "session from host", description: "Sesión desde host" },
            StringDef { name: "MultiplayerConnection", category: "Multiplayer", target: "multiplayer connection", description: "Conexión multijugador" },
            StringDef { name: "MultiplayerScore", category: "Multiplayer", target: "MultiplayerScore", description: "Puntuación multijugador" },
            StringDef { name: "MultiplayerPng", category: "Multiplayer", target: "multiplayer.png", description: "Icono multijugador" },

            // Party / Grupo
            StringDef { name: "CreateLobby", category: "Multiplayer", target: "CreateLobby", description: "Crear lobby" },
            StringDef { name: "JoinLobby", category: "Multiplayer", target: "JoinLobby", description: "Unirse a lobby" },
            StringDef { name: "LobbyStr", category: "Multiplayer", target: "lobby", description: "Lobby" },
            StringDef { name: "Invite", category: "Multiplayer", target: "Invite", description: "Invitar" },
            StringDef { name: "PartyId", category: "Multiplayer", target: "partyId", description: "ID del party" },
            StringDef { name: "PartyDataToken", category: "Multiplayer", target: "partyDataToken", description: "Token de datos del party" },
            StringDef { name: "PartySceneIsNull", category: "Multiplayer", target: "Party scene is null", description: "Escena de party es null" },
            StringDef { name: "PartyDataBearerTokens", category: "Multiplayer", target: "PartyDataBearerTokens", description: "Tokens de autenticación del party" },
            StringDef { name: "CreatePartyDataToken", category: "Multiplayer", target: "Party.CreatePartyDataBearerToken", description: "Crear token de datos del party" },

            // P2P Session Context
            StringDef { name: "P2PSessionSceneContext", category: "Multiplayer", target: "oCDtP2PSessionSceneContext", description: "Contexto de escena P2P" },

            // Stormancer / Escenas
            StringDef { name: "SendingSessionJoinRequest", category: "Multiplayer", target: "Sending Session Join Request", description: "Enviando solicitud de unión a sesión" },
            StringDef { name: "MultiplayerModalUiCtrl", category: "Multiplayer", target: "MultiplayerModalUiControllerEntityCpnt", description: "Controlador UI modal multijugador" },
            StringDef { name: "MultiplayerModalUiSettings", category: "Multiplayer", target: "MultiplayerModalUiControllerEntityCpntSettings", description: "Settings UI modal multijugador" },

            // Replicación
            StringDef { name: "ReplicationManagerSceneCtx", category: "Multiplayer", target: "ReplicationManagerSceneContext", description: "Contexto de escena del replication manager" },
            StringDef { name: "ReplicationManagerCtxUpdate", category: "Multiplayer", target: "replication manager context update", description: "Update del contexto de replication manager" },
            StringDef { name: "ReplicationInterval", category: "Multiplayer", target: "ReplicationInterval", description: "Intervalo de replicación" },
            StringDef { name: "ReplicationPeriod", category: "Multiplayer", target: "ReplicationPeriod", description: "Período de replicación" },
            StringDef { name: "ReplicationManagerScene", category: "Multiplayer", target: "Replication manager scene context", description: "Contexto de escena del replication manager" },
            StringDef { name: "ReplicationManagerSceneStr", category: "Multiplayer", target: "replication manager scene context", description: "Contexto de escena del replication manager (lowercase)" },
        ]
    }

    /// Escanea todos los patrones de funciones
    pub fn scan_all_patterns(&mut self) {
        let patterns = Self::get_patterns();
        let mut found_count = 0;

        // Muchas clases comparten una firma de constructor idéntica. Para no
        // asignar el mismo address a dos nombres, se reparten las coincidencias
        // de forma determinista: cada PatternDef toma la primera coincidencia
        // (en orden de address) que aún no haya reclamado otro PatternDef.
        let mut used_addresses: HashMap<usize, String> = HashMap::new();

        for pat_def in &patterns {
            let results = scanner::pattern_scan(self.memory, pat_def.pattern, self.process.base_address);

            if results.is_empty() {
                println!(
                    "  {} {} {}",
                    "[✗]".red(),
                    pat_def.name.yellow(),
                    "- No encontrado".red()
                );
                continue;
            }

            // Tomar la primera coincidencia libre. `pattern_scan` ya devuelve los
            // resultados en orden ascendente de address, así que el reparto es
            // estable entre ejecuciones.
            let selected = results
                .iter()
                .find(|r| !used_addresses.contains_key(&r.address))
                .cloned();

            // Si todas las coincidencias ya fueron reclamadas hay menos funciones
            // en el binario que PatternDefs con esta firma: no se puede nombrar
            // esta entrada sin duplicar un address, así que se omite con aviso.
            if selected.is_none() {
                println!(
                    "  {} {} {}",
                    "[!]".yellow(),
                    pat_def.name.yellow(),
                    "- omitido (todas las coincidencias de esta firma ya asignadas)".yellow()
                );
                continue;
            }

            if let Some(result) = selected {
                let rva = result.address - self.process.base_address;
                println!(
                    "  {} {} @ RVA {}{} (Abs: 0x{:X}){}",
                    "[✓]".green(),
                    pat_def.name.white().bold(),
                    "0x".bright_cyan(),
                    format!("{:X}", rva).cyan(),
                    result.address,
                    if results.len() > 1 {
                        format!(" ({} coincidencias)", results.len()).yellow().to_string()
                    } else {
                        String::new()
                    }
                );

                used_addresses.insert(result.address, pat_def.name.to_string());

                self.offsets.push(DumpedOffset {
                    name: pat_def.name.to_string(),
                    category: pat_def.category.to_string(),
                    address: result.address,
                    rva,
                    offset_type: pat_def.offset_type.clone(),
                    description: pat_def.description.to_string(),
                    vtable_lea_hint: pat_def.vtable_lea_offset,
                });

                found_count += 1;
            }
        }

        println!();
        println!(
            "  {} {}/{} patrones encontrados",
            "[*]".cyan(),
            found_count,
            Self::get_patterns().len()
        );
    }

    /// Escanea todos los strings
    pub fn scan_all_strings(&mut self) {
        let string_defs = Self::get_string_defs();
        let mut found_count = 0;

        for str_def in &string_defs {
            let results = scanner::scan_string(self.memory, str_def.target, self.process.base_address);

            if results.is_empty() {
                println!(
                    "  {} \"{}\" {}",
                    "[✗]".red(),
                    str_def.target.yellow(),
                    "- No encontrado".red()
                );
                continue;
            }

            let result = &results[0];
            let rva = result.address - self.process.base_address;
            println!(
                "  {} \"{}\" @ RVA {}{}",
                "[✓]".green(),
                str_def.target.white(),
                "0x".bright_cyan(),
                format!("{:X}", rva).cyan()
            );

            self.strings_found.push(DumpedOffset {
                name: str_def.name.to_string(),
                category: str_def.category.to_string(),
                address: result.address,
                rva,
                offset_type: OffsetType::StringRef,
                description: str_def.description.to_string(),
                vtable_lea_hint: None,
            });

            found_count += 1;
        }

        println!();
        println!(
            "  {} {}/{} strings encontrados",
            "[*]".cyan(),
            found_count,
            string_defs.len()
        );
    }

    /// ¿Está `addr` dentro del rango virtual del módulo cargado?
    fn in_module(&self, addr: usize) -> bool {
        addr >= self.process.base_address
            && addr < self.process.base_address + self.process.module_size
    }

    /// Lee un puntero (u64) directamente del volcado en memoria a partir de una
    /// dirección absoluta. Devuelve `None` si cae fuera del buffer.
    fn read_ptr_from_dump(&self, addr: usize) -> Option<usize> {
        let rva = addr.checked_sub(self.process.base_address)?;
        if rva + 8 > self.memory.len() {
            return None;
        }
        Some(u64::from_le_bytes(self.memory[rva..rva + 8].try_into().ok()?) as usize)
    }

    /// Heurística de validación de VTable: una VTable real empieza con un
    /// puntero a función virtual, es decir su primer slot apunta a código
    /// dentro del propio módulo. Descarta LEAs que cargan strings, floats u
    /// otros datos que no son tablas de métodos.
    fn looks_like_vtable(&self, vtable_addr: usize) -> bool {
        match self.read_ptr_from_dump(vtable_addr) {
            Some(first_slot) => self.in_module(first_slot),
            None => false,
        }
    }

    /// Resuelve VTables desde constructores encontrados.
    ///
    /// Estrategia por constructor:
    ///   1. Si el PatternDef declaró la posición del `LEA` de la VTable
    ///      (`vtable_lea_hint`), se prueba primero; es la señal más fiable.
    ///   2. Si no, se recorren los primeros 64 bytes buscando el primer `LEA`
    ///      RIP-relative cuya dirección **parezca** una VTable (primer slot =
    ///      puntero a código del módulo).
    ///   3. Como último recurso se acepta el primer `LEA` dentro del módulo,
    ///      preservando el comportamiento anterior.
    pub fn resolve_vtables(&mut self) {
        let mut resolved_vtables = Vec::new();

        for offset in &self.offsets {
            if !matches!(offset.offset_type, OffsetType::Function) {
                continue;
            }

            let func_offset = offset.address - self.process.base_address;
            let search_size = std::cmp::min(64, self.memory.len() - func_offset);

            // (1) Intentar la posición declarada del LEA.
            let mut chosen: Option<(usize, &'static str)> = None;
            if let Some(hint) = offset.vtable_lea_hint {
                if let Some(addr) =
                    scanner::try_resolve_lea_at(self.memory, func_offset + hint, self.process.base_address)
                {
                    if self.in_module(addr) {
                        chosen = Some((addr, "hint"));
                    }
                }
            }

            // (2)/(3) Escaneo de respaldo por los primeros 64 bytes.
            if chosen.is_none() {
                let mut first_in_module: Option<usize> = None;
                for i in 0..search_size.saturating_sub(7) {
                    if let Some(addr) = scanner::try_resolve_lea_at(
                        self.memory,
                        func_offset + i,
                        self.process.base_address,
                    ) {
                        if !self.in_module(addr) {
                            continue;
                        }
                        if first_in_module.is_none() {
                            first_in_module = Some(addr);
                        }
                        if self.looks_like_vtable(addr) {
                            chosen = Some((addr, "escaneo"));
                            break;
                        }
                    }
                }
                if chosen.is_none() {
                    if let Some(addr) = first_in_module {
                        chosen = Some((addr, "fallback"));
                    }
                }
            }

            if let Some((vtable_addr, source)) = chosen {
                let vtable_rva = vtable_addr - self.process.base_address;
                let vtable_name = format!(
                    "{}_VTable",
                    offset.name.replace("::Ctor", "").replace("::Factory", "")
                );
                println!(
                    "  {} {} @ RVA {}{} (desde {} · {})",
                    "[✓]".green(),
                    vtable_name.white().bold(),
                    "0x".bright_cyan(),
                    format!("{:X}", vtable_rva).cyan(),
                    offset.name.yellow(),
                    source.bright_black()
                );

                resolved_vtables.push(DumpedOffset {
                    name: vtable_name,
                    category: offset.category.clone(),
                    address: vtable_addr,
                    rva: vtable_rva,
                    offset_type: OffsetType::VTable,
                    description: format!("VTable resuelta desde {}", offset.name),
                    vtable_lea_hint: None,
                });
            }
        }

        let count = resolved_vtables.len();
        self.vtables.extend(resolved_vtables);

        println!();
        println!(
            "  {} {} VTables resueltas",
            "[*]".cyan(),
            count
        );
    }

    /// Obtiene todos los offsets de una categoría
    pub fn get_by_category(&self, category: &str) -> Vec<&DumpedOffset> {
        let mut result: Vec<&DumpedOffset> = self
            .offsets
            .iter()
            .chain(self.strings_found.iter())
            .chain(self.vtables.iter())
            .filter(|o| o.category == category)
            .collect();
        result.sort_by_key(|o| o.address);
        result
    }

    /// Obtiene todas las categorías únicas
    pub fn get_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self
            .offsets
            .iter()
            .chain(self.strings_found.iter())
            .chain(self.vtables.iter())
            .map(|o| o.category.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        categories.sort();
        categories
    }

    /// Imprime resumen del dump
    pub fn print_summary(&self) {
        println!(
            "  {} Funciones encontradas: {}",
            "[📊]".cyan(),
            self.offsets.len().to_string().green().bold()
        );
        println!(
            "  {} Strings encontrados:   {}",
            "[📊]".cyan(),
            self.strings_found.len().to_string().green().bold()
        );
        println!(
            "  {} VTables resueltas:     {}",
            "[📊]".cyan(),
            self.vtables.len().to_string().green().bold()
        );
        println!(
            "  {} Total offsets:         {}",
            "[📊]".cyan(),
            (self.offsets.len() + self.strings_found.len() + self.vtables.len())
                .to_string()
                .green()
                .bold()
        );

        println!();
        println!("  {} Por categoría:", "[📋]".cyan());
        for cat in self.get_categories() {
            let items = self.get_by_category(&cat);
            println!(
                "     {} {}: {} offsets",
                "→".bright_cyan(),
                cat.yellow(),
                items.len()
            );
        }
    }
}
