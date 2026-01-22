//! Sound and icon alias management

pub mod icon;
pub mod sound;

pub use sound::{
    add_sound_alias, list_sound_aliases, load_sound_aliases, remove_sound_alias, resolve_sound,
    save_sound_aliases, sound_files_dir, sounds_dir, SoundAliases,
};
