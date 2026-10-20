# Alaydriem's Light Levels

A client-side light-level overlay for the Minecraft Bedrock Windows client, loaded by
[LeviLamina](https://github.com/LiteLDev/LeviLamina). Use it to find the spots where hostile mobs can
spawn, and light them.

Press **Ctrl+L** to show or hide the overlay. Each block near you that a mob can stand on shows its light
level as a colored number. The number lies flat on the top face of the block. The game draws it in the
world, so blocks in front of it hide it.

## What the numbers mean

The number is the light in the space above the block at the current time of day: the brighter of the
block light and the sky light, with the sky light made darker for the time of day. Open ground shows 15
at noon and less at night.

| Color | Light |
| --- | --- |
| Red | 0 |
| Yellow | 1 to 7 |
| Green | 8 to 15 |

A block gets a number when its top face is full and solid, and the block above it has no collision (air,
a flower, a torch).

## Behaviour

- The overlay starts off in each world. Leave the world and it turns off.
- The key does nothing while a screen is open (chat, a sign, a book, the pause menu). L without Ctrl always reaches the game.
- Carpets and snow layers hide the number of the block under them.
- The numbers follow you and change after you place or break a light, within a quarter of a second.
- Change dimension and the overlay stays on and scans again.
- Hide the HUD (F1) and the numbers hide with it. The scan stops until the HUD comes back, and the
  numbers come back with it, updated within a quarter of a second.
- [Alaydriem's Free Camera](https://github.com/alaydriem/levi-freecam) hides the HUD when it turns on.
  Press F1 in the free camera to show the HUD and the numbers.

## Remote servers

The mod reads only the light data that your client already has, and it sends no packets. It works the
same on remote servers and in local worlds. Some servers do not allow client mods. Obey the rules of the
server you play on.

## Configuration

The mod reads `config.json` from its LeviLamina config folder when the game starts. A missing or broken
file gives the defaults and a warning in the log. A value out of range is clamped into its range, or, for
`passSeconds`, `lift` and `scale`, replaced by its default.

| Key | Default | Meaning |
| --- | --- | --- |
| `keys.toggle` | `76` (`L`) | Windows virtual-key code of the toggle key |
| `keys.ctrl` | `true` | Ctrl must be held with the toggle key |
| `scan.radius` | `24` | Radius in blocks, 1 to 32 |
| `scan.passSeconds` | `0.25` | Time for one full scan of the radius |
| `scan.maxLabels` | `8192` | Most numbers drawn; the nearest are kept |
| `scan.maxColumnsPerFrame` | `512` | Most block columns read in one frame |
| `light.redMax` | `0` | Light at or below this is red |
| `light.yellowMax` | `7` | Light at or below this (and above red) is yellow |
| `label.red`, `label.yellow`, `label.green` | see below | RGBA colors, each 0 to 1 |
| `label.lift` | `0.02` | Height of the number above the top face, 0 to 0.05 |
| `label.scale` | `1.0` | Size of the number |
| `telemetry` | `false` | Writes frame rate, cost and memory to the log every 2 seconds |
| `memoryBudgetMb` | `32` | Memory the overlay may use; telemetry warns above it |

Default colors: red `[1.0, 0.25, 0.25, 1.0]`, yellow `[1.0, 0.85, 0.2, 1.0]`, green
`[0.35, 1.0, 0.35, 1.0]`.

```json
{
  "keys": { "toggle": 76, "ctrl": true },
  "scan": { "radius": 16 },
  "light": { "redMax": 0, "yellowMax": 7 },
  "telemetry": true
}
```

## Build

Every build and test is a [mise](https://mise.jdx.dev) task. `mise tasks` lists them.

```bash
mise trust
mise run preflight
mise run build-mod     # builds the Rust library, the header and the mod into shim/bin/alaydriem-levels
mise run install-mod   # copies it to LM_MODS_DIR, set in mise.local.toml
```

## Licence

See [LICENSE](LICENSE).
