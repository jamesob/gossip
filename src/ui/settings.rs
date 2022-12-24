use super::GossipUi;
use crate::comms::BusMessage;
use crate::GLOBALS;
use eframe::egui;
use egui::widgets::{Button, Slider};
use egui::{Align, Context, Layout, Ui};

pub(super) fn update(
    app: &mut GossipUi,
    _ctx: &Context,
    _frame: &mut eframe::Frame,
    ui: &mut Ui,
    darkmode: bool,
) {
    ui.heading("Settings");

    ui.separator();

    ui.add_space(24.0);
    ui.heading("How Many Posts to Load");

    ui.horizontal(|ui| {
        ui.label("Feed Chunk: ").on_hover_text("This is the amount of time backwards from now that we will load events from. You'll eventually be able to load more, one chunk at a time. Mostly takes effect on restart.");
        ui.add(Slider::new(&mut app.settings.feed_chunk, 600..=86400).text("seconds, "));
        ui.label(secs_to_string(app.settings.feed_chunk));
    });

    ui.horizontal(|ui| {
        ui.label("Overlap: ").on_hover_text("If we recently loaded events up to time T, but restarted, we will now load events starting from time T minus overlap. Takes effect on restart.");
        ui.add(Slider::new(&mut app.settings.overlap, 0..=3600).text("seconds, "));
        ui.label(secs_to_string(app.settings.overlap));
    });

    ui.add_space(24.0);
    ui.heading("Feed Style / Order");

    ui.checkbox(&mut app.settings.view_threaded, "Threaded feed")
        .on_hover_text("If selected, replies are under what they reply to and the newest replied-to thread comes first. Otherwise all posts are independent and in time order.");

    ui.add_space(24.0);
    ui.heading("What Posts to Include");

    ui.checkbox(
        &mut app.settings.view_posts_referred_to,
        "View posts referred to by people you follow (not yet implemented)",
    )
    .on_hover_text(
        "Recommended, otherwise it's hard to understand what the person is talking about.",
    );

    ui.checkbox(&mut app.settings.view_posts_referring_to, "View posts referring to posts by people you follow (not yet implemented)")
        .on_hover_text("Not recommended, as anyone can reply to them and you'll certainly encounter spam this way.");

    ui.add_space(24.0);
    ui.heading("Miscellaneous");

    ui.checkbox(
        &mut app.settings.autofollow,
        "Autofollow everybody (not yet implemented)",
    )
    .on_hover_text("Definately not recommended. In fact we may remove this soon.");

    ui.add_space(24.0);
    ui.heading("Style");

    ui.horizontal(|ui| {
        ui.label("Switch to");

        #[allow(clippy::collapsible_else_if)]
        if darkmode {
            if ui
                .add(Button::new("☀ Light"))
                .on_hover_text("Switch to light mode")
                .clicked()
            {
                ui.ctx().set_visuals(super::style::light_mode_visuals());
            }
        } else {
            if ui
                .add(Button::new("🌙 Dark"))
                .on_hover_text("Switch to dark mode")
                .clicked()
            {
                ui.ctx().set_visuals(super::style::dark_mode_visuals());
            }
        }
    });

    ui.add_space(32.0);
    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        if ui.button("SAVE CHANGES").clicked() {
            let tx = GLOBALS.to_overlord.clone();
            let _ = tx.send(BusMessage {
                target: "overlord".to_string(),
                kind: "save_settings".to_string(),
                json_payload: serde_json::to_string(&app.settings).unwrap(),
            });
        }
    });
}

fn secs_to_string(secs: u64) -> String {
    let days = secs / 86400;
    let remainder = secs % 86400;
    let hours = remainder / 3600;
    let remainder = remainder % 3600;
    let minutes = remainder / 60;
    let seconds = remainder % 60;
    let mut output: String = String::new();
    if days > 0 {
        output.push_str(&format!(" {} days", days));
    }
    if hours > 0 {
        output.push_str(&format!(" {} hours", hours));
    }
    if minutes > 0 {
        output.push_str(&format!(" {} minutes", minutes));
    }
    output.push_str(&format!(" {} seconds", seconds));
    output
}