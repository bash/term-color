/// An RGB color with 16 bits per channel.
/// You can use [`Color::scale_to_8bit`] to convert to an 8bit RGB color.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
#[allow(clippy::exhaustive_structs)]
pub struct Color {
    /// Red
    pub red: u16,
    /// Green
    pub green: u16,
    /// Blue
    pub blue: u16,
    /// Can almost always be ignored as it is rarely set to
    /// something other than the default (`0xffff`).
    pub alpha: u16,
}

impl Color {
    /// Perceptual lightness (L*) as a value between 0.0 (black) and 1.0 (white)
    /// where 0.5 is the perceptual middle gray.
    ///
    /// Note that the color's alpha is ignored.
    /// ```
    /// # use terminal_colorsaurus::Color;
    /// # let color = Color::default();
    /// let is_dark = color.perceived_lightness() <= 0.5;
    /// ```
    pub fn perceived_lightness(&self) -> f32 {
        let color = xterm_color::Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha,
        };
        color.perceived_lightness()
    }

    /// Converts the color to 8 bit precision per channel by scaling each channel.
    ///
    /// ```
    /// # use terminal_colorsaurus::Color;
    /// let white = Color { red: u16::MAX, green: u16::MAX, blue: u16::MAX, alpha: u16::MAX };
    /// assert_eq!((u8::MAX, u8::MAX, u8::MAX, u8::MAX), white.scale_to_8bit());
    ///
    /// let black = Color { red: 0, green: 0, blue: 0, alpha: u16::MAX };
    /// assert_eq!((0, 0, 0, u8::MAX), black.scale_to_8bit());
    /// ```
    pub fn scale_to_8bit(&self) -> (u8, u8, u8, u8) {
        (
            scale_to_u8(self.red),
            scale_to_u8(self.green),
            scale_to_u8(self.blue),
            scale_to_u8(self.alpha),
        )
    }
}

#[cfg(test)]
impl Color {
    pub(crate) const fn rgb(red: u16, green: u16, blue: u16) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: u16::MAX,
        }
    }
}

fn scale_to_u8(channel: u16) -> u8 {
    (channel as u32 * (u8::MAX as u32) / (u16::MAX as u32)) as u8
}

#[cfg(feature = "rgb")]
impl From<Color> for rgb::RGB16 {
    fn from(value: Color) -> Self {
        rgb::RGB16 {
            r: value.red,
            g: value.green,
            b: value.blue,
        }
    }
}

#[cfg(feature = "rgb")]
impl From<Color> for rgb::RGBA16 {
    fn from(value: Color) -> Self {
        rgb::RGBA16 {
            r: value.red,
            g: value.green,
            b: value.blue,
            a: value.alpha,
        }
    }
}

#[cfg(feature = "rgb")]
impl From<Color> for rgb::RGB8 {
    fn from(value: Color) -> Self {
        let (r, g, b, _) = value.scale_to_8bit();
        rgb::RGB8 { r, g, b }
    }
}

#[cfg(feature = "rgb")]
impl From<Color> for rgb::RGBA8 {
    fn from(value: Color) -> Self {
        let (r, g, b, a) = value.scale_to_8bit();
        rgb::RGBA8 { r, g, b, a }
    }
}

#[cfg(feature = "rgb")]
impl From<rgb::RGB16> for Color {
    fn from(value: rgb::RGB16) -> Self {
        Color {
            red: value.r,
            green: value.g,
            blue: value.b,
            alpha: u16::MAX,
        }
    }
}

#[cfg(feature = "anstyle")]
impl From<Color> for anstyle::RgbColor {
    fn from(value: Color) -> Self {
        let (r, g, b, _) = value.scale_to_8bit();
        anstyle::RgbColor(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_has_perceived_lightness_zero() {
        let black = Color::default();
        assert_eq!(0.0, black.perceived_lightness())
    }

    #[test]
    fn white_has_perceived_lightness_100() {
        let white = Color {
            red: u16::MAX,
            green: u16::MAX,
            blue: u16::MAX,
            alpha: u16::MAX,
        };
        assert_eq!(1.0, white.perceived_lightness())
    }
}
