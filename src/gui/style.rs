use iced::{button, Background, Color};

/// https://github.com/iced-rs/iced/pull/1227
/// # Examples
///
/// ```
/// # use iced_core::{Color, color};
/// assert_eq!(color!(0, 0, 0), Color::from_rgb(0., 0., 0.));
/// assert_eq!(color!(0, 0, 0, 0.), Color::from_rgba(0., 0., 0., 0.));
/// assert_eq!(color!(0xffffff), Color::from_rgb(1., 1., 1.));
/// assert_eq!(color!(0xffffff, 0.), Color::from_rgba(1., 1., 1., 0.));
/// ```
#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color::from_rgb8($r, $g, $b)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::from_rgba8($r, $g, $b, $a)
    };
    ($hex:expr) => {{
        let hex = $hex as u32;
        let r = (hex & 0xff0000) >> 16;
        let g = (hex & 0xff00) >> 8;
        let b = (hex & 0xff);
        Color::from_rgb8(r as u8, g as u8, b as u8)
    }};
    ($hex:expr, $a:expr) => {{
        let hex = $hex as u32;
        let r = (hex & 0xff0000) >> 16;
        let g = (hex & 0xff00) >> 8;
        let b = (hex & 0xff);
        Color::from_rgba8(r as u8, g as u8, b as u8, $a)
    }};
}

pub struct PrimaryButtonStyle;
impl button::StyleSheet for PrimaryButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Default::default(),
            border_color: Color::TRANSPARENT,
            border_width: 0.,
            border_radius: 5.,
            text_color: color!(0xffffff),
            background: Some(Background::Color(color!(0x0096E6))),
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(color!(0x44A4D8))),
            ..self.active()
        }
    }
}

pub struct SecondaryButtonStyle;
impl button::StyleSheet for SecondaryButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Default::default(),
            border_color: color!(0x6B6B6B),
            border_width: 1.,
            border_radius: 5.,
            text_color: color!(0x6B6B6B),
            background: Some(Background::Color(Color::WHITE)),
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(color!(0xF3F3F3))),
            ..self.active()
        }
    }
}
