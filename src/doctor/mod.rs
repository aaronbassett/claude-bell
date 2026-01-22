//! System health checks and diagnostics

mod checks;

pub use checks::{
    check_config_directory, check_config_file, check_icon_aliases, check_sound_aliases,
    run_all_checks, CheckResult, CheckStatus,
};
