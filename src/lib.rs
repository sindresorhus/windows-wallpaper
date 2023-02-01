use std::path::{Path, PathBuf};

use clap::ValueEnum;
use windows::{
    core::{Result, PCWSTR, PWSTR},
    Win32::{
        System::Com::{
            CoCreateInstance, CoFreeUnusedLibraries, CoInitialize, CoUninitialize,
            CLSCTX_LOCAL_SERVER,
        },
        UI::Shell::{
            DesktopWallpaper, IDesktopWallpaper, DESKTOP_WALLPAPER_POSITION, DWPOS_CENTER,
            DWPOS_FILL, DWPOS_FIT, DWPOS_SPAN, DWPOS_STRETCH, DWPOS_TILE,
        },
    },
};

#[derive(Debug, Clone, Copy, ValueEnum)]
#[repr(usize)]
pub enum DesktopWallpaperPosition {
    Center,
    Tile,
    Stretch,
    Fit,
    Fill,
    Span,
}

impl From<DesktopWallpaperPosition> for DESKTOP_WALLPAPER_POSITION {
    fn from(value: DesktopWallpaperPosition) -> Self {
        match value {
            DesktopWallpaperPosition::Center => DWPOS_CENTER,
            DesktopWallpaperPosition::Tile => DWPOS_TILE,
            DesktopWallpaperPosition::Stretch => DWPOS_STRETCH,
            DesktopWallpaperPosition::Fit => DWPOS_FIT,
            DesktopWallpaperPosition::Fill => DWPOS_FILL,
            DesktopWallpaperPosition::Span => DWPOS_SPAN,
        }
    }
}

#[derive(Debug)]
pub struct Monitor {
    pub monitor_index: PWSTR,
    pub wallpaper: PWSTR,
}

#[derive(Debug)]
pub struct DesktopWallpaper {
    interface: IDesktopWallpaper,
}

impl DesktopWallpaper {
    pub fn new() -> Result<Self> {
        let interface: IDesktopWallpaper;

        unsafe {
            CoInitialize(None)?;
            interface = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_LOCAL_SERVER)?;
        }

        Ok(Self { interface })
    }

    pub fn get_monitors(&self) -> Result<Vec<Monitor>> {
        let monitor_count = unsafe { self.interface.GetMonitorDevicePathCount()? };

        (0..monitor_count)
            .map(|index| -> Result<Monitor> {
                unsafe {
                    let monitor_index = self.interface.GetMonitorDevicePathAt(index)?;
                    let wallpaper = self.interface.GetWallpaper(PCWSTR(monitor_index.0))?;

                    Ok(Monitor {
                        monitor_index,
                        wallpaper,
                    })
                }
            })
            .collect()
    }

    pub fn get_wallpaper(&self, monitor: &Monitor) -> Option<PathBuf> {
        unsafe {
            let wallpaper: Result<PWSTR> =
                self.interface.GetWallpaper(PCWSTR(monitor.monitor_index.0));

            let wallpaper_string = match wallpaper {
                Ok(pwstr) => pwstr.to_string().ok()?,
                Err(_) => String::new(),
            };

            let path = Path::new(&wallpaper_string);

            if path.exists() && path.is_file() {
                if let Some(path) = path.parent() {
                    if path.is_dir() {
                        return Some(path.to_path_buf());
                    }
                }
            }

            None
        }
    }

    pub fn set_wallpaper(
        &self,
        monitor: &Monitor,
        path: &Path,
        position: DesktopWallpaperPosition,
    ) -> Result<()> {
        unsafe {
            let full_path = format!(
                "{}",
                path.canonicalize()
                    .expect("Could not canonicalize")
                    .display()
            );

            let mut encoded = full_path.encode_utf16().chain([0u16]).collect::<Vec<u16>>();

            self.interface.SetWallpaper(
                PCWSTR(monitor.monitor_index.0),
                PCWSTR(encoded.as_mut_ptr()),
            )?;

            self.interface.SetPosition(position.into())
        }
    }
}

impl Drop for DesktopWallpaper {
    fn drop(&mut self) {
        unsafe {
            CoFreeUnusedLibraries();
            CoUninitialize();
        }
    }
}
