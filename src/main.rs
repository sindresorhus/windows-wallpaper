use std::path::{Path, PathBuf};

use clap::{command, Parser, Subcommand};
use wallpaper::{DesktopWallpaper, DesktopWallpaperPosition, Monitor};

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
        #[arg(short, long, default_value_t = 0)]
        monitor: usize,
    },

    /// Sets the current wallpaper
    #[command(arg_required_else_help = true)]
    Set {
        /// Path to wallpaper
        path: PathBuf,
        /// Specifies how the desktop wallpaper should be displayed.
        #[arg(
            short,
            long,
            default_value_t = DesktopWallpaperPosition::Span,
            default_missing_value = "always",
            value_enum
        )]
        scale: DesktopWallpaperPosition,
        /// Index of monitor starting from 0
        #[arg(short, long, default_value_t = 0)]
        monitor: usize,
    },
}

fn if_chosen_monitor_within_range(choice: usize, monitors: &[Monitor], func: impl Fn()) {
    if choice < monitors.len() {
        func();
    } else {
        eprintln!(
            "The available monitors are from 0 to {} but {choice} was given",
            monitors.len() - 1,
        );
    }
}

fn main() {
    let wallpaper = match DesktopWallpaper::new() {
        Ok(wallpaper) => wallpaper,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };
    let monitors = match wallpaper.get_monitors() {
        Ok(monitors) => monitors,
        Err(error) => {
            eprintln!("Failed to retrieve available monitors: {error}");
            return;
        }
    };

    let args = Cli::parse();
    match args.command {
        Commands::Get { monitor } => if_chosen_monitor_within_range(monitor, &monitors, || {
            match wallpaper.get_wallpaper(&monitors[monitor]) {
                Some(path) => println!("{}", path.display()),
                None => eprintln!("Failed to get the desktop wallpaper"),
            }
        }),
        Commands::Set {
            monitor,
            scale,
            path,
        } => if_chosen_monitor_within_range(monitor, &monitors, || {
            match wallpaper.set_wallpaper(&monitors[monitor], Path::new(&path), scale) {
                Ok(_) => (),
                Err(error) => eprintln!("Failed to set the desktop wallpaper: {error}"),
            }
        }),
    }
}
