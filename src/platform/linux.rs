use crate::{Theme, ThemeOpt};
use futures_lite::StreamExt;
use gtk::prelude::GtkSettingsExt;
use gtk::Settings;
use std::sync::LazyLock;
use tauri::{async_runtime, command, AppHandle, Runtime};
use tintanum::{SchemePreference, SchemeProxy};
use tokio::sync::{
    mpsc::{channel, Sender},
    Mutex,
};

#[command]
pub fn set_theme<R: Runtime>(app: AppHandle<R>, theme: Theme) -> Result<(), &'static str> {
    app.get_theme_config().unwrap().set_theme(theme);
    match theme {
        Theme::Auto => {
            async_runtime::spawn(start_proxy(app));
        }
        Theme::Light => {
            update_theme(app, false);
            async_runtime::spawn(close_proxy());
        }
        Theme::Dark => {
            update_theme(app, true);
            async_runtime::spawn(close_proxy());
        }
    }
    Ok(())
}

fn update_theme<R: Runtime>(app: AppHandle<R>, dark_theme: bool) {
    let _ = app.run_on_main_thread(move || {
        if let Some(settings) = Settings::default() {
            settings.set_gtk_application_prefer_dark_theme(dark_theme);
        }
    });
}

static TX: LazyLock<Mutex<Option<Sender<()>>>> = LazyLock::new(|| Mutex::new(None));

async fn start_proxy<R: Runtime>(app: AppHandle<R>) {
    let mut mutex = TX.lock().await;
    if mutex.as_ref().is_some() {
        return;
    }

    let proxy = match SchemeProxy::with_new_connection().await {
        Ok(proxy) => proxy,
        Err(err) => {
            eprintln!("{:?}", err);
            return;
        }
    };

    let mut stream = match proxy.init_and_receive_changed().await {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("{:?}", err);
            return;
        }
    };

    let (tx, mut rx) = channel::<()>(1);
    *mutex = Some(tx);
    drop(mutex);

    // TODO: https://docs.rs/tokio/latest/tokio/task/index.html#cooperative-scheduling
    loop {
        tokio::select! {
            _ = rx.recv() => {
                break;
            }
            Some(preference) = stream.next() => {
                match preference {
                    SchemePreference::NoPreference | SchemePreference::Light => {
                        update_theme(app.clone() ,false);
                    }
                    SchemePreference::Dark => {
                        update_theme(app.clone() ,true);
                    }
                }
            }
        }
    }
}

async fn close_proxy() {
    let mut mutex = TX.lock().await;
    if let Some(tx) = mutex.as_ref() {
        let _ = tx.send(()).await;
        *mutex = None;
    }
}
