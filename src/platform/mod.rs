#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub use macos::set_theme;

#[cfg(target_os = "linux")]
pub use linux::set_theme;

#[cfg(target_os = "windows")]
pub use windows::set_theme;

#[cfg(target_os = "windows")]
pub use windows::set_color;

#[cfg(not(target_os = "windows"))]
pub fn set_color<R: Runtime>(app: AppHandle<R>, color: u32) -> Result<(), &'static str> {

}
