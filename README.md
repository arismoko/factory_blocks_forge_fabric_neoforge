# Factory Blocks for Sunrise

49 industrial decorative blocks: panels, grates, vents, fans, pipes and hazard stripes.

A port of [Factory Blocks](https://github.com/matthewperiut/factory_blocks) by slaincow to [Sunrise](https://playsunrise.net) (Minecraft Java 26.3), as a content mod with a small native part: the original's blocks, models, textures and strings, plus its tooltips, metal sounds and creative placement.

## What it does

- 49 full-block industrial decorations: factory panels, grating, vents, riveted plates, pipes, hazard stripes and machinery faces.
- Fans that face the way you place them, including the always-running ones. As in the original's 26.3 build, the three redstone fans stay off: their redstone listener there replaces no game method, so nothing calls it.
- A grey name line on every block's tooltip, the mod's own metal sound for placing, walking on, landing on and breaking them, and a place at the end of the Building Blocks tab.
- The original's connected-texture data, its recipes, drops and creative tab.

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
