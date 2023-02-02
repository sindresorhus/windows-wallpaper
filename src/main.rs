use std::path::{Path, PathBuf};

use clap::{command, Parser};
use wallpaper::{DesktopWallpaper, DesktopWallpaperPosition, Monitor};

/// Manage the desktop wallpaper on Windows
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
enum Args {
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
            value_enum
        )]
        scale: DesktopWallpaperPosition,
        /// Index of monitor starting from 0
        #[arg(short, long, default_value_t = 0)]
        monitor: usize,
    },
}

fn if_chosen_monitor_within_range(
    choice: usize,
    monitors: &[Monitor],
    func: impl FnOnce(&Monitor) -> Result<(), String>,
) -> Result<(), String> {
    match monitors.get(choice) {
        Some(m) => func(m),
        None => Err(format!(
            "The available monitors are from 0 - {} but {choice} was given",
            monitors.len() - 1,
        )),
    }
}

fn main() -> Result<(), String> {
    let wallpaper = DesktopWallpaper::new().map_err(|e| e.to_string())?;
    let monitors = wallpaper
        .get_monitors()
        .map_err(|error| format!("Failed to retrieve available monitors: {error}"))?;

    let args = Args::parse();

    match args {
        Args::Get { monitor } => if_chosen_monitor_within_range(monitor, &monitors, |monitor| {
            Ok(println!("{}", wallpaper.get_wallpaper(monitor)?.display()))
        })?,
        Args::Set {
            monitor,
            scale,
            path,
        } => if_chosen_monitor_within_range(monitor, &monitors, |monitor| {
            wallpaper
                .set_wallpaper(monitor, Path::new(&path), scale)
                .map_err(|error| format!("Failed to set the desktop wallpaper: {error}"))
        })?,
    }

    Ok(())
}
