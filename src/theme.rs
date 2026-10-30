use eframe::egui::{
    self, Color32, ColorImage, Context, FontData, FontDefinitions, FontFamily, FontId, TextStyle,
    TextureHandle, TextureOptions,
};

pub const BG_MAIN: Color32 = Color32::from_rgb(17, 17, 18);
pub const BG_SIDEBAR: Color32 = Color32::from_rgb(28, 28, 30);
pub const CARD: Color32 = Color32::from_rgb(38, 38, 41);
pub const CARD_SELECTED: Color32 = Color32::from_rgb(40, 40, 43);
pub const TEXT: Color32 = Color32::from_rgb(242, 242, 244);
pub const TEXT_DIM: Color32 = Color32::from_rgb(138, 138, 144);
pub const STROKE: Color32 = Color32::from_rgb(70, 70, 75);
pub const TEAL: Color32 = Color32::from_rgb(0, 205, 215);
pub const BLUE: Color32 = Color32::from_rgb(0, 140, 255);
pub const RED: Color32 = Color32::from_rgb(255, 92, 92);
pub const YELLOW: Color32 = Color32::from_rgb(255, 196, 40);

pub fn regular(size: f32) -> FontId {
    FontId::proportional(size)
}

pub fn bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("bold".into()))
}

/// Uses Segoe UI from the system on Windows (zero bytes bundled, so the exe stays tiny).
/// Falls back to egui's built-in font everywhere else.
pub fn install_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    let fallback = fonts
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();

    let mut proportional = fallback.clone();
    let mut bold_family = fallback;

    if let Ok(bytes) = std::fs::read("C:\\Windows\\Fonts\\segoeui.ttf") {
        fonts
            .font_data
            .insert("ui-regular".to_owned(), FontData::from_owned(bytes));
        proportional.insert(0, "ui-regular".to_owned());
        bold_family.insert(0, "ui-regular".to_owned());
    }
    if let Ok(bytes) = std::fs::read("C:\\Windows\\Fonts\\segoeuib.ttf") {
        fonts
            .font_data
            .insert("ui-bold".to_owned(), FontData::from_owned(bytes));
        bold_family.insert(0, "ui-bold".to_owned());
    }

    fonts.families.insert(FontFamily::Proportional, proportional);
    fonts
        .families
        .insert(FontFamily::Name("bold".into()), bold_family);
    ctx.set_fonts(fonts);
}

pub fn apply(ctx: &Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = BG_MAIN;
    visuals.window_fill = BG_SIDEBAR;
    visuals.override_text_color = Some(TEXT);
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
    ctx.set_visuals(visuals);

    ctx.style_mut(|s| {
        s.text_styles.insert(TextStyle::Body, regular(15.0));
        s.text_styles.insert(TextStyle::Button, regular(15.0));
    });
}

pub struct Blob {
    pub cx: f32,
    pub cy: f32,
    pub rx: f32,
    pub ry: f32,
    pub color: [u8; 3],
    pub strength: f32,
}

/// Builds a small soft-gradient texture (a few KB of memory) that is stretched
/// over banners and cards. Much cheaper than shaders or large image assets.
pub fn gradient_texture(
    ctx: &Context,
    name: &str,
    w: usize,
    h: usize,
    base: [u8; 3],
    blobs: &[Blob],
) -> TextureHandle {
    let mut pixels = Vec::with_capacity(w * h);
    for y in 0..h {
        for x in 0..w {
            let fx = x as f32 / (w - 1) as f32;
            let fy = y as f32 / (h - 1) as f32;
            let mut c = [base[0] as f32, base[1] as f32, base[2] as f32];
            for b in blobs {
                let dx = (fx - b.cx) / b.rx;
                let dy = (fy - b.cy) / b.ry;
                let a = (b.strength * (-(dx * dx + dy * dy) * 2.0).exp()).clamp(0.0, 1.0);
                for i in 0..3 {
                    c[i] = c[i] * (1.0 - a) + b.color[i] as f32 * a;
                }
            }
            pixels.push(Color32::from_rgb(c[0] as u8, c[1] as u8, c[2] as u8));
        }
    }
    ctx.load_texture(
        name,
        ColorImage {
            size: [w, h],
            pixels,
        },
        TextureOptions::LINEAR,
    )
}
