use eframe::egui::{
    self, pos2, Color32, CursorIcon, Painter, PointerButton, Pos2, Rect, Response, Sense, Shape,
    Stroke, Ui, ViewportCommand,
};
use std::f32::consts::PI;

/// Clickable area with hand cursor.
pub fn clickable(ui: &Ui, rect: Rect, id: &str) -> Response {
    let r = ui.interact(rect, ui.id().with(id), Sense::click());
    if r.hovered() {
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }
    r
}

/// Subtle white wash on hover / press.
pub fn hover_wash(painter: &Painter, rect: Rect, rounding: f32, resp: &Response) {
    let alpha = if resp.is_pointer_button_down_on() {
        24
    } else if resp.hovered() {
        13
    } else {
        return;
    };
    painter.rect_filled(rect, rounding, Color32::from_white_alpha(alpha));
}

/// Perimeter of a rounded rectangle as a closed point list (for dashed borders).
pub fn rounded_path(rect: Rect, r: f32, seg: usize) -> Vec<Pos2> {
    let corners = [
        (pos2(rect.left() + r, rect.top() + r), PI),
        (pos2(rect.right() - r, rect.top() + r), 1.5 * PI),
        (pos2(rect.right() - r, rect.bottom() - r), 0.0),
        (pos2(rect.left() + r, rect.bottom() - r), 0.5 * PI),
    ];
    let mut pts = Vec::with_capacity(4 * (seg + 1) + 1);
    for (center, start) in corners {
        for i in 0..=seg {
            let a = start + 0.5 * PI * i as f32 / seg as f32;
            pts.push(pos2(center.x + r * a.cos(), center.y + r * a.sin()));
        }
    }
    pts.push(pts[0]);
    pts
}

pub fn dashed_rounded_rect(painter: &Painter, rect: Rect, r: f32, stroke: Stroke) {
    let pts = rounded_path(rect, r, 8);
    painter.extend(Shape::dashed_line(&pts, stroke, 5.0, 4.0));
}

/// Lets the user drag the (undecorated) window by this area; double-click toggles maximize.
pub fn window_drag_area(ui: &Ui, rect: Rect, id: &str) {
    let r = ui.interact(rect, ui.id().with(id), Sense::click_and_drag());
    if r.drag_started_by(PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }
    if r.double_clicked() {
        toggle_maximize(ui.ctx());
    }
}

/// Paints a (small) gradient texture stretched over `rect` with rounded corners.
pub fn paint_texture(ui: &Ui, tex: &egui::TextureHandle, rect: Rect, rounding: f32) {
    egui::Image::from_texture(egui::load::SizedTexture::new(tex.id(), rect.size()))
        .rounding(egui::Rounding::same(rounding))
        .paint_at(ui, rect);
}

pub fn toggle_maximize(ctx: &egui::Context) {
    let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    ctx.send_viewport_cmd(ViewportCommand::Maximized(!maximized));
}
