//! Vector icons drawn directly with the egui painter.
//! No image files, no SVG parser: zero asset memory and always crisp at any DPI.

use eframe::egui::{pos2, Color32, Painter, Pos2, Rect, Rounding, Shape, Stroke};

fn p(c: Pos2, s: f32, x: f32, y: f32) -> Pos2 {
    pos2(c.x + x * s, c.y + y * s)
}

fn line(painter: &Painter, pts: &[Pos2], stroke: Stroke) {
    painter.add(Shape::line(pts.to_vec(), stroke));
}

pub fn home(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let st = Stroke::new(1.8, col);
    line(painter, &[p(c, s, -0.5, -0.02), p(c, s, 0.0, -0.5), p(c, s, 0.5, -0.02)], st);
    line(
        painter,
        &[p(c, s, -0.36, -0.14), p(c, s, -0.36, 0.46), p(c, s, 0.36, 0.46), p(c, s, 0.36, -0.14)],
        st,
    );
    line(
        painter,
        &[p(c, s, -0.12, 0.46), p(c, s, -0.12, 0.12), p(c, s, 0.12, 0.12), p(c, s, 0.12, 0.46)],
        st,
    );
}

pub fn share(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let a = p(c, s, 0.34, -0.34);
    let b = p(c, s, -0.34, 0.0);
    let d = p(c, s, 0.34, 0.34);
    let st = Stroke::new(1.6, col);
    painter.line_segment([a, b], st);
    painter.line_segment([b, d], st);
    for pt in [a, b, d] {
        painter.circle_filled(pt, s * 0.17, col);
    }
}

pub fn plus(painter: &Painter, c: Pos2, s: f32, col: Color32, w: f32) {
    let st = Stroke::new(w, col);
    painter.line_segment([p(c, s, -0.5, 0.0), p(c, s, 0.5, 0.0)], st);
    painter.line_segment([p(c, s, 0.0, -0.5), p(c, s, 0.0, 0.5)], st);
}

pub fn search(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let st = Stroke::new(1.8, col);
    painter.circle_stroke(p(c, s, -0.1, -0.1), s * 0.34, st);
    painter.line_segment([p(c, s, 0.16, 0.16), p(c, s, 0.46, 0.46)], st);
}

pub fn grid(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let st = Stroke::new(1.6, col);
    for (x, y) in [(-0.46, -0.46), (0.06, -0.46), (-0.46, 0.06), (0.06, 0.06)] {
        let r = Rect::from_min_size(p(c, s, x, y), eframe::egui::vec2(s * 0.4, s * 0.4));
        painter.rect_stroke(r, Rounding::same(s * 0.08), st);
    }
}

pub fn chevron_down(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    line(
        painter,
        &[p(c, s, -0.35, -0.15), p(c, s, 0.0, 0.2), p(c, s, 0.35, -0.15)],
        Stroke::new(1.8, col),
    );
}

pub fn trash(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let st = Stroke::new(1.7, col);
    painter.line_segment([p(c, s, -0.46, -0.28), p(c, s, 0.46, -0.28)], st);
    line(
        painter,
        &[p(c, s, -0.15, -0.28), p(c, s, -0.15, -0.46), p(c, s, 0.15, -0.46), p(c, s, 0.15, -0.28)],
        st,
    );
    line(
        painter,
        &[p(c, s, -0.34, -0.28), p(c, s, -0.28, 0.5), p(c, s, 0.28, 0.5), p(c, s, 0.34, -0.28)],
        st,
    );
    painter.line_segment([p(c, s, -0.1, -0.08), p(c, s, -0.1, 0.3)], st);
    painter.line_segment([p(c, s, 0.1, -0.08), p(c, s, 0.1, 0.3)], st);
}

pub fn swap(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let st = Stroke::new(1.7, col);
    painter.line_segment([p(c, s, -0.45, -0.2), p(c, s, 0.45, -0.2)], st);
    line(painter, &[p(c, s, 0.2, -0.42), p(c, s, 0.45, -0.2), p(c, s, 0.2, 0.0)], st);
    painter.line_segment([p(c, s, 0.45, 0.2), p(c, s, -0.45, 0.2)], st);
    line(painter, &[p(c, s, -0.2, 0.0), p(c, s, -0.45, 0.2), p(c, s, -0.2, 0.42)], st);
}

pub fn bulb(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    painter.circle_filled(p(c, s, 0.0, -0.1), s * 0.3, col);
    painter.rect_filled(
        Rect::from_min_max(p(c, s, -0.14, 0.2), p(c, s, 0.14, 0.38)),
        Rounding::same(s * 0.05),
        col,
    );
    let st = Stroke::new(1.4, col);
    painter.line_segment([p(c, s, 0.0, -0.58), p(c, s, 0.0, -0.5)], st);
    painter.line_segment([p(c, s, -0.5, -0.1), p(c, s, -0.58, -0.1)], st);
    painter.line_segment([p(c, s, 0.5, -0.1), p(c, s, 0.58, -0.1)], st);
}

pub fn feedback(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let st = Stroke::new(1.6, col);
    let r = Rect::from_min_max(p(c, s, -0.5, -0.45), p(c, s, 0.5, 0.25));
    painter.rect_stroke(r, Rounding::same(s * 0.12), st);
    line(painter, &[p(c, s, -0.15, 0.25), p(c, s, -0.15, 0.5), p(c, s, 0.15, 0.25)], st);
    for x in [-0.22, 0.0, 0.22] {
        painter.circle_filled(p(c, s, x, -0.1), s * 0.05, col);
    }
}

pub fn settings(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let st = Stroke::new(1.6, col);
    let pts: Vec<Pos2> = (0..6)
        .map(|i| {
            let a = std::f32::consts::FRAC_PI_3 * i as f32 + std::f32::consts::FRAC_PI_6;
            p(c, s, 0.52 * a.cos(), 0.52 * a.sin())
        })
        .collect();
    painter.add(Shape::closed_line(pts, st));
    painter.circle_stroke(c, s * 0.2, st);
}

pub fn minimize(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    painter.line_segment([p(c, s, -0.4, 0.0), p(c, s, 0.4, 0.0)], Stroke::new(1.4, col));
}

pub fn maximize(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let r = Rect::from_min_max(p(c, s, -0.38, -0.38), p(c, s, 0.38, 0.38));
    painter.rect_stroke(r, Rounding::same(1.0), Stroke::new(1.4, col));
}

pub fn close(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let st = Stroke::new(1.4, col);
    painter.line_segment([p(c, s, -0.4, -0.4), p(c, s, 0.4, 0.4)], st);
    painter.line_segment([p(c, s, 0.4, -0.4), p(c, s, -0.4, 0.4)], st);
}

/// Two overlapping rounded squares (used inside the blue "Long video to shorts" badge).
pub fn clips(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    let st = Stroke::new(1.5, col);
    let back = Rect::from_min_max(p(c, s, -0.2, -0.5), p(c, s, 0.5, 0.2));
    let front = Rect::from_min_max(p(c, s, -0.5, -0.2), p(c, s, 0.2, 0.5));
    painter.rect_stroke(back, Rounding::same(s * 0.1), st);
    painter.rect_filled(front, Rounding::same(s * 0.1), col);
}

pub fn user(painter: &Painter, c: Pos2, s: f32, col: Color32) {
    painter.circle_filled(p(c, s, 0.0, -0.18), s * 0.2, col);
    let pts: Vec<Pos2> = (0..=12)
        .map(|i| {
            let a = std::f32::consts::PI + std::f32::consts::PI * i as f32 / 12.0;
            p(c, s, 0.42 * a.cos(), 0.62 + 0.38 * a.sin())
        })
        .collect();
    painter.add(Shape::convex_polygon(pts, col, Stroke::NONE));
}
