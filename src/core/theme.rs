use iced::{color, Color};

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Theme {
    #[default]
    Lupin,
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy)]
pub struct BaseColors {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct NormalColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub base: BaseColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
}

impl Theme {
    pub const ALL: [Self; 3] = [Self::Lupin, Self::Dark, Self::Light];
    pub fn palette(self) -> ColorPalette {
        match self {
            Self::Dark => ColorPalette {
                base: BaseColors {
                    background: color!(0x111111),
                    foreground: color!(0x1C1C1C),
                },
                normal: NormalColors {
                    primary: color!(0x5E4266),
                    secondary: color!(0x386e50),
                    surface: color!(0x828282),
                    error: color!(0x992B2B),
                },
                bright: BrightColors {
                    primary: color!(0xBA84FC),
                    secondary: color!(0x49eb7a),
                    surface: color!(0xE0E0E0),
                    error: color!(0xC13047),
                },
            },
            Self::Light => ColorPalette {
                base: BaseColors {
                    background: color!(0xEEEEEE),
                    foreground: color!(0xE0E0E0),
                },
                normal: NormalColors {
                    primary: color!(0x230F08),
                    secondary: color!(0xF9D659),
                    surface: color!(0x818181),
                    error: color!(0x992B2B),
                },
                bright: BrightColors {
                    primary: color!(0x673AB7),
                    secondary: color!(0x3797A4),
                    surface: color!(0x000000),
                    error: color!(0xC13047),
                },
            },
            Self::Lupin => ColorPalette {
                base: BaseColors {
                    background: color!(0x282a36),
                    foreground: color!(0x353746),
                },
                normal: NormalColors {
                    primary: color!(0x58406F),
                    secondary: color!(0x386e50),
                    surface: color!(0xa2a4a3),
                    error: color!(0xA13034),
                },
                bright: BrightColors {
                    primary: color!(0xbd94f9),
                    secondary: color!(0x49eb7a),
                    surface: color!(0xf4f8f3),
                    error: color!(0xE63E6D),
                },
            },
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dark => "Dark",
                Self::Light => "Light",
                Self::Lupin => "Lupin",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_all() {
        assert_eq!(Theme::ALL.len(), 3);
        assert!(Theme::ALL.contains(&Theme::Dark));
        assert!(Theme::ALL.contains(&Theme::Light));
        assert!(Theme::ALL.contains(&Theme::Lupin));
    }

    #[test]
    fn test_theme_display() {
        assert_eq!(Theme::Dark.to_string(), "Dark");
        assert_eq!(Theme::Light.to_string(), "Light");
        assert_eq!(Theme::Lupin.to_string(), "Lupin");
    }

    #[test]
    fn test_theme_palette_returns_valid_colors() {
        for theme in Theme::ALL {
            let palette = theme.palette();
            // Verify all color fields have valid alpha values (0.0 - 1.0)
            assert!((0.0..=1.0).contains(&palette.base.background.a));
            assert!((0.0..=1.0).contains(&palette.base.foreground.a));
            assert!((0.0..=1.0).contains(&palette.normal.primary.a));
            assert!((0.0..=1.0).contains(&palette.bright.primary.a));
        }
    }

    #[test]
    fn test_theme_default_is_lupin() {
        assert_eq!(Theme::default(), Theme::Lupin);
    }

    #[test]
    fn test_theme_copy_and_clone() {
        let theme = Theme::Dark;
        let cloned = theme;
        assert_eq!(theme, cloned);
    }
}
