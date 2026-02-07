//! CLI argument definitions.

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "rze")]
#[command(about = "Wallpaper-driven colorscheme generator", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Apply an image (path or URL) to wallpaper + theme.
    Img {
        input: String,

        /// Skip reload/restart actions.
        #[arg(long)]
        no_reset: bool,
    },

    /// Re-apply the last cached state in the current session.
    Apply {
        /// Skip reload/restart actions.
        #[arg(long)]
        no_reset: bool,
    },

    /// Materialize default templates into user config.
    Init {
        /// Overwrite existing user templates.
        #[arg(long)]
        force: bool,
    },

    /// Print env exports (for shell init).
    Env,

    /// (Planned) query wallhaven and return a cached image path.
    Wallhaven {
        query: String,

        /// Print URL instead of local cached path.
        #[arg(long)]
        print_url: bool,
    },
}
