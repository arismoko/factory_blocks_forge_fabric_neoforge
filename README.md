# Factory Blocks for Sunrise

49 industrial decorative blocks: panels, grates, vents, fans, pipes and hazard stripes.

A port of [Factory Blocks](https://github.com/matthewperiut/factory_blocks) by slaincow to [Sunrise](https://playsunrise.net) (Minecraft Java 26.3), as a content mod: the original's blocks, models, textures and strings, with no code.

## What it does

- 49 full-block industrial decorations: factory panels, grating, vents, riveted plates, pipes, hazard stripes and machinery faces.
- Fans that face the way you place them, including the always-running ones. As in the original's 26.3 build, the three redstone fans stay off: their redstone listener there replaces no game method, so nothing calls it.
- The original's connected-texture data, its recipes, drops and creative tab.

## Differences from the original

- Blocks show no tooltips, and use the iron block's sounds instead of the mod's own metal sound.

## Building

With the Sunrise mod tool (`cargo sunrise-mod`, from the Sunrise SDK):

```sh
cargo sunrise-mod check
cargo sunrise-mod build    # target/sunrise-mod/factory_blocks.smod
```

This version is made for the Sunrise Dev build `dev-2026-11-22-7cfdf81`.

## Licence and credits

Factory Blocks is by slaincow (https://github.com/matthewperiut/factory_blocks). Its textures, models, recipes and strings are slaincow's and are used here under the **MIT licence**, the original's; `LICENSE` holds the text and its copyright notice, and `LICENSE-NOTICE.md` says what this port carries.

Carried to Minecraft 26.3 by AdLf0208 ([AdLf0208/factory_blocks_forge_fabric_neoforge](https://github.com/AdLf0208/factory_blocks_forge_fabric_neoforge)), and ported to Sunrise by arismoko. This port is distributed under the same licence.
