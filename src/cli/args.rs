use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(
    name = "claude-bell",
    about = "macOS notifications for Claude Code",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    // Content flags
    /// Notification title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Notification subtitle
    #[arg(short, long)]
    pub subtitle: Option<String>,

    /// Notification message body
    #[arg(short, long)]
    pub message: Option<String>,

    /// Thumbnail image path or URL
    #[arg(short, long)]
    pub image: Option<String>,

    /// App icon (path or @alias)
    #[arg(long)]
    pub icon: Option<String>,

    /// Sound (system name, path, or @alias)
    #[arg(long)]
    pub sound: Option<String>,

    // Interaction flags
    /// Comma-separated action buttons
    #[arg(short, long, value_delimiter = ',')]
    pub actions: Option<Vec<String>>,

    /// Enable reply input with placeholder
    #[arg(short, long)]
    pub reply: Option<String>,

    /// URL to open when notification clicked
    #[arg(long)]
    pub url: Option<String>,

    // Behavior flags
    /// Keep notification on screen until dismissed
    #[arg(long)]
    pub persistent: bool,

    /// Override implicit persistence
    #[arg(long)]
    pub not_persistent: bool,

    /// Timeout duration (e.g., 30s, 5m)
    #[arg(long)]
    pub timeout: Option<String>,

    /// Default value on dismiss/timeout
    #[arg(long)]
    pub default: Option<String>,

    /// Value to return on dismiss
    #[arg(long)]
    pub on_dismiss: Option<String>,

    /// Value to return on timeout
    #[arg(long)]
    pub on_timeout: Option<String>,

    // Input flags
    /// Process newline-delimited JSON from stdin
    #[arg(long)]
    pub batch: bool,

    // Output flags
    /// JSON output targets (stdout, stderr, logs, response)
    #[arg(long, value_delimiter = ',')]
    pub json: Option<Vec<String>>,

    /// Pretty-print JSON output
    #[arg(long)]
    pub pretty: bool,

    /// Suppress stdout
    #[arg(long)]
    pub quiet: bool,

    /// Suppress all output
    #[arg(long)]
    pub silent: bool,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "warn")]
    pub log_level: String,

    // Template flags
    /// Use named template
    #[arg(long)]
    pub template: Option<String>,

    /// Template variables (key:value, repeatable)
    #[arg(long, value_delimiter = ',')]
    pub var: Option<Vec<String>>,
}

#[derive(Subcommand, Debug, Clone)]
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

#[derive(Subcommand, Debug, Clone)]
pub enum TemplateCommands {
    /// List all templates
    List,
    /// Show a template
    Show {
        name: String,
        /// Render with variables instead of showing raw template
        #[arg(long)]
        render: bool,
    },
    /// Create a new template
    Create {
        /// Template name (required for non-interactive)
        #[arg(long)]
        name: Option<String>,

        /// Title (required for non-interactive)
        #[arg(long)]
        title: Option<String>,

        /// Subtitle
        #[arg(long)]
        subtitle: Option<String>,

        /// Message
        #[arg(long)]
        message: Option<String>,

        /// Sound
        #[arg(long)]
        sound: Option<String>,

        /// Icon
        #[arg(long)]
        icon: Option<String>,

        /// Actions (comma-separated)
        #[arg(long, value_delimiter = ',')]
        actions: Option<Vec<String>>,

        /// Reply placeholder
        #[arg(long)]
        reply: Option<String>,

        /// URL
        #[arg(long)]
        url: Option<String>,

        /// Persistent
        #[arg(long)]
        persistent: bool,

        /// Read JSON from stdin
        #[arg(long)]
        json: bool,
    },
    /// Update an existing template
    Update {
        name: String,

        /// Title
        #[arg(long)]
        title: Option<String>,

        /// Subtitle
        #[arg(long)]
        subtitle: Option<String>,

        /// Message
        #[arg(long)]
        message: Option<String>,

        /// Sound
        #[arg(long)]
        sound: Option<String>,

        /// Icon
        #[arg(long)]
        icon: Option<String>,

        /// Actions (comma-separated)
        #[arg(long, value_delimiter = ',')]
        actions: Option<Vec<String>>,

        /// Reply placeholder
        #[arg(long)]
        reply: Option<String>,

        /// URL
        #[arg(long)]
        url: Option<String>,

        /// Persistent
        #[arg(long)]
        persistent: Option<bool>,

        /// Read JSON from stdin
        #[arg(long)]
        json: bool,
    },
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

#[derive(Subcommand, Debug, Clone)]
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

#[derive(Subcommand, Debug, Clone)]
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

#[derive(Subcommand, Debug, Clone)]
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
