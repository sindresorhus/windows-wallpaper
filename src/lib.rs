use std::path::{Path, PathBuf};

use clap::ValueEnum;
use windows::{
    core::{Result, HSTRING, PCWSTR, PWSTR},
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
        let interface: IDesktopWallpaper = unsafe {
            CoInitialize(None)?;
            CoCreateInstance(&DesktopWallpaper, None, CLSCTX_LOCAL_SERVER)?
        };

        Ok(Self { interface })
    }

    pub fn get_monitors(&self) -> Result<Vec<Monitor>> {
        let monitor_count = unsafe { self.interface.GetMonitorDevicePathCount()? };

        (0..monitor_count)
            .map(|index| -> Result<Monitor> {
                let monitor_index = unsafe { self.interface.GetMonitorDevicePathAt(index)? };
                let wallpaper = unsafe { self.interface.GetWallpaper(PCWSTR(monitor_index.0))? };

                Ok(Monitor {
                    monitor_index,
                    wallpaper,
                })
            })
            .collect()
    }

    pub fn get_wallpaper(&self, monitor: &Monitor) -> Option<PathBuf> {
        let wallpaper: Result<PWSTR> =
            unsafe { self.interface.GetWallpaper(PCWSTR(monitor.monitor_index.0)) };

        let wallpaper_string = match wallpaper {
            Ok(pwstr) => unsafe { pwstr.to_string().ok()? },
            Err(error) => {
                eprintln!("{error}");
                return None;
            }
        };

        let path = Path::new(&wallpaper_string);

        (path.exists() && path.is_file()).then_some(path.to_path_buf())
    }

    pub fn set_wallpaper(
        &self,
        monitor: &Monitor,
        path: &Path,
        position: DesktopWallpaperPosition,
    ) -> Result<()> {
        unsafe {
            self.interface.SetWallpaper(
                PCWSTR(monitor.monitor_index.0),
                &HSTRING::from(path.as_os_str()),
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
