use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Enable service
    Enable { service: String },

    /// Disable service
    Disable { service: String },

    /// Start service
    Start { service: String },

    /// Stop service
    Stop { service: String },

    /// Stop service
    Restart { service: String },

    /// Create new service
    New { service: String },

    /// Show service status
    Status { service: Option<String> },

    /// List services
    List,
}
