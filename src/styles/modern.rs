use iced::{
    application, color,
    widget::{button, container, rule, scrollable, text, text_input, toggler},
    Background, Color,
};
use iced_native::Vector;
pub mod modern_widget {
    use super::ModernTheme;

    pub type Renderer = iced::Renderer<ModernTheme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
    pub type Text<'a> = iced::widget::Text<'a, Renderer>;
    pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;
    pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;
    pub type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, Renderer>;
}

/// [`RGBAColor`] is a type for human readable conversion
///
/// Example:
///
/// RGBAColor = `(255.0, 255.0, 255.0, 100.0)` -> Color = `(1.0, 1.0, 1.0, 1.0)`
pub type RGBAColor = (f32, f32, f32, f32);

/// [`RGBColor`] is a type for human readable conversion
///
/// Example:
///
/// RGBColor = `(255.0, 255.0, 255.0)` -> Color = `(1.0, 1.0, 1.0)`
pub type RGBColor = (f32, f32, f32);

pub trait PaletteConversor {
    /// Converts a color from `RGBA` format (as four separate floating-point values) to a `Color` object,
    /// using the `PaletteConversor` trait. This function takes in four `f32` values, representing the
    /// red, green, blue, and alpha (opacity) components of the color. The values should be in the range
    /// of 0 to 255, where 0 represents minimum intensity and 255 represents maximum intensity.
    ///
    /// # Arguments
    ///
    /// * `r` - The red component of the color, as an `f32` value between `0` and `255`.
    /// * `g` - The green component of the color, as an `f32` value between `0` and `255`.
    /// * `b` - The blue component of the color, as an `f32` value between `0` and `255`.
    /// * `a` - The alpha (opacity) component of the color, as an `f32` value between `0` and `100`.
    ///
    /// # Returns
    ///
    /// A new `Color` object, created from the given RGBA components. The values of `r`, `g`, `b`, and `a`
    /// are divided by 255.0 and 100.0, respectively, to convert them to the proper range for `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use search::styles::modern::PaletteConversor;
    /// use iced::Color;
    ///
    /// struct Palette {};
    ///
    /// impl PaletteConversor for Palette {};
    ///
    /// let color = Palette::from_rgba(127.0, 0.0, 255.0, 50.0);
    /// let should_be_equal = Color {
    /// r: 0.49803922,
    /// g: 0.0,
    /// b: 1.0,
    /// a: 0.5
    /// };
    ///
    /// assert_eq!(color, should_be_equal)
    /// ```
    fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color::from_rgba(r / 255.0, g / 255.0, b / 255.0, a / 100.0)
    }

    /// Converts a color from `RGB` format (as four separate floating-point values) to a `Color` object,
    /// using the `PaletteConversor` trait. This function takes in three `f32` values, representing the
    /// red, green, and blue components of the color. The values should be in the range
    /// of 0 to 255, where 0 represents minimum intensity and 255 represents maximum intensity.
    ///
    /// # Arguments
    ///
    /// * `r` - The red component of the color, as an `f32` value between `0` and `255`.
    /// * `g` - The green component of the color, as an `f32` value between `0` and `255`.
    /// * `b` - The blue component of the color, as an `f32` value between `0` and `255`.
    ///
    /// # Returns
    ///
    /// A new `Color` object, created from the given RGBA components. The values of `r`, `g`, `b`
    /// are divided by 255.0 and 100.0, respectively, to convert them to the proper range for `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use search::styles::modern::PaletteConversor;
    /// use iced::Color;
    ///
    /// struct Palette {};
    ///
    /// impl PaletteConversor for Palette {};
    ///
    /// let color = Palette::from_rgb(127.0, 0.0, 255.0);
    /// let should_be_equal = Color {
    /// r: 0.49803922,
    /// g: 0.0,
    /// b: 1.0,
    /// a: 1.0
    /// };
    ///
    /// assert_eq!(color, should_be_equal)
    /// ```
    fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color::from_rgba(r / 255.0, g / 255.0, b / 255.0, 1.0)
    }
}

trait Properties {
    const BORDER_RADIUS: f32 = 6.0;
    const BORDER_WIDTH: f32 = 0.0;
}

#[derive(Default, Clone, Copy)]
pub enum ModernTheme {
    #[default]
    Dark,
    Light,
}

impl Properties for ModernTheme {}

#[allow(dead_code)]
pub struct ButtonsPalette {
    text: RGBAColor,
    principal: RGBAColor,
    secondary: RGBAColor,
    tag: RGBAColor,
}

#[derive(Default)]
pub enum ModernButton {
    #[default]
    Principal,
    Secondary,
    Text,
    Tag((f32, f32, f32)),
}

impl PaletteConversor for ButtonsPalette {}

impl ButtonsPalette {
    pub fn label(&self) -> Color {
        let (r, g, b, a) = self.text;
        Self::from_rgba(r, g, b, a)
    }

    pub fn primary(&self) -> Color {
        let (r, g, b, a) = self.principal;
        Self::from_rgba(r, g, b, a)
    }

    pub fn secondary(&self) -> Color {
        let (r, g, b, a) = self.secondary;
        Self::from_rgba(r, g, b, a)
    }

    pub fn text(&self) -> Color {
        let (r, g, b, a) = self.text;
        Self::from_rgba(r, g, b, a)
    }

    pub fn tag(&self) -> Color {
        let (r, g, b, a) = self.tag;
        Self::from_rgba(r, g, b, a)
    }
}

#[allow(dead_code)]
pub struct InputPalette {
    background: RGBAColor,
    border_color: RGBAColor,
    icon_color: RGBAColor,
    placeholder_text: RGBAColor,
    text: RGBAColor,
    disabled_color: RGBAColor,
    disabled: RGBAColor,
}

impl PaletteConversor for InputPalette {}

impl InputPalette {
    pub fn background(&self) -> Color {
        let (r, g, b, a) = self.background;
        Self::from_rgba(r, g, b, a)
    }

    pub fn border_color(&self) -> Color {
        let (r, g, b, a) = self.border_color;
        Self::from_rgba(r, g, b, a)
    }

    pub fn icon_color(&self) -> Color {
        let (r, g, b, a) = self.icon_color;
        Self::from_rgba(r, g, b, a)
    }

    pub fn placeholder_text(&self) -> Color {
        let (r, g, b, a) = self.placeholder_text;
        Self::from_rgba(r, g, b, a)
    }

    pub fn text(&self) -> Color {
        let (r, g, b, a) = self.text;
        Self::from_rgba(r, g, b, a)
    }

    pub fn disabled_color(&self) -> Color {
        let (r, g, b, a) = self.disabled_color;
        Self::from_rgba(r, g, b, a)
    }

    pub fn disabled(&self) -> Color {
        let (r, g, b, a) = self.disabled;
        Self::from_rgba(r, g, b, a)
    }
}

#[allow(dead_code)]
pub struct ApplicationPalette {
    background: RGBAColor,
    text: RGBAColor,
}

impl PaletteConversor for ApplicationPalette {}

impl ApplicationPalette {
    pub fn background(&self) -> Color {
        let (r, g, b, a) = self.background;
        Self::from_rgba(r, g, b, a)
    }

    pub fn text(&self) -> Color {
        let (r, g, b, a) = self.text;
        Self::from_rgba(r, g, b, a)
    }
}

#[derive(Default)]
pub enum ModernContainer {
    #[default]
    Default,
    Historial,
    Input,
    Line,
}
#[derive(Default, Clone, Copy)]
pub enum ModernColor {
    #[default]
    Default,
    Custom(f32, f32, f32),
}

pub struct ContainerPalette {
    text: RGBAColor,
    border_radius: f32,
    border_width: f32,
    border_color: Option<RGBAColor>,
    background: Option<RGBAColor>,
}

impl PaletteConversor for ContainerPalette {}

impl ContainerPalette {
    pub fn text(&self) -> Color {
        let (r, g, b, a) = self.text;
        Self::from_rgba(r, g, b, a)
    }

    pub fn border_radius(&self) -> f32 {
        self.border_radius
    }

    pub fn border_width(&self) -> f32 {
        self.border_width
    }

    pub fn border_color(&self) -> Option<Color> {
        if let Some((r, g, b, a)) = self.border_color {
            return Some(Self::from_rgba(r, g, b, a));
        }
        None
    }

    pub fn background(&self) -> Option<Background> {
        if let Some((r, g, b, a)) = self.background {
            return Some(Self::from_rgba(r, g, b, a).into());
        }
        None
    }
}

pub struct TogglerPalette {
    background: RGBAColor,
    foreground: RGBAColor,
}

impl TogglerPalette {
    pub fn background(&self) -> Color {
        let (r, g, b, a) = self.background;
        Self::from_rgba(r, g, b, a)
    }

    pub fn foreground(&self) -> Color {
        let (r, g, b, a) = self.foreground;
        Self::from_rgba(r, g, b, a)
    }
}

impl PaletteConversor for TogglerPalette {}

pub struct ModernPalette {
    pub buttons: ButtonsPalette,
    pub inputs: InputPalette,
    pub container: ContainerPalette,
    pub toggler: TogglerPalette,
    pub app: ApplicationPalette,
}

impl PaletteConversor for ModernTheme {}

impl ModernPalette {
    const DARK: Self = Self {
        buttons: ButtonsPalette {
            text: (255.0, 255.0, 255.0, 100.0),
            principal: (253.0, 213.0, 193.0, 100.0),
            secondary: (82.0, 89.0, 96.0, 100.0),
            tag: (82.0, 89.0, 96.0, 100.0),
        },
        inputs: InputPalette {
            background: (39.0, 38.0, 47.0, 100.0),
            border_color: (60.0, 60.0, 60.0, 30.0),
            icon_color: (90.0, 90.0, 90.0, 100.0),
            placeholder_text: (100.0, 100.0, 100.0, 60.0),
            text: (233.0, 233.0, 233.0, 100.0),
            disabled_color: (60.0, 60.0, 60.0, 60.0),
            disabled: (60.0, 60.0, 60.0, 60.0),
        },
        container: ContainerPalette {
            text: (90.0, 90.0, 90.0, 100.0),
            border_radius: 6.0,
            border_width: 0.0,
            border_color: None,
            background: Some((60.0, 60.0, 60.0, 30.0)),
        },
        toggler: TogglerPalette {
            background: (33.0, 35.0, 37.0, 100.0),
            foreground: (60.0, 60.0, 60.0, 100.0),
        },
        app: ApplicationPalette {
            background: (31.0, 30.0, 37.0, 100.0),
            text: (250.0, 250.0, 242.0, 100.0),
        },
    };
    const LIGHT: Self = Self {
        buttons: ButtonsPalette {
            text: (255.0, 255.0, 255.0, 100.0),
            principal: (51.0, 88.0, 219.0, 100.0),
            secondary: (82.0, 89.0, 96.0, 100.0),
            tag: (51.0, 245.0, 106.0, 100.0),
        },
        inputs: InputPalette {
            background: (250.0, 250.0, 242.0, 100.0),
            border_color: (60.0, 60.0, 60.0, 30.0),
            icon_color: (90.0, 90.0, 90.0, 100.0),
            placeholder_text: (60.0, 60.0, 60.0, 60.0),
            text: (90.0, 90.0, 90.0, 100.0),
            disabled_color: (60.0, 60.0, 60.0, 60.0),
            disabled: (60.0, 60.0, 60.0, 60.0),
        },
        container: ContainerPalette {
            text: (90.0, 90.0, 90.0, 100.0),
            border_radius: 6.0,
            border_width: 0.0,
            border_color: None,
            background: Some((60.0, 60.0, 60.0, 30.0)),
        },
        toggler: TogglerPalette {
            background: (250.0, 250.0, 250.0, 100.0),
            foreground: (60.0, 60.0, 60.0, 30.0),
        },
        app: ApplicationPalette {
            text: (33.0, 35.0, 37.0, 100.0),
            background: (250.0, 250.0, 242.0, 100.0),
        },
    };
}

impl ModernTheme {
    fn palette(&self) -> ModernPalette {
        match self {
            ModernTheme::Dark => ModernPalette::DARK,
            ModernTheme::Light => ModernPalette::LIGHT,
        }
    }
}

impl application::StyleSheet for ModernTheme {
    type Style = ModernTheme;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette().app.background(),
            text_color: self.palette().app.text(),
        }
    }
}

impl text::StyleSheet for ModernTheme {
    type Style = ModernColor;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            ModernColor::Default => text::Appearance {
                ..Default::default()
            },
            ModernColor::Custom(r, g, b) => text::Appearance {
                color: Some(Self::from_rgba(r, g, b, 100.0)),
            },
        }
    }
}

impl button::StyleSheet for ModernTheme {
    type Style = ModernButton;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ModernButton::Principal => button::Appearance {
                background: self.palette().buttons.primary().into(),
                border_radius: 100.0,
                border_width: Self::BORDER_WIDTH,
                border_color: Color::TRANSPARENT,
                text_color: color!(255, 110, 1),
                ..Default::default()
            },
            ModernButton::Secondary => button::Appearance {
                background: self.palette().buttons.secondary().into(),
                border_radius: 100.0,
                border_width: Self::BORDER_WIDTH,
                border_color: Color::TRANSPARENT,
                text_color: self.palette().buttons.label(),
                ..Default::default()
            },
            ModernButton::Tag((r, g, b)) => button::Appearance {
                background: Self::from_rgb(*r, *g, *b).into(),
                border_radius: 100.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                text_color: self.palette().buttons.label(),
                ..Default::default()
            },
            ModernButton::Text => button::Appearance {
                background: Color::TRANSPARENT.into(),
                border_radius: 100.0,
                text_color: self.palette().buttons.label(),
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ModernButton::Secondary => self.active(&ModernButton::Principal),
            _ => self.active(style),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = match style {
            ModernButton::Secondary => self.active(&ModernButton::Principal),
            _ => self.active(style),
        };

        button::Appearance {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    r: color.r * 0.7,
                    g: color.g * 0.7,
                    b: color.b * 0.7,
                    ..color
                }),
            }),
            text_color: Color {
                r: active.text_color.r * 0.7,
                g: active.text_color.g * 0.7,
                b: active.text_color.b * 0.7,
                ..active.text_color
            },
            ..active
        }
    }
}

impl container::StyleSheet for ModernTheme {
    type Style = ModernContainer;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            ModernContainer::Default => container::Appearance::default(),
            ModernContainer::Input => container::Appearance {
                background: self.palette().inputs.background().into(),
                border_radius: 100.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                ..Default::default()
            },
            ModernContainer::Historial => container::Appearance {
                background: self.palette().inputs.background().into(),
                border_radius: 35.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                ..Default::default()
            },
            ModernContainer::Line => container::Appearance {
                background: self.palette().inputs.placeholder_text().into(),
                border_radius: 35.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                ..Default::default()
            },
        }
    }
}

impl toggler::StyleSheet for ModernTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style, _is_active: bool) -> toggler::Appearance {
        toggler::Appearance {
            background: self.palette().toggler.background(),
            background_border: None,
            foreground: self.palette().toggler.foreground(),
            foreground_border: None,
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_active: bool) -> toggler::Appearance {
        toggler::Appearance {
            background: self.palette().toggler.background(),
            background_border: None,
            foreground: self.palette().toggler.foreground(),
            foreground_border: None,
        }
    }
}

impl text_input::StyleSheet for ModernTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.palette().inputs.background().into(),
            border_radius: 100.0,
            border_width: 0.0,
            border_color: self.palette().inputs.border_color(),
            icon_color: self.palette().inputs.icon_color(),
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.palette().inputs.background().into(),
            border_radius: 100.0,
            border_width: 0.0,
            border_color: self.palette().inputs.border_color(),
            icon_color: self.palette().inputs.icon_color(),
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.palette().inputs.placeholder_text()
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.palette().inputs.text()
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        color!(70, 70, 70, 0.6)
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        self.palette().inputs.disabled_color()
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.palette().inputs.background().into(),
            border_radius: Self::BORDER_RADIUS,
            border_width: 2.0,
            border_color: self.palette().inputs.border_color(),
            icon_color: self.palette().inputs.icon_color(),
        }
    }
}

impl rule::StyleSheet for ModernTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: self.palette().inputs.placeholder_text(),
            width: 2,
            radius: 90.0,
            fill_mode: rule::FillMode::Percent(20.0),
        }
    }
}

impl scrollable::StyleSheet for ModernTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: self.palette().buttons.secondary().into(),
            border_radius: 90.0,
            border_width: 2.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: self.palette().inputs.placeholder_text(),
                border_radius: 90.0,
                border_width: 2.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: self.palette().inputs.placeholder_text().into(),
            border_radius: 90.0,
            border_width: 2.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: self.palette().buttons.primary(),
                border_radius: 90.0,
                border_width: 2.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}
