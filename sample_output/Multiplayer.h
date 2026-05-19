#pragma once
// ============================================================
// SISTEMA MULTIJUGADOR / SESIONES / P2P / PARTY
// Auto-generado por Game SDK Dumper
// Fecha: 2026-05-16 23:08:46
// ============================================================

namespace SDK
{
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
    {
        constexpr uintptr_t StormancerMainHandler  = 0x8DD890;   // Handler principal de escena Stormancer
        constexpr uintptr_t ConnectToScene          = 0x176EF0; // Conexión a escena/servidor
    }

    // ============================================================
    // VTables Multiplayer (estáticas)
    // ============================================================
    namespace MultiplayerVTables
    {
        // Stormancer
        constexpr uintptr_t oCStormancerSceneContext         = 0xEB0CA8;
        constexpr uintptr_t oCDtStormancerSteamSceneContext  = 0xEB0CA8;

        // Matchmaking
        constexpr uintptr_t oCMatchmakingOptionsSettings     = 0xEF3C40;

        // Session / Replication
        constexpr uintptr_t oCEntityReplicationCpnt          = 0xF52778;
        constexpr uintptr_t oCGameNamedEventNetwork          = 0xEF2D28;
    }

    // ============================================================
    // Strings de Sesiones
    // ============================================================
    namespace SessionStrings
    {
        constexpr uintptr_t MatchmakingStart       = 0xEF2340;
        constexpr uintptr_t MatchmakingEnd         = 0xEF2398;
        constexpr uintptr_t Matchmaking            = 0xEF2340;
        constexpr uintptr_t MatchmakingOptions     = 0xEF5770;
        constexpr uintptr_t SessionEnd             = 0xEEF168;
        constexpr uintptr_t IsSessionHost          = 0xEF0B70;
        constexpr uintptr_t GameSessionSize        = 0xEF0B40;
        constexpr uintptr_t GameSessionInfos       = 0xEDB2F8;
        constexpr uintptr_t GameSession            = 0xEC6CB8;
        constexpr uintptr_t P2PSessionCreateFailed = 0xEF3EF8;
        constexpr uintptr_t P2PSessionJoinFailed   = 0xEF3F98;
        constexpr uintptr_t SessionFromHost        = 0xEF3D96;
        constexpr uintptr_t MultiplayerConnection  = 0xEF3F81;
        constexpr uintptr_t MultiplayerScore       = 0xEEE703;
        constexpr uintptr_t MultiplayerPng         = 0xEED5AF;
    }

    // ============================================================
    // Strings de Party / Lobby
    // ============================================================
    namespace PartyStrings
    {
        constexpr uintptr_t CreateLobby            = 0xEE1EE6;
        constexpr uintptr_t JoinLobby              = 0xEE1EFE;
        constexpr uintptr_t Lobby                  = 0xEE1D51;
        constexpr uintptr_t Invite                 = 0xEE1F46;
        constexpr uintptr_t PartyId                = 0xEE1B3B;
        constexpr uintptr_t PartyDataToken         = 0xEE2638;
        constexpr uintptr_t PartySceneIsNull       = 0xEE26D0;
        constexpr uintptr_t PartyDataBearerTokens  = 0xEE1E1C;
        constexpr uintptr_t CreatePartyDataToken   = 0xEE1E55;
    }

    // ============================================================
    // Strings de P2P
    // ============================================================
    namespace P2PStrings
    {
        constexpr uintptr_t P2PSessionSceneContext = 0xEEAC70;
        constexpr uintptr_t SendingJoinRequest     = 0xEF80F0;
    }

    // ============================================================
    // Strings de UI Multiplayer
    // ============================================================
    namespace MultiplayerUiStrings
    {
        constexpr uintptr_t ModalUiController      = 0xEF5610;
        constexpr uintptr_t ModalUiSettings        = 0xEF57C0;
    }

    // ============================================================
    // Strings de Replicación
    // ============================================================
    namespace ReplicationStrings
    {
        constexpr uintptr_t ReplicationManagerScene    = 0xEE98F1;
        constexpr uintptr_t ReplicationManagerUpdate   = 0xF7D761;
        constexpr uintptr_t ReplicationInterval        = 0xF0EB00;
        constexpr uintptr_t ReplicationPeriod          = 0xF51353;
        constexpr uintptr_t ReplicationManagerSceneCtx = 0xF73AA0;
        constexpr uintptr_t ReplicationManagerSceneLow = 0xF7D791;
    }
}
