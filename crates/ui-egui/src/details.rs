//! An app's details drawer: it slides in from the right over a dimmed grid. Hand-written copy comes
//! from `core::details`; the live facts (version, date, download size, release notes) from the
//! app's latest GitHub release. Text only: no pictures are fetched from anywhere.

use artcraft_launcher_core::CraftApp;

use artcraft_launcher_core::details::AppDetails;
use artcraft_launcher_core::platform::pick_asset;
use egui::{CornerRadius, Id, Margin, Order, Rect, RichText, Sense, Stroke, Ui, emath::easing, pos2, vec2};

use crate::app::LauncherUi;
use crate::model::AppEntry;
use crate::theme::{self, Tokens, rgb, tint};
use crate::views::{FOOTER_H, HEADER_H, date_text};
use crate::widgets::{self, Glyph, Kind};

const PAD: f32 = 24.0;
/// As tall as the name, headline, "Like …" and version lines beside it, so the icon fills the row.
const ICON: f32 = 112.0;
const SLIDE_SECS: f32 = 0.22;

impl LauncherUi {
    /// Draws the drawer for `self.detail`, sliding in and out. Call it every frame, after the
    /// main panels, so it covers them.
    pub(crate) fn details_drawer(&mut self, ctx: &egui::Context) {
        if let Some(id) = &self.detail
            && self.detail_shown.as_ref() != Some(id)
        {
            self.detail_shown = Some(id.clone());
        }
        // Asked every frame, even while closed, so the first opening starts from 0 and slides.
        let open = self.detail.is_some();
        let slide = if self.reduce_motion { 0.0 } else { SLIDE_SECS };
        let k = ctx.animate_bool_with_time_and_easing(Id::new("details-slide"), open, slide, easing::cubic_out);
        let Some(id) = self.detail_shown.clone() else { return };
        let Some(details) = AppDetails::by_id(&id) else {
            self.detail = None;
            self.detail_shown = None;
            return;
        };
        if k <= 0.0 && !open {
            self.detail_shown = None;
            return;
        }
        let Some(entry) = self.entry(&id).cloned() else { return };
        self.details_keys(ctx, open);

        let t = Tokens::get(ctx);
        let area = drawer_area(ctx);
        let width = (area.width() * 0.5).clamp(440.0, 680.0).min(area.width());
        // The drawer slides in from the right, or from the left in Arabic.
        let rtl = egui::is_rtl();
        let left = if rtl { area.left() - width + width * k } else { area.right() - width * k };
        self.details_scrim(ctx, area, k, &t);

        // Not constrained to the screen: the drawer is partly off it while it slides, and a constrained
        // area would keep its contents in place while only the panel behind them moved.
        egui::Area::new(Id::new("details-drawer")).order(Order::Foreground).fixed_pos(pos2(left, area.top())).constrain(false).show(ctx, |ui| {
            let panel = Rect::from_min_size(pos2(left, area.top()), vec2(width, area.height()));
            let painter = ui.painter();
            painter.add(theme::shadow(&t).as_shape(panel, CornerRadius::ZERO));
            painter.rect_filled(panel, CornerRadius::ZERO, t.bg);
            let edge = if rtl { panel.right() - 0.5 } else { panel.left() + 0.5 };
            painter.vline(edge, panel.y_range(), Stroke::new(1.0, t.line_strong));

            ui.set_min_size(panel.size());
            ui.set_max_size(panel.size());
            // A bar that stays on screen says there is more below the fold. egui's default one is
            // floating and invisible until the pointer is over it, so the drawer uses the solid style.
            ui.spacing_mut().scroll = egui::style::ScrollStyle::solid();
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing.y = 8.0;
                self.details_hero(ui, &entry, details, &t);
                egui::Frame::new().inner_margin(Margin::same(PAD as i8)).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    for p in details.paragraphs() {
                        ui.add(egui::Label::new(RichText::new(p).size(14.5).color(t.ink)).wrap());
                    }
                    ui.add_space(14.0);
                    features(ui, details, entry.app, &t);
                    ui.add_space(14.0);
                    self.release_section(ui, &entry, details, &t);
                });
            });
        });
    }

    /// Esc closes the drawer. Nothing happens under a dialog.
    fn details_keys(&mut self, ctx: &egui::Context, open: bool) {
        if open && self.dialog.is_none() && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.detail = None;
        }
    }

    /// The page behind the drawer, dimmed (egui can't blur what is under a layer); a click on it closes the drawer.
    fn details_scrim(&mut self, ctx: &egui::Context, area: Rect, k: f32, t: &Tokens) {
        egui::Area::new(Id::new("details-scrim")).order(Order::Middle).fixed_pos(area.min).show(ctx, |ui| {
            let (rect, resp) = ui.allocate_exact_size(area.size(), Sense::click());
            ui.painter().rect_filled(rect, CornerRadius::ZERO, theme::scrim(t, k));
            if resp.on_hover_cursor(egui::CursorIcon::Default).clicked() {
                self.detail = None;
            }
        });
    }

    /// The drawer's top: a wash of the app's colour under a 2 px rule, the close button, the name,
    /// version and main button, and the app's links.
    fn details_hero(&mut self, ui: &mut Ui, entry: &AppEntry, details: &AppDetails, t: &Tokens) {
        let app = entry.app;
        let app_color = rgb(app.color);
        let status = entry.status(self.platform, self.caps.manage_apps);
        let hero = egui::Frame::new().fill(tint(app_color, if t.dark { 0.09 } else { 0.07 })).inner_margin(Margin::same(PAD as i8)).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                widgets::hud(ui, tl!("App details"), t.faint);
                ui.with_layout(crate::views::far_layout(), |ui| {
                    if widgets::glyph_button(ui, Glyph::Close, tl!("Close (Esc)")).clicked() {
                        self.detail = None;
                    }
                });
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if let Some(icon) = crate::icons::app(app.id) {
                    ui.add(egui::Image::new(icon).fit_to_exact_size(vec2(ICON, ICON)));
                }
                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 5.0;
                    widgets::wordmark(ui, app, 32.0);
                    ui.add(egui::Label::new(RichText::new(details.headline).size(15.0).color(t.ink)).wrap());
                    // "Version 0.5.0 · Oct 8, 2026", or the installed-versus-latest line.
                    let version = crate::views::version_line(entry, &status);
                    if !version.trim().is_empty() {
                        ui.label(RichText::new(version).size(12.5).color(t.muted));
                    }
                });
            });
            ui.add_space(14.0);
            // The menu leaves out the links: they are the buttons right below.
            let row_w = ui.available_width();
            ui.horizontal(|ui| self.actions_in(ui, entry, &status, row_w, false));
            link_buttons(ui, app, row_w);
        });
        let r = hero.response.rect;
        ui.painter().rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 2.0)), CornerRadius::ZERO, app_color);
        ui.painter().hline(r.x_range(), r.bottom() - 0.5, t.hairline());
    }

    /// The facts of the latest release in cells, then its notes.
    fn release_section(&self, ui: &mut Ui, entry: &AppEntry, details: &AppDetails, t: &Tokens) {
        widgets::eyebrow(ui, "02", tl!("Latest release"));
        ui.add_space(4.0);
        let Some(r) = entry.release() else {
            ui.label(RichText::new(tl!("No release published yet")).color(t.muted));
            return;
        };
        let size = self.platform.and_then(|p| {
            let stems: Vec<&str> = entry.app.stems().collect();
            pick_asset(r, &stems, p, p.default_kinds()).map(|(a, _)| widgets::human_bytes(a.size))
        });
        let facts: Vec<(&str, String)> = [
            (tl!("Version"), Some(r.version.as_ref().map_or_else(|| r.tag.clone(), |v| v.to_string()))),
            (tl!("Released"), r.published_at.map(date_text)),
            (tl!("Download"), size),
            (tl!("Platforms"), Some(details.platforms.to_string())),
        ]
        .into_iter()
        .filter_map(|(label, value)| value.map(|v| (label, v)))
        .collect();
        for pair in facts.chunks(2) {
            ui.columns(2, |cols| {
                for (col, (label, value)) in cols.iter_mut().zip(pair) {
                    egui::Frame::new().stroke(t.hairline()).inner_margin(Margin::same(12)).show(col, |ui| {
                        ui.set_width(ui.available_width());
                        ui.spacing_mut().item_spacing.y = 2.0;
                        widgets::hud(ui, label, t.faint);
                        ui.add(egui::Label::new(RichText::new(value).size(14.0).color(t.ink_strong)).wrap());
                    });
                }
            });
        }
        ui.add_space(8.0);
        crate::notes::show(ui, &r.notes);
    }
}

/// Between the header and the footer, which stay usable. In a window shorter than the two bars
/// together there is no room left: an empty area, never a negative one.
fn drawer_area(ctx: &egui::Context) -> Rect {
    let full = ctx.content_rect();
    let top = (full.top() + HEADER_H).min(full.bottom());
    let bottom = (full.bottom() - FOOTER_H).max(top);
    Rect::from_min_max(pos2(full.left(), top), pos2(full.right(), bottom))
}

/// The app's links as a row of buttons sharing `row_w`; the ↗ says each opens the browser, like the links on the cards.
fn link_buttons(ui: &mut Ui, app: &CraftApp, row_w: f32) {
    let links = [(tl!("Website"), app.website()), (tl!("GitHub"), app.repo_url()), (tl!("Releases"), app.releases_url())];
    let gap = ui.spacing().item_spacing.x;
    let link_w = ((row_w - gap * (links.len() as f32 - 1.0)) / links.len() as f32).floor();
    ui.horizontal(|ui| {
        for (label, url) in links {
            if widgets::button(ui, &format!("{label} ↗"), Kind::Secondary, link_w).on_hover_text(url.as_str()).clicked() {
                ui.ctx().open_url(egui::OpenUrl::new_tab(url));
            }
        }
    });
}

/// The numbered feature cells.
fn features(ui: &mut Ui, details: &AppDetails, app: &CraftApp, t: &Tokens) {
    widgets::eyebrow(ui, "01", tl!("Features"));
    ui.add_space(4.0);
    for (i, f) in details.features.iter().enumerate() {
        egui::Frame::new().stroke(t.hairline()).inner_margin(Margin::symmetric(14, 12)).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 3.0;
            ui.horizontal(|ui| {
                widgets::hud(ui, &format!("{:02}", i + 1), t.app_ink(app));
                ui.label(RichText::new(f.title).size(15.0).family(theme::semibold()).color(t.ink_strong));
            });
            ui.add(egui::Label::new(RichText::new(f.text).size(13.5).color(t.muted)).wrap());
        });
    }
}
