use std::{
    ffi::OsString,
    mem::ManuallyDrop,
    os::windows::prelude::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
};

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
    pub monitor_index: OsString,
    pub wallpaper: OsString,
}

#[derive(Debug)]
pub struct DesktopWallpaper {
    interface: ManuallyDrop<IDesktopWallpaper>,
}

impl DesktopWallpaper {
    pub fn new() -> Result<Self> {
        let interface: IDesktopWallpaper = unsafe {
            CoInitialize(None)?;
            CoCreateInstance(&DesktopWallpaper, None, CLSCTX_LOCAL_SERVER)?
        };

        Ok(Self {
            interface: ManuallyDrop::new(interface),
        })
    }

    pub fn get_monitors(&self) -> Result<Vec<Monitor>> {
        let monitor_count = unsafe { self.interface.GetMonitorDevicePathCount()? };

        (0..monitor_count)
            .map(|index| -> Result<Monitor> {
                let monitor_index = unsafe {
                    OsString::from_wide(self.interface.GetMonitorDevicePathAt(index)?.as_wide())
                };
                let wallpaper = unsafe {
                    OsString::from_wide(
                        self.interface
                            .GetWallpaper(PCWSTR::from_raw(
                                monitor_index.encode_wide().collect::<Vec<u16>>().as_ptr(),
                            ))?
                            .as_wide(),
                    )
                };

                Ok(Monitor {
                    monitor_index,
                    wallpaper,
                })
            })
            .collect()
    }

    pub fn get_wallpaper(&self, monitor: &Monitor) -> std::result::Result<PathBuf, String> {
        let wallpaper: PWSTR = unsafe {
            self.interface
                .GetWallpaper(PCWSTR(
                    monitor
                        .monitor_index
                        .encode_wide()
                        .collect::<Vec<u16>>()
                        .as_ptr(),
                ))
                .map_err(|e| e.to_string())?
        };

        let wallpaper_string = unsafe { OsString::from_wide(wallpaper.as_wide()) };

        let path = Path::new(&wallpaper_string);

        (path.exists() && path.is_file())
            .then_some(path.to_path_buf())
            .ok_or("Failed to get the desktop wallpaper".to_string())
    }

    pub fn set_wallpaper(
        &mut self,
        monitor: &Monitor,
        path: &Path,
        position: DesktopWallpaperPosition,
    ) -> Result<()> {
        unsafe {
            self.interface.SetWallpaper(
                PCWSTR(
                    monitor
                        .monitor_index
                        .encode_wide()
                        .collect::<Vec<u16>>()
                        .as_ptr(),
                ),
                &HSTRING::from(path.as_os_str()),
            )?;

            self.interface.SetPosition(position.into())
        }
    }
}

impl Drop for DesktopWallpaper {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.interface); // ensure to release of COM pointers before the CoUninitialize call
        }
        unsafe {
            CoFreeUnusedLibraries();
            CoUninitialize()
        }
    }
}
