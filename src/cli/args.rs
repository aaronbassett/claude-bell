use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "claude-bell",
    about = "macOS notifications for Claude Code",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Notification title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Notification subtitle
    #[arg(short, long)]
    pub subtitle: Option<String>,

    /// Notification message body
    #[arg(short, long)]
    pub message: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage notification templates
    Template {
        #[command(subcommand)]
        action: TemplateCommands,
    },
    /// Manage sound aliases
    Sound {
        #[command(subcommand)]
        action: SoundCommands,
    },
    /// Manage icon aliases
    Icon {
        #[command(subcommand)]
        action: IconCommands,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
    /// Run first-time setup wizard
    Setup,
    /// Check system health and configuration
    Doctor,
}

#[derive(Subcommand, Debug)]
pub enum TemplateCommands {
    /// List all templates
    List,
    /// Show a template
    Show { name: String },
    /// Create a new template
    Create,
    /// Update an existing template
    Update { name: String },
    /// Delete a template
    Delete { name: String },
    /// Validate templates
    Validate {
        /// Template name (or --all)
        name: Option<String>,
        #[arg(long)]
        all: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum SoundCommands {
    /// List sound aliases
    List,
    /// Add a sound alias
    Add {
        alias: String,
        path: String,
        #[arg(long)]
        cache: bool,
    },
    /// Remove a sound alias
    Remove {
        alias: String,
        #[arg(long)]
        with_file: bool,
    },
    /// Prune orphaned sounds
    Prune {
        /// What to prune: files, aliases, or both
        target: Option<String>,
    },
    /// Check sound alias health
    Doctor,
}

#[derive(Subcommand, Debug)]
pub enum IconCommands {
    /// List icon aliases
    List,
    /// Add an icon alias
    Add { alias: String, path: String },
    /// Remove an icon alias
    Remove {
        alias: String,
        #[arg(long)]
        with_bundle: bool,
    },
    /// Prune orphaned icons
    Prune {
        /// What to prune: bundles, aliases, or both
        target: Option<String>,
    },
    /// Check icon alias health
    Doctor,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Show current configuration
    Show {
        #[arg(long)]
        pretty: bool,
    },
    /// Set a configuration value
    Set { key: String, value: String },
    /// Unset a configuration value
    Unset { key: String },
    /// Reset configuration to defaults
    Reset,
    /// Validate configuration
    Validate {
        #[arg(long)]
        path: Option<String>,
    },
}
