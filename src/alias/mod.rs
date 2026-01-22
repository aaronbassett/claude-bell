//! Sound and icon alias management

pub mod icon;
pub mod sound;

pub use icon::{
    add_icon_alias, icon_bundles_dir, icons_dir, list_icon_aliases, load_icon_aliases,
    remove_icon_alias, resolve_icon, save_icon_aliases, IconAliases, BUNDLED_ICONS,
};
pub use sound::{
    add_sound_alias, list_sound_aliases, load_sound_aliases, remove_sound_alias, resolve_sound,
    save_sound_aliases, sound_files_dir, sounds_dir, SoundAliases,
};
