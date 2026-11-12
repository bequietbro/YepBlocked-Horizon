
# YepBlocked for Horizon

## Project goal

Port YepBlocked from Minecraft Java Edition (Kotlin)
to Horizon Native Mods (Rust).

The goal is to preserve the behavior of the original
mod while implementing it using Horizon's native mod API.

## Reference implementation

The original Java/Kotlin implementation is the behavioral
reference.

Important source modules:
- common/: platform-independent logic
- fabric-26-3/: Minecraft 26.3 Fabric integration
- common/src/test/: existing behavioral tests

Do not silently change the original behavior during porting.

## Architecture

Keep the project separated into:

1. Core logic:
   - configuration
   - spawn rules
   - entity matching
   - command parsing
   - logging
2. Horizon integration:
   - native mod initialization
   - spawn-related hooks
   - command registration
   - configuration storage
3. Version compatibility:
   - version-specific adapters when necessary
   - shared code wherever APIs are compatible

Do not couple core logic to Horizon-specific types.

## Compatibility

The project aims to support multiple Horizon versions.

Never assume that a hook, API, structure, or function
exists in every Horizon revision.

Before using an API:
- inspect the pinned Horizon revision;
- inspect its actual source and documentation;
- verify the hook signature;
- verify the mixin behavior;
- document version-specific assumptions.

Do not claim compatibility with an untested version.

## Development rules

- Prefer small, independently testable changes.
- Preserve existing JSON configuration compatibility.
- Add tests for every new core behavior.
- Avoid unnecessary dependencies.
- Do not invent Horizon APIs or hook names.
- Do not replace working code without explaining why.
- Do not remove existing tests just to make builds pass.
- Do not modify generated metadata unless necessary.

## Before editing

1. Inspect the relevant source files.
2. Explain the intended implementation.
3. Identify API and compatibility risks.
4. Present a short implementation plan.

## After editing

Run the applicable checks:

- cargo fmt --check
- cargo test
- cargo check

Also run the appropriate Horizon-specific checks
when available.

Report:
- files changed;
- behavior implemented;
- commands executed;
- test results;
- unresolved compatibility issues.

## Git

- Never run git push without explicit approval.
- Never force-push.
- Never rewrite published history.
- Do not commit unrelated changes.
- Prefer one logical change per commit.
- Never commit secrets or local machine configuration.
