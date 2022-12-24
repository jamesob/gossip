use super::GossipUi;
use crate::globals::GLOBALS;
use eframe::egui;
use egui::{Context, RichText, ScrollArea, TextStyle, Ui};

pub(super) fn update(_app: &mut GossipUi, _ctx: &Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
    ui.add_space(8.0);
    ui.heading("People Followed");
    ui.add_space(18.0);

    let people = GLOBALS.people.blocking_lock().clone();

    ScrollArea::vertical().show(ui, |ui| {
        for (_, person) in people.iter() {
            if person.followed != 1 {
                continue;
            }

            ui.label(&person.pubkey.0);

            ui.label(
                RichText::new(person.name.as_deref().unwrap_or(""))
                    .text_style(TextStyle::Name("Bold".into())),
            );

            ui.label(person.about.as_deref().unwrap_or(""));

            ui.label(person.dns_id.as_deref().unwrap_or(""));

            ui.add_space(12.0);

            ui.separator();
        }
    });
}