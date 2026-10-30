use eframe::egui::{self, pos2, vec2, Align2, Color32, Rect, Rounding, Stroke, ViewportCommand};

use crate::app::LiteCutApp;
use crate::{icons, theme, widgets};

pub fn show(ctx: &egui::Context, app: &mut LiteCutApp) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(theme::BG_MAIN))
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let p = ui.painter().clone();
            let l = rect.left() + 33.0;
            let r = rect.right() - 33.0;
            let top = rect.top() + 67.0;

            window_controls(ui, rect);

            // ---- "Create project" banner ----
            let banner = Rect::from_min_max(pos2(l, top), pos2(r, top + 136.0));
            let resp = widgets::clickable(ui, banner, "create_project");
            widgets::paint_texture(ui, &app.tex.banner, banner, 20.0);
            p.rect_stroke(banner, Rounding::same(20.0), Stroke::new(1.0, Color32::from_white_alpha(28)));

            let label = "Create project";
            let galley = p.layout_no_wrap(label.to_owned(), theme::bold(24.0), Color32::WHITE);
            let icon_w = 34.0;
            let gap = 16.0;
            let total = icon_w + gap + galley.size().x;
            let start = banner.center().x - total / 2.0;
            let badge = Rect::from_center_size(pos2(start + icon_w / 2.0, banner.center().y), vec2(icon_w, icon_w));
            p.rect_filled(badge, Rounding::same(9.0), Color32::WHITE);
            icons::plus(&p, badge.center(), 20.0, Color32::from_rgb(20, 20, 22), 2.6);
            p.galley(
                pos2(start + icon_w + gap, banner.center().y - galley.size().y / 2.0),
                galley,
                Color32::WHITE,
            );
            widgets::hover_wash(&p, banner, 20.0, &resp);
            if resp.clicked() {
                app.show_toast("The editor arrives in Phase 2");
            }

            // ---- "Long video to shorts" ----
            let lv = Rect::from_min_size(pos2(l, top + 160.0), vec2(310.0, 72.0));
            let resp = widgets::clickable(ui, lv, "long_video");
            p.rect_filled(lv, Rounding::same(14.0), Color32::from_rgb(28, 28, 30));
            widgets::dashed_rounded_rect(&p, lv, 14.0, Stroke::new(1.0, theme::STROKE));
            let ic = pos2(lv.left() + 38.0, lv.center().y);
            p.circle_filled(ic, 16.0, theme::BLUE);
            icons::clips(&p, ic, 17.0, Color32::WHITE);
            p.text(pos2(lv.left() + 72.0, lv.center().y), Align2::LEFT_CENTER, "Long video to shorts", theme::bold(18.0), theme::TEXT);
            widgets::hover_wash(&p, lv, 14.0, &resp);
            if resp.clicked() {
                app.show_toast("Long video to shorts arrives in a later phase");
            }

            // ---- Projects header ----
            let cy = top + 283.0;
            p.text(
                pos2(l, cy),
                Align2::LEFT_CENTER,
                format!("Projects  ({})", app.projects.len()),
                theme::bold(18.0),
                theme::TEXT,
            );

            // Trash
            let trash = Rect::from_min_max(pos2(r - 93.0, cy - 16.0), pos2(r, cy + 16.0));
            let t_resp = widgets::clickable(ui, trash, "trash");
            p.rect_filled(trash, Rounding::same(8.0), theme::CARD);
            icons::trash(&p, pos2(trash.left() + 23.0, cy), 17.0, theme::TEXT);
            p.text(pos2(trash.left() + 41.0, cy), Align2::LEFT_CENTER, "Trash", theme::bold(15.0), theme::TEXT);
            widgets::hover_wash(&p, trash, 8.0, &t_resp);
            if t_resp.clicked() {
                app.show_toast("Trash is empty");
            }

            // View switcher
            let view = Rect::from_min_max(pos2(trash.left() - 12.0 - 54.0, cy - 16.0), pos2(trash.left() - 12.0, cy + 16.0));
            let v_resp = widgets::clickable(ui, view, "view_switch");
            p.rect_filled(view, Rounding::same(8.0), theme::CARD);
            icons::grid(&p, pos2(view.left() + 20.0, cy), 17.0, theme::TEXT);
            icons::chevron_down(&p, pos2(view.right() - 14.0, cy), 12.0, theme::TEXT);
            widgets::hover_wash(&p, view, 8.0, &v_resp);

            // Search
            let search = Rect::from_min_max(pos2(view.left() - 12.0 - 32.0, cy - 16.0), pos2(view.left() - 12.0, cy + 16.0));
            let s_resp = widgets::clickable(ui, search, "search");
            p.rect_filled(search, Rounding::same(8.0), theme::CARD);
            icons::search(&p, search.center(), 16.0, theme::TEXT);
            widgets::hover_wash(&p, search, 8.0, &s_resp);

            // ---- Empty state ----
            if app.projects.is_empty() {
                let area_top = cy + 16.0;
                let mid = (area_top + rect.bottom()) / 2.0;
                let cx = (l + r) / 2.0;
                let gy = mid - 20.0;
                let gray = Stroke::new(2.0, Color32::from_rgb(112, 112, 118));

                // Playhead + clip graphic
                let clip = Rect::from_center_size(pos2(cx, gy + 2.0), vec2(129.0, 43.0));
                p.rect_stroke(clip, Rounding::same(7.0), gray);
                let tab = Rect::from_min_max(pos2(cx - 7.5, gy - 49.0), pos2(cx + 7.5, gy - 33.0));
                p.rect_stroke(tab, Rounding::same(4.0), gray);
                p.line_segment([pos2(cx, gy - 33.0), pos2(cx, gy + 36.0)], gray);

                p.text(
                    pos2(cx, mid + 52.0),
                    Align2::CENTER_CENTER,
                    "Project edited will be saved automatically. Start creating your own videos!",
                    theme::regular(15.0),
                    Color32::from_rgb(150, 150, 156),
                );
            }
        });
}

/// Custom title-bar controls (the window is undecorated to match the reference design).
fn window_controls(ui: &egui::Ui, rect: Rect) {
    let p = ui.painter().clone();
    let ctx = ui.ctx().clone();
    let cy = rect.top() + 28.0;

    // Drag area = everything left of the buttons.
    widgets::window_drag_area(
        ui,
        Rect::from_min_max(rect.min, pos2(rect.right() - 230.0, rect.top() + 56.0)),
        "main_drag",
    );

    let button = |id: &str, x: f32, danger: bool, draw: &dyn Fn(egui::Pos2, Color32)| -> egui::Response {
        let hit = Rect::from_center_size(pos2(x, cy), vec2(40.0, 36.0));
        let resp = widgets::clickable(ui, hit, id);
        if resp.hovered() {
            let fill = if danger { Color32::from_rgb(196, 43, 28) } else { Color32::from_white_alpha(18) };
            p.rect_filled(hit, Rounding::same(6.0), fill);
        }
        draw(pos2(x, cy), theme::TEXT);
        resp
    };

    let right = rect.right();
    if button("wc_close", right - 24.0, true, &|c, col| icons::close(&p, c, 16.0, col)).clicked() {
        ctx.send_viewport_cmd(ViewportCommand::Close);
    }
    if button("wc_max", right - 68.0, false, &|c, col| icons::maximize(&p, c, 16.0, col)).clicked() {
        widgets::toggle_maximize(&ctx);
    }
    if button("wc_min", right - 112.0, false, &|c, col| icons::minimize(&p, c, 16.0, col)).clicked() {
        ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
    }
    p.line_segment(
        [pos2(right - 136.0, cy - 9.0), pos2(right - 136.0, cy + 9.0)],
        Stroke::new(1.0, Color32::from_white_alpha(35)),
    );
    button("wc_settings", right - 161.0, false, &|c, col| {
        icons::settings(&p, c, 19.0, col);
        p.circle_filled(pos2(c.x + 9.0, c.y - 11.0), 4.0, theme::TEAL);
    });
    button("wc_feedback", right - 205.0, false, &|c, col| icons::feedback(&p, c, 19.0, col));
}
