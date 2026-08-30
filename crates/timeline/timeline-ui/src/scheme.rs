//! Theme-aware palette access for the custom-drawn timeline. The drawing code
//! otherwise hardcodes the dark palette variants.

use shrimply_math_color::Color;

fn dark_scheme() -> bool {
    adw::StyleManager::default().is_dark()
}

pub(crate) fn view_bg() -> Color {
    if dark_scheme() {
        Color::VIEW_BG_DARK
    } else {
        Color::VIEW_BG_LIGHT
    }
}

pub(crate) fn view_fg() -> Color {
    if dark_scheme() {
        Color::VIEW_FG_DARK
    } else {
        Color::VIEW_FG_LIGHT
    }
}

pub(crate) fn sidebar_border() -> Color {
    if dark_scheme() {
        Color::SIDEBAR_BORDER_DARK
    } else {
        Color::SIDEBAR_BORDER_LIGHT
    }
}

pub(crate) fn sidebar_shade() -> Color {
    if dark_scheme() {
        Color::SIDEBAR_SHADE_DARK
    } else {
        Color::SIDEBAR_SHADE_LIGHT
    }
}
