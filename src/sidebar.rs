use eframe::egui::{self, pos2, vec2, Align2, Color32, Rect, Rounding, Stroke};

use crate::app::LiteCutApp;
use crate::{icons, theme, widgets};

const USER_NAME: &str = "HUSS...I698";
const USER_ID: &str = "0182846297";
const SPACE_NAME: &str = "HUSSAIN MEHDI...";

pub fn show(ctx: &egui::Context, app: &mut LiteCutApp) {
    egui::SidePanel::left("sidebar")
        .exact_width(297.0)
        .resizable(false)
        .frame(egui::Frame::none().fill(theme::BG_SIDEBAR))
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let p = ui.painter().clone();
            let x0 = rect.left() + 34.0;
            let x1 = rect.right() - 34.0;

            // Draggable title area (window is undecorated).
            widgets::window_drag_area(
                ui,
                Rect::from_min_max(rect.min, pos2(rect.right(), rect.top() + 56.0)),
                "sidebar_drag",
            );

            // App mark + name (top-left).
            let mark = Rect::from_center_size(pos2(x0 - 6.0 + 9.0, rect.top() + 26.0), vec2(18.0, 18.0));
            p.rect_filled(mark, Rounding::same(5.0), theme::TEAL);
            icons::plus(&p, mark.center(), 11.0, Color32::BLACK, 2.0);
            p.text(
                pos2(mark.right() + 8.0, mark.center().y),
                Align2::LEFT_CENTER,
                "LiteCut",
                theme::bold(15.0),
                theme::TEXT_DIM,
            );

            // ---- Profile card ----
            let y0 = rect.top() + 67.0;
            let card = Rect::from_min_max(pos2(x0, y0), pos2(x1, y0 + 103.0));
            p.rect_filled(card, Rounding::same(12.0), Color32::from_rgb(36, 36, 39));

            let av = pos2(x0 + 32.0, y0 + 31.0);
            p.circle_filled(av, 21.0, Color32::from_rgb(52, 54, 66));
            icons::user(&p.with_clip_rect(Rect::from_center_size(av, vec2(42.0, 42.0))), av, 28.0, Color32::from_rgb(150, 156, 180));

            p.text(pos2(x0 + 66.0, y0 + 21.0), Align2::LEFT_CENTER, USER_NAME, theme::bold(16.0), theme::TEXT);
            p.text(pos2(x0 + 66.0, y0 + 44.0), Align2::LEFT_CENTER, USER_ID, theme::regular(15.0), theme::TEXT_DIM);

            let swap_c = pos2(x1 - 25.0, y0 + 32.0);
            let swap_rect = Rect::from_center_size(swap_c, vec2(34.0, 34.0));
            let r = widgets::clickable(ui, swap_rect, "swap_account");
            p.circle_filled(swap_c, 17.0, Color32::from_rgb(62, 62, 66));
            icons::swap(&p, swap_c, 15.0, theme::TEXT);
            if r.hovered() {
                p.circle_filled(swap_c, 17.0, Color32::from_white_alpha(14));
            }
            if r.clicked() {
                app.show_toast("Account switching arrives in a later phase");
            }

            let pro = Rect::from_min_max(pos2(x0 + 8.0, y0 + 63.0), pos2(x1 - 8.0, y0 + 95.0));
            let r = widgets::clickable(ui, pro, "join_pro");
            widgets::paint_texture(ui, &app.tex.join_pro, pro, 8.0);
            p.text(pro.center(), Align2::CENTER_CENTER, "Join Pro", theme::bold(16.0), Color32::from_rgb(10, 10, 12));
            widgets::hover_wash(&p, pro, 8.0, &r);
            if r.clicked() {
                app.show_toast("Pro plans are not part of Phase 1");
            }

            // ---- Navigation ----
            let nav_item = |ui: &egui::Ui, id: &str, cy: f32, selected: bool, label: &str, icon: &dyn Fn(egui::Pos2)| -> egui::Response {
                let row = Rect::from_min_max(pos2(x0, cy - 28.0), pos2(x1, cy + 28.0));
                let r = widgets::clickable(ui, row, id);
                if selected {
                    p.rect_filled(row, Rounding::same(10.0), theme::CARD_SELECTED);
                }
                widgets::hover_wash(&p, row, 10.0, &r);
                icon(pos2(x0 + 29.0, cy));
                p.text(pos2(x0 + 58.0, cy), Align2::LEFT_CENTER, label, theme::bold(17.0), theme::TEXT);
                r
            };

            nav_item(ui, "nav_home", y0 + 179.0, true, "Home", &|c| icons::home(&p, c, 20.0, theme::TEXT));
            let r = nav_item(ui, "nav_share", y0 + 249.0, false, "Share history", &|c| {
                icons::share(&p, c, 20.0, theme::TEXT)
            });
            if r.clicked() {
                app.show_toast("Share history is empty");
            }

            // ---- Spaces ----
            p.text(pos2(x0 + 16.0, y0 + 313.0), Align2::LEFT_CENTER, "Spaces", theme::bold(15.0), Color32::from_rgb(110, 110, 116));

            let space_row = Rect::from_min_max(pos2(x0, y0 + 336.0), pos2(x1, y0 + 376.0));
            let r = widgets::clickable(ui, space_row, "space_item");
            widgets::hover_wash(&p, space_row, 10.0, &r);
            let badge = Rect::from_center_size(pos2(x0 + 28.0, y0 + 356.0), vec2(24.0, 24.0));
            p.rect_filled(badge, Rounding::same(6.0), theme::RED);
            p.text(badge.center(), Align2::CENTER_CENTER, "H", theme::bold(14.0), Color32::WHITE);
            p.text(pos2(x0 + 52.0, y0 + 356.0), Align2::LEFT_CENTER, SPACE_NAME, theme::bold(16.0), theme::TEXT);

            let create_row = Rect::from_min_max(pos2(x0, y0 + 394.0), pos2(x1, y0 + 434.0));
            let r = widgets::clickable(ui, create_row, "create_or_join");
            widgets::hover_wash(&p, create_row, 10.0, &r);
            icons::plus(&p, pos2(x0 + 28.0, y0 + 414.0), 14.0, theme::TEXT, 1.8);
            p.text(pos2(x0 + 52.0, y0 + 414.0), Align2::LEFT_CENTER, "Create or join", theme::bold(16.0), theme::TEXT);
            if r.clicked() {
                app.show_toast("Spaces (teams) arrive in a later phase");
            }

            // ---- Bottom promo area ----
            let promo = Rect::from_min_max(
                pos2(x0, rect.bottom() - 35.0 - 109.0),
                pos2(x1, rect.bottom() - 35.0),
            );
            let tip = Rect::from_min_max(pos2(x0, promo.top() - 11.0 - 58.0), pos2(x1, promo.top() - 11.0));

            let r = widgets::clickable(ui, tip, "auto_captions");
            p.rect_filled(tip, Rounding::same(12.0), Color32::from_rgb(36, 36, 39));
            icons::bulb(&p, pos2(x0 + 28.0, tip.center().y), 20.0, theme::YELLOW);
            p.text(pos2(x0 + 56.0, tip.center().y - 10.0), Align2::LEFT_CENTER, "Recommended", theme::regular(14.0), theme::TEXT_DIM);
            p.text(pos2(x0 + 56.0, tip.center().y + 10.0), Align2::LEFT_CENTER, "Auto captions", theme::bold(17.0), theme::TEXT);
            widgets::hover_wash(&p, tip, 12.0, &r);
            if r.clicked() {
                app.show_toast("Auto captions arrive in a later phase");
            }

            let (tex, title, sub) = if app.promo_idx == 0 {
                (&app.tex.promo_longer, "Make it longer?", "Extend clips with AI")
            } else {
                (&app.tex.promo_sale, "Sale is LIVE", "Ends Oct 30")
            };
            widgets::paint_texture(ui, tex, promo, 12.0);
            p.text(pos2(x0 + 13.0, promo.top() + 60.0), Align2::LEFT_CENTER, title, theme::bold(20.0), Color32::WHITE);
            p.text(pos2(x0 + 13.0, promo.top() + 86.0), Align2::LEFT_CENTER, sub, theme::regular(14.0), Color32::from_rgb(225, 225, 232));

            // Carousel dots (click to switch; no timers -> zero idle CPU).
            for i in 0..2usize {
                let c = pos2(promo.left() + 105.0 + 12.0 * i as f32, promo.top() + 100.0);
                let hit = Rect::from_center_size(c, vec2(14.0, 14.0));
                let r = widgets::clickable(ui, hit, if i == 0 { "dot0" } else { "dot1" });
                let active = app.promo_idx == i;
                p.circle_filled(c, 2.6, if active { theme::TEAL } else { Color32::from_white_alpha(70) });
                if r.clicked() {
                    app.promo_idx = i;
                }
            }
            p.rect_stroke(promo, Rounding::same(12.0), Stroke::new(1.0, Color32::from_white_alpha(18)));
        });
}
