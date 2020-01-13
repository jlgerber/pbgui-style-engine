use hsluv::{hex_to_rgb, rgb_to_hex};
use serde::ser::{Serialize as SerializeTrait, Serializer};
use std::fmt;

/// Color represnetations in qss
#[derive(Debug, PartialEq)]
pub enum Color {
    Rgb { r: u8, g: u8, b: u8 },
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    Hex(String),
    Hsl(u8, f32, f32),
    Hsla(u8, f32, f32, f32),
    Red,
    Green,
    Blue,
    Grey,
    LightGrey,
    DarkGrey,
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    ConrflowerBlue,
    Cornsilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGlodenRod,
    DarkGray,
    DarkGreen,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DimGrey,
    DodgerBlue,
    FireBrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    GoldenRod,
    GreenYellow,
    HoneyDew,
    HotPink,
    IndiaRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenRodYellow,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSlateGrey,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquaMarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenRod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    RebeccaPurple,
    Rosybrown,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    SlateGrey,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,
}

impl SerializeTrait for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            _ => serializer.serialize_str(&*self.to_string().as_str()),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rgb { r, g, b } => write!(f, "rgb({}, {}, {})", r, g, b),
            Self::Rgba { r, g, b, a } => write!(f, "rgba({}, {}, {}, {})", r, g, b, a),
            Self::Hex(ref val) => {
                // convert back and forth to keep it valid
                let color = hex_to_rgb(val);
                write!(f, "{}", rgb_to_hex((color.0, color.1, color.2)))
            }
            Self::Hsl(h, s, l) => {
                let sat = if s < &0.0 {
                    0.0
                } else if s > &100.0 {
                    100.0
                } else {
                    *s
                };
                let light = if l < &0.0 {
                    0.0
                } else if l > &100.0 {
                    100.0
                } else {
                    *l
                };
                write!(f, "hsl({}, {}%, {}%)", h, sat, light)
            }
            Self::Hsla(h, s, l, a) => {
                let sat = if s < &0.0 {
                    0.0
                } else if s > &100.0 {
                    100.0
                } else {
                    *s
                };
                let light = if l < &0.0 {
                    0.0
                } else if l > &100.0 {
                    100.0
                } else {
                    *l
                };
                let alpha = if a < &0.0 {
                    0.0
                } else if a > &1.0 {
                    1.0
                } else {
                    *a
                };
                write!(f, "hsla({}, {}%, {}%, {})", h, sat, light, alpha)
            }
            Self::Red => write!(f, "red"),
            Self::Green => write!(f, "green"),
            Self::Blue => write!(f, "blue"),
            Self::Grey => write!(f, "grey"),
            Self::LightGrey => write!(f, "lightgrey"),
            Self::DarkGrey => write!(f, "darkgrey"),
            Self::AliceBlue => write!(f, "aliceblue"),
            Self::AntiqueWhite => write!(f, "antiquewhite"),
            Self::Aqua => write!(f, "aqua"),
            Self::Aquamarine => write!(f, "aquamarine"),
            Self::Azure => write!(f, "azure"),
            Self::Beige => write!(f, "beige"),
            Self::Bisque => write!(f, "bisque"),
            Self::Black => write!(f, "black"),
            Self::BlanchedAlmond => write!(f, "blanchedalmond"),
            Self::BlueViolet => write!(f, "blueviolet"),
            Self::Brown => write!(f, "brown"),
            Self::BurlyWood => write!(f, "burlywood"),
            Self::CadetBlue => write!(f, "cadetblue"),
            Self::Chartreuse => write!(f, "chartreuse"),
            Self::Chocolate => write!(f, "chocolate"),
            Self::Coral => write!(f, "coral"),
            Self::ConrflowerBlue => write!(f, "cornflowerblue"),
            Self::Cornsilk => write!(f, "cornsilk"),
            Self::Crimson => write!(f, "crimson"),
            Self::Cyan => write!(f, "cyan"),
            Self::DarkBlue => write!(f, "darkblue"),
            Self::DarkCyan => write!(f, "darkcyan"),
            Self::DarkGlodenRod => write!(f, "darkgoldenrod"),
            Self::DarkGray => write!(f, "darkgray"),
            Self::DarkGreen => write!(f, "darkgreen"),
            Self::DarkOrchid => write!(f, "darkorchid"),
            Self::DarkRed => write!(f, "darkred"),
            Self::DarkSalmon => write!(f, "darksalmon"),
            Self::DarkSeaGreen => write!(f, "darkseagreen"),
            Self::DarkSlateBlue => write!(f, "darkslateblue"),
            Self::DarkSlateGray => write!(f, "darkslategray"),
            Self::DarkSlateGrey => write!(f, "darkslategrey"),
            Self::DarkTurquoise => write!(f, "darkturquoise"),
            Self::DarkViolet => write!(f, "darkviolet"),
            Self::DeepPink => write!(f, "deeppink"),
            Self::DeepSkyBlue => write!(f, "deepskyblue"),
            Self::DimGray => write!(f, "dimgray"),
            Self::DimGrey => write!(f, "dimgrey"),
            Self::DodgerBlue => write!(f, "dodgerblue"),
            Self::FireBrick => write!(f, "firebrick"),
            Self::FloralWhite => write!(f, "floralwhite"),
            Self::ForestGreen => write!(f, "forestgreen"),
            Self::Fuchsia => write!(f, "fuchsia"),
            Self::Gainsboro => write!(f, "gainsboro"),
            Self::GhostWhite => write!(f, "ghostwhite"),
            Self::Gold => write!(f, "gold"),
            Self::GoldenRod => write!(f, "goldenrod"),
            Self::GreenYellow => write!(f, "greenyellow"),
            Self::HoneyDew => write!(f, "honeydew"),
            Self::HotPink => write!(f, "hotpink"),
            Self::IndiaRed => write!(f, "indiared"),
            Self::Indigo => write!(f, "indigo"),
            Self::Ivory => write!(f, "ivory"),
            Self::Khaki => write!(f, "khaki"),
            Self::Lavender => write!(f, "lavender"),
            Self::LavenderBlush => write!(f, "lavenderblush"),
            Self::LawnGreen => write!(f, "lawngreen"),
            Self::LemonChiffon => write!(f, "lemonchiffon"),
            Self::LightBlue => write!(f, "lightblue"),
            Self::LightCoral => write!(f, "lightcoral"),
            Self::LightCyan => write!(f, "lightcyan"),
            Self::LightGoldenRodYellow => write!(f, "lightgoldenrodyellow"),
            Self::LightGreen => write!(f, "lightgreen"),
            Self::LightPink => write!(f, "lightpink"),
            Self::LightSalmon => write!(f, "lightsalmon"),
            Self::LightSeaGreen => write!(f, "lightseagreen"),
            Self::LightSkyBlue => write!(f, "lightskyblue"),
            Self::LightSlateGray => write!(f, "lightslategray"),
            Self::LightSlateGrey => write!(f, "lightslategrey"),
            Self::LightSteelBlue => write!(f, "lightsteelblue"),
            Self::LightYellow => write!(f, "lightyellow"),
            Self::Lime => write!(f, "lime"),
            Self::LimeGreen => write!(f, "limegreen"),
            Self::Linen => write!(f, "linen"),
            Self::Magenta => write!(f, "magenta"),
            Self::Maroon => write!(f, "maroon"),
            Self::MediumAquaMarine => write!(f, "mediumaquamarine"),
            Self::MediumBlue => write!(f, "mediumblue"),
            Self::MediumOrchid => write!(f, "medumorchid"),
            Self::MediumPurple => write!(f, "mediumpurple"),
            Self::MediumSeaGreen => write!(f, "mediumseagreen"),
            Self::MediumSlateBlue => write!(f, "mediumslateblue"),
            Self::MediumSpringGreen => write!(f, "mediumspringgreen"),
            Self::MediumTurquoise => write!(f, "mediumturquoise"),
            Self::MediumVioletRed => write!(f, "mediumvioletred"),
            Self::MidnightBlue => write!(f, "mednightblue"),
            Self::MintCream => write!(f, "mintcream"),
            Self::MistyRose => write!(f, "mistyrose"),
            Self::Moccasin => write!(f, "moccasin"),
            Self::NavajoWhite => write!(f, "navajowhite"),
            Self::Navy => write!(f, "navy"),
            Self::OldLace => write!(f, "oldlace"),
            Self::Olive => write!(f, "olive"),
            Self::OliveDrab => write!(f, "olivedrab"),
            Self::Orange => write!(f, "orange"),
            Self::OrangeRed => write!(f, "orangered"),
            Self::Orchid => write!(f, "orchid"),
            Self::PaleGoldenRod => write!(f, "palegoldenrod"),
            Self::PaleGreen => write!(f, "palegreen"),
            Self::PaleTurquoise => write!(f, "paleturquoise"),
            Self::PaleVioletRed => write!(f, "palevioletred"),
            Self::PapayaWhip => write!(f, "papayawhip"),
            Self::PeachPuff => write!(f, "peachpuff"),
            Self::Peru => write!(f, "peru"),
            Self::Pink => write!(f, "pink"),
            Self::Plum => write!(f, "plum"),
            Self::PowderBlue => write!(f, "powderblue"),
            Self::Purple => write!(f, "purple"),
            Self::RebeccaPurple => write!(f, "rebeccapurple"),
            Self::Rosybrown => write!(f, "rosybrown"),
            Self::SaddleBrown => write!(f, "saddlebrown"),
            Self::Salmon => write!(f, "salmon"),
            Self::SandyBrown => write!(f, "sandybrown"),
            Self::SeaGreen => write!(f, "seagreen"),
            Self::SeaShell => write!(f, "seashell"),
            Self::Sienna => write!(f, "sienna"),
            Self::Silver => write!(f, "silver"),
            Self::SkyBlue => write!(f, "skyblue"),
            Self::SlateBlue => write!(f, "slateblue"),
            Self::SlateGray => write!(f, "slategray"),
            Self::SlateGrey => write!(f, "slategrey"),
            Self::Snow => write!(f, "snow"),
            Self::SpringGreen => write!(f, "springgreen"),
            Self::SteelBlue => write!(f, "steelblue"),
            Self::Tan => write!(f, "tan"),
            Self::Teal => write!(f, "teal"),
            Self::Thistle => write!(f, "thistle"),
            Self::Tomato => write!(f, "tomato"),
            Self::Turquoise => write!(f, "turquoise"),
            Self::Violet => write!(f, "violet"),
            Self::Wheat => write!(f, "wheat"),
            Self::White => write!(f, "white"),
            Self::WhiteSmoke => write!(f, "whitesmoke"),
            Self::Yellow => write!(f, "yellow"),
            Self::YellowGreen => write!(f, "yellowgreen"),
        }
    }
}

/// Size Units in qss
/// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub enum Size {
    Px(u8),
    Em(u8),
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Px(val) => write!(f, "{}px", val),
            Self::Em(val) => write!(f, "{}em", val),
        }
    }
}

impl SerializeTrait for Size {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            _ => serializer.serialize_str(&*self.to_string().as_str()),
        }
    }
}

/// Size Units in qss
/// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub enum Size4 {
    Px(u8, u8, u8, u8),
}

impl fmt::Display for Size4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Px(a, b, c, d) => write!(f, "{}px {}px {}px {}px", a, b, c, d),
        }
    }
}

impl SerializeTrait for Size4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            _ => serializer.serialize_str(&*self.to_string().as_str()),
        }
    }
}
/// Border styles, from css. Presumably, these are all valid in qss
#[derive(Debug)]
pub enum Border {
    Dot { size: Size, color: Color },
    Dash { size: Size, color: Color },
    Solid { size: Size, color: Color },
    Double { size: Size, color: Color },
    Groove { size: Size, color: Color },
    Ridge { size: Size, color: Color },
    Inset { size: Size, color: Color },
    Outset { size: Size, color: Color },
    Mix { size: Size, color: Color },
    None,
    Hidden,
}

impl SerializeTrait for Border {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            _ => serializer.serialize_str(&*self.to_string().as_str()),
        }
    }
}

impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dot { size, color } => {
                write!(f, "{} dotted {}", size.to_string(), color.to_string())
            }
            Self::Dash { size, color } => {
                write!(f, "{} dashed {}", size.to_string(), color.to_string())
            }
            Self::Solid { size, color } => {
                write!(f, "{} solid {}", size.to_string(), color.to_string())
            }
            Self::Double { size, color } => {
                write!(f, "{} double {}", size.to_string(), color.to_string())
            }
            Self::Groove { size, color } => {
                write!(f, "{} groove {}", size.to_string(), color.to_string())
            }
            Self::Ridge { size, color } => {
                write!(f, "{} ridge {}", size.to_string(), color.to_string())
            }
            Self::Inset { size, color } => {
                write!(f, "{} inset {}", size.to_string(), color.to_string())
            }
            Self::Outset { size, color } => {
                write!(f, "{} outset {}", size.to_string(), color.to_string())
            }
            Self::Mix { size, color } => write!(
                f,
                "{} dotted dahsed solid double {}",
                size.to_string(),
                color.to_string()
            ),
            Self::None => write!(f, "none"),
            Self::Hidden => write!(f, "hidden"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_rgb_to_string() {
        let color = Color::Rgb {
            r: 255,
            g: 25,
            b: 200,
        }
        .to_string();
        assert_eq!(color.as_str(), "rgb(255, 25, 200)");
    }

    #[test]
    fn can_convert_rgba_to_string() {
        let color = Color::Rgba {
            r: 200,
            g: 201,
            b: 202,
            a: 250,
        }
        .to_string();
        assert_eq!(color.as_str(), "rgba(200, 201, 202, 250)");
    }

    #[test]
    fn can_convert_hex_string() {
        let color = Color::Hex("#FFFFFF".to_string()).to_string();
        assert_eq!(color.as_str(), "#ffffff");
    }
    #[test]
    fn can_convert_hsl_string() {
        let color = Color::Hsl(200, 75.0, 50.0).to_string();
        assert_eq!(color.as_str(), "hsl(200, 75%, 50%)");
    }
    #[test]
    fn can_convert_hsla_string() {
        let color = Color::Hsla(200, 75.0, 50.0, 0.5).to_string();
        assert_eq!(color.as_str(), "hsla(200, 75%, 50%, 0.5)");
    }
    #[test]
    fn can_convert_red_to_string() {
        let color = Color::Red.to_string();
        assert_eq!(color.as_str(), "red");
    }

    #[test]
    fn can_convert_green_to_string() {
        let color = Color::Green.to_string();
        assert_eq!(color.as_str(), "green");
    }

    #[test]
    fn can_convert_blue_to_string() {
        let color = Color::Blue.to_string();
        assert_eq!(color.as_str(), "blue");
    }

    #[test]
    fn can_convert_grey_to_string() {
        let color = Color::Grey.to_string();
        assert_eq!(color.as_str(), "grey");
    }

    #[test]
    fn can_convert_lightgrey_to_string() {
        let color = Color::LightGrey.to_string();
        assert_eq!(color.as_str(), "lightgrey");
    }

    #[test]
    fn can_convert_darkgrey_to_string() {
        let color = Color::DarkGrey.to_string();
        assert_eq!(color.as_str(), "darkgrey");
    }
    //
    // sz tests
    //
    #[test]
    fn can_convert_px_to_string() {
        assert_eq!(Size::Px(80).to_string().as_str(), "80px");
    }

    #[test]
    fn can_convert_em_to_string() {
        assert_eq!(Size::Em(80).to_string().as_str(), "80em");
    }

    //
    // border tests
    //
    #[test]
    fn can_convert_solid_border_to_string() {
        assert_eq!(
            Border::Solid {
                size: Size::Px(1),
                color: Color::Red
            }
            .to_string()
            .as_str(),
            "1px solid red"
        );
    }

    #[test]
    fn can_convert_no_border_to_string() {
        assert_eq!(Border::None.to_string().as_str(), "none");
    }
}
