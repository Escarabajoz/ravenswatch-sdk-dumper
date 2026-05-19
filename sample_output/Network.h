#pragma once
// ============================================================
// SISTEMA DE RED / NETWORK / ESTADÍSTICAS
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
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
    // Constructor: 0x704210
    // ============================================================
    namespace EntityCpntNetwork
    {
        constexpr uintptr_t Constructor = 0x704210;
    }

    // ============================================================
    // oCEntityCpntNetworkSettings
    // Constructor: 0xC3BC0
    // ============================================================
    namespace EntityCpntNetworkSettings
    {
        constexpr uintptr_t Constructor = 0xC3BC0;
    }

    // ============================================================
    // oCEntityCpntHitPointNetworkData
    // Constructor: 0xC3C30
    // ============================================================
    namespace HitPointNetworkData
    {
        constexpr uintptr_t Constructor = 0xC3C30;
    }

    // ============================================================
    // oCEntityCpntValueNetworkData
    // Constructor: 0x3005B0
    // ============================================================
    namespace ValueNetworkData
    {
        constexpr uintptr_t Constructor = 0x3005B0;
    }

    // ============================================================
    // oCGlobalEntityValueNetworkData
    // Factory: 0x211F10
    // ============================================================
    namespace GlobalEntityValueNetworkData
    {
        constexpr uintptr_t Factory = 0x211F10;
    }

    // ============================================================
    // URLs de Servidores
    // ============================================================
    namespace ServerUrls
    {
        constexpr uintptr_t Live      = 0xEED7B0;    // "https://dt-live.passtechgames.com"
        constexpr uintptr_t Dev       = 0xEED7D8;     // "http://dt-dev.passtechgames.com:8888"
        constexpr uintptr_t Live2     = 0xEED800;   // "https://dt-live-2.passtechgames.com"
        constexpr uintptr_t Live3     = 0xEED840;   // "https://dt-live-3.passtechgames.com"
        constexpr uintptr_t Localhost  = 0xEED828;   // "http://localhost"
    }

    // ============================================================
    // VTables de Red conocidas (estáticas)
    // ============================================================
    namespace NetworkVTables
    {
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
    }

    // ============================================================
    // Imports de red (IAT)
    // ============================================================
    namespace NetworkImports
    {
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
    }

    // ============================================================
    // Strings de Red
    // ============================================================
    namespace NetworkStrings
    {
        constexpr uintptr_t NetworkStats           = 0xEF28C0;
        constexpr uintptr_t NetworkStatsUI         = 0xEF8168;
        constexpr uintptr_t NetworkSessionSize     = 0xEF7330;
        constexpr uintptr_t NetworkSettings        = 0xEE4A1C;
        constexpr uintptr_t IsConnectedToSession   = 0xEF0B80;
        constexpr uintptr_t NoConnection           = 0xEE1D30;
        constexpr uintptr_t LoginFailed            = 0xEE15C0;
        constexpr uintptr_t SessionId              = 0xEF29B8;
    }

    // ============================================================
    // Strings de Estadísticas / Analytics
    // ============================================================
    namespace StatisticsStrings
    {
        constexpr uintptr_t Statistics             = 0xF0E478;
        constexpr uintptr_t StatisticsCollection   = 0xF461D2;
        constexpr uintptr_t StatisticsSample       = 0xF461EA;
        constexpr uintptr_t StatisticsCollector    = 0xF464E8;
        constexpr uintptr_t StatsLogsEnable        = 0xEED97B;
        constexpr uintptr_t StatsLogInterval       = 0xEED9A3;
        constexpr uintptr_t Analytics              = 0xEEED23;
    }
}
