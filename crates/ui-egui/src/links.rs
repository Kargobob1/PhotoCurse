//! Project links: the Help menu, the About dialog and the start screen open these.

use serde_json::{Value, json};

use crate::PhotocraftApp;

/// The maintainer's Discord user name. Discord has no public profile URL for a user name, so the
/// UI shows it and copies it to the clipboard instead of opening a link.
pub const DISCORD_USER: &str = "execurse2";
/// Open `url` in the system browser (a new tab on the web) and note it in the status bar. Prefers
/// the platform `open_url` service (reliable on Windows/macOS/Linux); falls back to `ctx.open_url`.
pub fn open(app: &mut PhotocraftApp, ctx: &egui::Context, url: &str) -> Value {
    let opened = app.services.open_url.as_ref().map(|f| f(url).is_ok()).unwrap_or(false);
    if !opened {
        ctx.open_url(egui::OpenUrl::new_tab(url));
    }
    let template = "Opened {url}";
    app.ui.status = crate::i18n::fmt(tl!(template), &[("url", url)]);
    json!({"url": url})
}

/// Help › Discord: copy the maintainer's Discord name to the clipboard and say so.
pub fn copy_discord(app: &mut PhotocraftApp, ctx: &egui::Context) -> Value {
    ctx.copy_text(DISCORD_USER.to_string());
    let template = "Discord: {user} (copied to the clipboard)";
    app.ui.status = crate::i18n::fmt(tl!(template), &[("user", DISCORD_USER)]);
    json!({"discord": DISCORD_USER})
}

/// "Discord: execurse2", centred; a click copies the name.
pub fn discord_row(app: &mut PhotocraftApp, ui: &mut egui::Ui) {
    let t = crate::theme::Tokens::get(ui.ctx());
    let text = format!("Discord: {DISCORD_USER}");
    let clicked = ui
        .vertical_centered(|ui| ui.link(egui::RichText::new(text).size(12.5).color(t.accent)).on_hover_text(tl!("Copy the Discord name")).clicked())
        .inner;
    if clicked {
        copy_discord(app, ui.ctx());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_open_through_the_platform_service() {
        // Issue #14: links must open in the browser. They route through the `open_url` service
        // rather than the unreliable `ctx.open_url`.
        use std::sync::{Arc, Mutex};
        let opened: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let rec = opened.clone();
        let services = crate::Services {
            open_url: Some(Box::new(move |u: &str| {
                rec.lock().unwrap().push(u.to_string());
                Ok(())
            })),
            ..Default::default()
        };
        let mut app = PhotocraftApp::new(photocraft_engine::Session::new(), services);
        let ctx = egui::Context::default();
        let url = "https://example.com/";
        open(&mut app, &ctx, url);
        assert_eq!(opened.lock().unwrap().last().map(String::as_str), Some(url));
    }

    #[test]
    fn help_menu_lists_discord_then_separator_then_system_info_and_about() {
        let app = PhotocraftApp::new(photocraft_engine::Session::new(), Default::default());
        let help: Vec<String> = crate::menus::menu_items(&app).into_iter().filter(|i| i.path == ["Help"]).map(|i| i.id).collect();
        assert_eq!(help, ["help.discord", "---", "help.systemInfo", "help.about"]);
        assert!(crate::menus::is_live("help.discord") && crate::menus::is_enabled(&app, "help.discord"));
    }

    #[test]
    fn discord_shows_only_the_maintainer_name() {
        let mut app = PhotocraftApp::new(photocraft_engine::Session::new(), Default::default());
        let ctx = egui::Context::default();
        let r = crate::menus::invoke(&mut app, &ctx, "help.discord", json!({})).unwrap();
        assert_eq!(r["discord"], "execurse2");
        assert_eq!(app.ui.status, "Discord: execurse2 (copied to the clipboard)");
    }
}
