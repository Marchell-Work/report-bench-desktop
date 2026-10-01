// Report Bench desktop shell.
//
// A thin native window around the live Report Bench site. Everything you change in
// the Cloudflare worker shows up here immediately (and the site's own "new version"
// banner handles those). The only thing this app updates itself for is a new
// version of the shell, which it checks for on every launch.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_updater::UpdaterExt;

/// The Report Bench site this app opens. Change it here if the address ever moves.
const APP_URL: &str = "https://reports.m-botha.workers.dev/";

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_url: url::Url = APP_URL.parse().expect("APP_URL is not a valid URL");
            let app_host = app_url.host_str().unwrap_or_default().to_string();
            let handle_for_nav = app.handle().clone();

            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(app_url))
                .title("Report Bench")
                .inner_size(1200.0, 850.0)
                .min_inner_size(420.0, 600.0)
                .center()
                .on_navigation(move |url| {
                    // Stay inside the window for Report Bench itself.
                    let same_site = matches!(url.scheme(), "http" | "https")
                        && url.host_str() == Some(app_host.as_str());
                    if same_site {
                        return true;
                    }
                    // "Open in Outlook" (mailto:) and any outside links go to the
                    // person's normal mail app / browser instead of replacing the app.
                    if matches!(url.scheme(), "mailto" | "http" | "https") {
                        let _ = handle_for_nav.opener().open_url(url.as_str(), None::<&str>);
                    }
                    false
                })
                .build()?;

            let handle = app.handle().clone();
            std::thread::spawn(move || {
                if let Err(err) = check_for_update(&handle) {
                    // No internet, GitHub down, etc. - just carry on; we'll check next launch.
                    eprintln!("Update check failed: {err}");
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Report Bench");
}

fn check_for_update(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let updater = app.updater()?;
    let Some(update) = tauri::async_runtime::block_on(updater.check())? else {
        return Ok(()); // already on the latest version
    };

    let wants_update = app
        .dialog()
        .message(format!(
            "A new version of the Report Bench app is available (version {}, you have {}).\n\n\
             Update now? Report Bench will close, install the update and reopen, \
             and you'll log in again.",
            update.version, update.current_version
        ))
        .title("Report Bench update")
        .kind(MessageDialogKind::Info)
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Update now".into(),
            "Later".into(),
        ))
        .blocking_show();

    if !wants_update {
        return Ok(());
    }

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_title("Report Bench - downloading update...");
    }
    // Downloads, checks the signature against the public key in tauri.conf.json,
    // then runs the installer (which closes and reopens the app on Windows).
    tauri::async_runtime::block_on(update.download_and_install(|_, _| {}, || {}))?;
    app.restart();
}
