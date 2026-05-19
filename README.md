# Ravenswatch SDK Dumper

> A Rust-based game SDK dumper for **Ravenswatch** (Passtech Games). Scans the live process, identifies VTables by pattern, resolves strings and **auto-generates a complete C++ SDK** with offsets, VTables and function addresses.

```
  ██████╗  █████╗ ███╗   ███╗███████╗    ██████╗ ██╗   ██╗███╗   ███╗██████╗ ███████╗██████╗
 ██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔══██╗██║   ██║████╗ ████║██╔══██╗██╔════╝██╔══██╗
 ██║  ███╗███████║██╔████╔██║█████╗      ██║  ██║██║   ██║██╔████╔██║██████╔╝█████╗  ██████╔╝
 ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║  ██║██║   ██║██║╚██╔╝██║██╔═══╝ ██╔══╝  ██╔══██╗
 ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ██████╔╝╚██████╔╝██║ ╚═╝ ██║██║     ███████╗██║  ██╗
  ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝    ╚═════╝  ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝
```

## 🎯 Target

- **Game**: Ravenswatch (Passtech Games)
- **Engine**: Custom proprietary `oC*` engine (NOT Unreal / NOT Unity)
- **Architecture**: x64 Windows PE
- **Networking**: Stormancer + RakNet + Epic Online Services + Steam API

## ⚙️ How it works

The dumper runs an automated 4-stage pipeline against the running game process:

1. **Process Open** — Locates `Ravenswatch.exe` PID, opens with `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ`, reads the full module memory.
2. **AOB Scan** — Scans for function constructor signatures (with wildcards) and resolves their RVAs.
3. **String Scan** — Identifies engine-internal strings (`PlayerLocation`, `ViewProjMat`, `Cooldown`, server URLs, etc.) and resolves XRefs.
4. **VTable Resolution** — From each known constructor, walks the `LEA reg, [rip+disp]` instruction that loads the VTable pointer and resolves it.
5. **SDK Generation** — Emits 14 organized C++ headers ready to `#include` in your project.

## 📦 Output

Generates a folder `SDK_dumped_Ravenswatch/` with:

| File | Contents |
|---|---|
| `SDK.h` | Entry point, helpers (`Vec3`, `Vec4`, `Mat4x4`, `GetBase()`, `Resolve()`) |
| `Offsets.h` | All VTables + Functions + Strings in one place |
| `Player.h` | `BarkPlayerEntityCpnt` (player position chain) |
| `Camera.h` | `oCEntityCpntBasicCamera` + `SetupCameraShaderUniforms` (ViewProjMat) |
| `Damage.h` | `oCEntityCpntBasicDamage` (impact pos, direction, amount) |
| `Health.h` | `oCNamedEventGainHealth` + Max/Percent/Absolute strings |
| `Movement.h` | `oCEntityCpntBasicMove` (velocity, target) |
| `Ability.h` | `AbilityController` (cooldown, charges, available) |
| `Entity.h` | `oCGameObject` + `oCEntitySceneContext` (3 entity lists) |
| `Enemy.h` | `oCDtEnemyDefinition` + enemy iteration notes |
| `Network.h` | EOS + RakNet + WinHTTP imports + server URLs |
| `Multiplayer.h` | Stormancer + party + lobby + matchmaking |
| `Weapon.h` | All attack types (charge, combo, zone, target, projectile) |
| `Projectile.h` | `oCEntityCpntGpnProjectileAttack` |

## 🔑 Key offsets discovered

```cpp
// Player position chain
BarkPlayerEntityCpnt + 0x140 -> oCEntity3dLocator
  +0x148 -> float X
  +0x150 -> float Y
  +0x158 -> float Z

// Enemy position chain (same locator pattern)
oCEntityCpntBasicMove + 0xE0 -> oCEntity3dLocator
  +0xE8 -> X, +0xF0 -> Y, +0xF8 -> Z

// Camera
oCEntityCpntBasicCamera + 0x98 -> Camera position locator
                       + 0xB8 -> Look-at target
                       + 0x12C -> FOV (60.0f)

// ViewProjection matrix (for WorldToScreen)
SetupCameraShaderUniforms @ 0x57B940
  -> renderer uniform array[82] = g_ViewProjMat (offset +48 in struct)

// Entity lists (3 of them for different entity categories)
oCEntitySceneContext + 0x2B8 / +0x2D0 / +0x2E8
```

## 🚀 Usage

```bash
# Build
cargo build --release

# Run (Ravenswatch must be open, run as Administrator)
cargo run --release
# > Process name: Ravenswatch.exe

# Output: ./SDK_dumped_Ravenswatch/
```

## 🧰 Dependencies

- `winapi` 0.3 — `processthreadsapi`, `memoryapi`, `tlhelp32`, `psapi`, `winnt`
- `windows-sys` — modern Windows API
- `colored` 2.1 — terminal output
- `chrono` 0.4 — timestamps in generated SDK

## 🔍 World-to-Screen formula

Using the dumped `g_ViewProjMat`:

```cpp
Vec4 clipPos = ViewProjMat * Vec4(worldPos.x, worldPos.y, worldPos.z, 1.0f);
float ndcX = clipPos.x / clipPos.w;
float ndcY = clipPos.y / clipPos.w;
float screenX = (ndcX + 1.0f) * 0.5f * screenWidth;
float screenY = (1.0f - ndcY) * 0.5f * screenHeight;
```

## ⚠️ Disclaimer

This project is for **educational and research purposes only**. Use against your own copy of the game. Do not use to interfere with online multiplayer sessions.

## 📝 Credits

**El Escarabajo** — [github.com/Escarabajoz](https://github.com/Escarabajoz)

Built with assembly, coffee & spite. 🪲
