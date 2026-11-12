# YepBlocked

Mod for detailed mob/animal spawn configuration

A Horizon native mod by BEQUIETBRO, made with Horizon Translator's mod kit.

## What it does

- `/hello`.
- `/yepblocked` lists the commands and keys. Keys and options are in Options > Mods > YepBlocked.

## Install it

- **The easy way:** `yepblocked.hnmod` in this folder is the ready mod. Horizon Translator adds it to Horizon for you when "Add it to Horizon" is ticked; otherwise copy it into `%APPDATA%\horizon\horizon-native-mods\`. Then in the Horizon launcher's **Mods** page: **Build** under "Native build of Horizon" (if it isn't there, turn on Settings > Game > Native mods first), then play that build.
- **From this folder:** `horizon-launcher native-build "<this folder>" --trust-native-code`.

The launcher downloads its own Rust the first time (20 to 30 minutes), then builds the game with your native mods.

## Change it

| File | What it is |
|---|---|
| `mixins/yepblocked/runtime.rs` | The mod: its settings at the top, its logic, and tests at the bottom |
| `mixins/yepblocked/*.rs` (the others) | The hooks: each runs when Horizon calls it (a key, a tick, the HUD...) |
| `mixins/yepblocked/horizon-mixin.toml` | Which hook runs which file |
| `horizon-kit.json` | What you picked in the builder (open it in Horizon Translator to make a new version) |
| `HOOKS.md` | Every hook point and game helper of the Horizon it was made on (when the kit had its source) |
| `AGENTS.md` | Notes for an AI coding agent working on the mod |

Text and settings are at the top of `runtime.rs`. Edit them, then in Horizon Translator use **Make a mod > Check and
pack a folder** to check your code and pack a new `.hnmod`. With Rust installed, `cargo test` here runs the tests.

The guide that comes with Horizon Translator (Make a mod > Guide) lists every hook and game helper you can use.

Native mods run with the player's rights and aren't sandboxed: only share mods whose code you know.
