use eframe::egui::{self, Align2, Margin, Rounding, TextureHandle};
use std::time::{Duration, Instant};

use crate::theme::{self, Blob};
use crate::{home, sidebar};

pub struct Textures {
    pub banner: TextureHandle,
    pub join_pro: TextureHandle,
    pub promo_longer: TextureHandle,
    pub promo_sale: TextureHandle,
}

pub struct LiteCutApp {
    pub tex: Textures,
    /// Which bottom-left promo slide is visible (0 = "Make it longer?", 1 = "Sale is LIVE").
    pub promo_idx: usize,
    /// Saved projects (empty in Phase 1 -> shows the empty state).
    pub projects: Vec<String>,
    toast: Option<(String, Instant)>,
}

impl LiteCutApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;
        theme::install_fonts(ctx);
        theme::apply(ctx);

        let banner = theme::gradient_texture(
            ctx,
            "banner",
            384,
            64,
            [22, 22, 34],
            &[
                Blob { cx: 0.30, cy: 0.35, rx: 0.22, ry: 0.55, color: [140, 50, 170], strength: 0.85 },
                Blob { cx: 0.62, cy: 0.40, rx: 0.26, ry: 0.55, color: [30, 70, 200], strength: 0.85 },
                Blob { cx: 0.25, cy: 1.00, rx: 0.30, ry: 0.55, color: [30, 190, 205], strength: 0.95 },
                Blob { cx: 0.62, cy: 1.05, rx: 0.30, ry: 0.55, color: [40, 110, 225], strength: 0.95 },
                Blob { cx: 0.99, cy: 1.00, rx: 0.14, ry: 0.60, color: [70, 60, 220], strength: 0.95 },
            ],
        );
        let join_pro = theme::gradient_texture(
            ctx,
            "join_pro",
            192,
            32,
            [214, 198, 244],
            &[
                Blob { cx: 0.12, cy: 0.5, rx: 0.30, ry: 1.0, color: [255, 192, 212], strength: 0.95 },
                Blob { cx: 0.95, cy: 0.5, rx: 0.30, ry: 1.0, color: [190, 242, 230], strength: 0.95 },
            ],
        );
        let promo_sale = theme::gradient_texture(
            ctx,
            "promo_sale",
            192,
            110,
            [18, 14, 36],
            &[
                Blob { cx: 0.85, cy: 0.30, rx: 0.40, ry: 0.50, color: [120, 60, 220], strength: 0.95 },
                Blob { cx: 0.78, cy: 0.85, rx: 0.20, ry: 0.30, color: [210, 80, 160], strength: 0.60 },
            ],
        );
        let promo_longer = theme::gradient_texture(
            ctx,
            "promo_longer",
            192,
            110,
            [14, 22, 38],
            &[
                Blob { cx: 0.80, cy: 0.40, rx: 0.40, ry: 0.50, color: [0, 170, 190], strength: 0.85 },
                Blob { cx: 0.30, cy: 0.95, rx: 0.35, ry: 0.40, color: [40, 80, 220], strength: 0.75 },
            ],
        );

        Self {
            tex: Textures { banner, join_pro, promo_longer, promo_sale },
            promo_idx: 1,
            projects: Vec::new(),
            toast: None,
        }
    }

    pub fn show_toast(&mut self, msg: &str) {
        self.toast = Some((msg.to_owned(), Instant::now()));
    }

    fn draw_toast(&mut self, ctx: &egui::Context) {
        let Some((msg, started)) = self.toast.clone() else { return };
        if started.elapsed() > Duration::from_millis(2500) {
            self.toast = None;
            return;
        }
        egui::Area::new(egui::Id::new("toast"))
            .anchor(Align2::CENTER_BOTTOM, [0.0, -40.0])
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(theme::CARD)
                    .rounding(Rounding::same(10.0))
                    .inner_margin(Margin::symmetric(18.0, 10.0))
                    .show(ui, |ui| {
                        ui.label(msg);
                    });
            });
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}

impl eframe::App for LiteCutApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        sidebar::show(ctx, self);
        home::show(ctx, self);
        self.draw_toast(ctx);
    }
}
