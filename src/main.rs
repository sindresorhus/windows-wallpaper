use std::{env, ffi::OsString, path::Path};

use clap::{command, Parser, Subcommand};
use wallpaper::{DesktopWallpaper, DesktopWallpaperPosition};

/// Manage the desktop wallpaper on Windows
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Gets current wallpaper
    Get {
        /// Index of monitor starting from 0
        #[arg(default_value_t = 0)]
        monitor: usize,
    },

    /// Sets the current wallpaper
    #[command(arg_required_else_help = true)]
    Set {
        /// Path to wallpaper
        path: OsString,
        /// Specifies how the desktop wallpaper should be displayed.
        #[arg(
            long,
            require_equals = true,
            num_args = 0..=1,
            default_value_t = DesktopWallpaperPosition::Span,
            default_missing_value = "always",
            value_enum
        )]
        scale: DesktopWallpaperPosition,
        /// Index of monitor starting from 0
        #[arg(require_equals = true, default_value_t = 0)]
        monitor: usize,
    },
}

fn main() -> windows::core::Result<()> {
    let wallpaper = DesktopWallpaper::new()?;
    let monitors = wallpaper.get_monitors()?;

    match env::args().len() > 1 {
        true => {
            let args = Cli::parse();
            match args.command {
                Commands::Get { monitor } => match wallpaper.get_wallpaper(&monitors[monitor]) {
                    Some(path) => println!("{}", path.display()),
                    None => eprintln!("Failed to get the desktop wallpaper"),
                },
                Commands::Set {
                    monitor,
                    scale,
                    path,
                } => match wallpaper.set_wallpaper(&monitors[monitor], Path::new(&path), scale) {
                    Ok(_) => (),
                    Err(_) => eprintln!("Failed to set the desktop wallpaper"),
                },
            }
        }

        false => {
            if let Some(path) = wallpaper.get_wallpaper(&monitors[0]) {
                println!("{}", path.display());
            }
        }
    }

    Ok(())
}
