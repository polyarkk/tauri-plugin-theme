#![allow(unused_variables)]

mod platform;

use platform::set_theme;
use serde::{Deserialize, Serialize};
use std::sync::{LockResult, Mutex, MutexGuard};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{command, generate_handler, AppHandle, Config, Runtime};
use crate::platform::set_color;

static CONF: Mutex<ThemeConfig> = Mutex::new(ThemeConfig {
    theme: Theme::Auto,
    color: 0x000000,
});

pub struct ThemeConfig {
    theme: Theme,
    color: u32, // only supported in windows
}

impl ThemeConfig {
    pub fn get_theme(&self) -> Theme {
        self.theme
    }

    pub fn get_color(&self) -> u32 {
        self.color
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }
}

pub trait ThemeOpt {
    fn get_theme_config(&self) -> LockResult<MutexGuard<'_, ThemeConfig>>;
}

impl<R: Runtime> ThemeOpt for AppHandle<R> {
    fn get_theme_config(&self) -> LockResult<MutexGuard<'_, ThemeConfig>> {
        CONF.lock()
    }
}

pub fn init<R: Runtime>(config: &mut Config) -> TauriPlugin<R> {
    #[cfg(target_os = "windows")]
    {
        let theme = CONF.lock().unwrap().get_theme();

        for window in &mut config.app.windows {
            match theme {
                Theme::Auto => window.theme = None,
                Theme::Light => window.theme = Some(tauri::Theme::Light),
                Theme::Dark => window.theme = Some(tauri::Theme::Dark),
            }
        }
    }
    Builder::new("theme")
        .invoke_handler(generate_handler![get_theme, set_theme, get_color, set_color])
        .on_event(|app, e| {
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            if let tauri::RunEvent::Ready = e {
                if let Err(err) = set_theme(app.clone(), app.get_theme_config().unwrap().get_theme()) {
                    eprintln!("Failed to set theme: {}", err);
                }
            }
        })
        .build()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Auto,
    Light,
    Dark,
}

impl From<String> for Theme {
    fn from(value: String) -> Self {
        match value.as_str() {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::Auto,
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Auto => write!(f, "auto"),
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}

#[command]
fn get_theme<R: Runtime>(app: AppHandle<R>) -> Result<Theme, ()> {
    let theme = app.get_theme_config().unwrap().get_theme();
    Ok(theme)
}

#[command]
fn get_color<R: Runtime>(app: AppHandle<R>) -> Result<u32, ()> {
    let color = app.get_theme_config().unwrap().get_color();
    Ok(color)
}