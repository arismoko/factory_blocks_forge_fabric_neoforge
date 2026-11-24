//! Factory Blocks for Sunrise: the blocks, models and textures are content in `data/` and
//! `assets/`; this code gives them what the original's Java classes do.
//!
//! | Moment | Where | What it does |
//! |---|---|---|
//! | `register` | [`declare`] | attaches [`Metal`] to every block and [`Tip`] to every item |
//! | `sound_type` | [`Metal`] | the blocks' own `metal_sound` event (`FactorySound.FACTORY_BLOCKS`) |
//! | `play_hit_sound` | [`Metal`] | the same event while a block is mined |
//! | `tooltip` | [`Tip`] | the grey name line (`TooltipBlockItem.appendHoverText`) |
//! | `build_creative_mode_tab_contents` | this module | adds the blocks to Building Blocks |

use std::sync::OnceLock;

use mod_api::{
    self, BlockPart, BlockSoundType, CreativeTabPlacement, CreativeTabVisibility, Hooks, HostSide,
    ItemKey, ItemPart, ItemStack, Registrar, RegistrationContext, Text, TooltipEvent,
};

/// Every block and item the mod declares, in the order the original registers them
/// (`RegisterBlocks.register`), which is also their order in Building Blocks.
pub const NAMES: [&str; 49] = [
    "factory",
    "rust",
    "vrust",
    "srust",
    "wireframe",
    "pwireframe",
    "hazard",
    "hazardo",
    "circuit",
    "metalbox",
    "gcircuit",
    "pgcircuit",
    "grinder",
    "old_vents",
    "rust_plates",
    "bcircuit",
    "ice",
    "mosaic",
    "bwireframe",
    "rusty_scaffold",
    "caution",
    "large_pipes",
    "small_pipes",
    "vent",
    "gvent",
    "insulation",
    "gears",
    "cables",
    "rust_bplates",
    "grate",
    "rgrate",
    "hex",
    "wgpanel",
    "wopanel",
    "sturdy",
    "megacell",
    "exhaust",
    "engineer",
    "scaffold",
    "piping",
    "large_plating",
    "fan_side",
    "fan_on",
    "fan_four_on",
    "fan_malfunction_on",
    "fan",
    "fan_four",
    "fan_malfunction",
    "medium_fan",
];

const MOD: &str = "factory_blocks";
/// The sound event every block plays for every action (`FactorySound.METAL_BLOCK_SOUND`).
const METAL_SOUND: &str = "factory_blocks:metal_sound";
/// `ChatFormatting.GRAY` as the tooltip line colour, `rgba` packed.
const GRAY: u32 = 0xAAAA_AAFF;

/// The translation key of an item's tooltip line (`"item." + MODID + "." + name + ".tooltip"`).
pub fn tooltip_key(name: &str) -> String {
    format!("item.{MOD}.{name}.tooltip")
}

/// Point every sound of `sound` at the one metal event, with volume and pitch 1: the original's
/// `FACTORY_BLOCKS` sound type.
pub fn metal_sound(sound: &mut BlockSoundType) {
    let event: std::sync::Arc<str> = METAL_SOUND.into();
    sound.volume = 1.0;
    sound.pitch = 1.0;
    sound.break_sound = event.clone();
    sound.step = event.clone();
    sound.place = event.clone();
    sound.hit = event.clone();
    sound.fall = event;
}

/// The blocks' sounds.
pub struct Metal;

#[mod_api::sunrise_part]
impl BlockPart for Metal {
    fn sound_type(
        &self,
        _context: &dyn mod_api::LoopContext,
        event: &mut mod_api::SoundTypeEvent<'_>,
    ) {
        metal_sound(event.sound);
    }

    /// The mining hit sound (`ClientLevel.playBreakingSound` with the `FACTORY_BLOCKS` sound
    /// type): the metal event at the block's centre, at `(1 + 1) / 8` and pitch `1 * 0.5`.
    fn play_hit_sound(
        &self,
        host: &mut dyn mod_api::ClientHost,
        event: &mut mod_api::PlayHitSoundEvent,
    ) -> mod_api::Flow {
        let pos = event.pos;
        let centre = [
            f64::from(pos.x) + 0.5,
            f64::from(pos.y) + 0.5,
            f64::from(pos.z) + 0.5,
        ];
        let sound = mod_api::SoundPlay::at(METAL_SOUND, mod_api::SoundSource::Blocks, centre)
            .with_volume(0.25)
            .with_pitch(0.5);
        host.play_local_sound(&sound, 0);
        mod_api::Flow::Cancel
    }
}

/// One item's tooltip line.
pub struct Tip {
    key: String,
}

#[mod_api::sunrise_part]
impl ItemPart for Tip {
    fn tooltip(&self, _host: &dyn mod_api::ClientHost, event: &mut TooltipEvent<'_>) {
        event
            .lines
            .push((Text::translate(self.key.clone(), Vec::new()), GRAY));
    }
}

static ITEMS: OnceLock<Vec<ItemKey>> = OnceLock::new();

/// Attach the parts and keep the item keys for the creative tab.
pub fn declare(r: &mut dyn Registrar) {
    let mut items = Vec::with_capacity(NAMES.len());
    for name in NAMES {
        let block = r.block_key(&format!("{MOD}:{name}"));
        r.block_part(block, Metal);
        let item = r.item_key(&format!("{MOD}:{name}"));
        r.item_part(
            item,
            Tip {
                key: tooltip_key(name),
            },
        );
        items.push(item);
    }
    let _ = ITEMS.set(items);
}

#[derive(Default)]
struct Mod;

#[mod_api::sunrise_mod]
impl Hooks for Mod {
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        declare(r);
    }

    /// `RegisterBlocks.registerItemGroups`: every item goes to the end of Building Blocks.
    fn build_creative_mode_tab_contents(
        &self,
        host: HostSide,
        event: &mut mod_api::BuildCreativeModeTabContentsEvent<'_>,
    ) {
        if event.tab != "minecraft:building_blocks" {
            return;
        }
        let Some(items) = ITEMS.get() else { return };
        let host: &dyn mod_api::Host = match &host {
            HostSide::Server(host) => &**host,
            HostSide::Client(host) => &**host,
        };
        for item in items {
            let _ = event.edit.insert(
                host,
                CreativeTabPlacement::Last,
                ItemStack::of(*item),
                CreativeTabVisibility::Both,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::path::Path;

    fn root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn names_match_the_declared_blocks() {
        let on_disk: BTreeSet<String> =
            std::fs::read_dir(root().join("data/factory_blocks/sunrise/block"))
                .unwrap()
                .map(|f| {
                    f.unwrap()
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned()
                })
                .collect();
        let listed: BTreeSet<String> = NAMES.iter().map(|n| n.to_string()).collect();
        assert_eq!(on_disk, listed);
    }

    #[test]
    fn every_tooltip_is_translated() {
        let lang =
            std::fs::read_to_string(root().join("assets/factory_blocks/lang/en_us.json")).unwrap();
        for name in NAMES {
            assert!(
                lang.contains(&format!("\"{}\"", tooltip_key(name))),
                "{name}"
            );
        }
    }

    #[test]
    fn the_metal_event_is_defined_in_sounds_json() {
        let sounds =
            std::fs::read_to_string(root().join("assets/factory_blocks/sounds.json")).unwrap();
        let event = METAL_SOUND.strip_prefix("factory_blocks:").unwrap();
        assert!(sounds.contains(&format!("\"{event}\"")));
    }
}
