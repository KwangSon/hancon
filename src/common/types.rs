use serde::{Deserialize, Serialize};

/// HWP unit: 1/7200 inch (1 twip)
/// Used for margins, font sizes, positions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HwpUnit(pub i32);

impl HwpUnit {
    pub fn from_twips(twips: i32) -> Self {
        HwpUnit(twips)
    }

    pub fn to_mm(&self) -> f64 {
        (self.0 as f64) * 25.4 / 7200.0
    }

    pub fn to_pt(&self) -> f64 {
        (self.0 as f64) / 20.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color(pub u32); // 0xBBGGRR format

impl Color {
    pub fn from_bgr(b: u8, g: u8, r: u8) -> Self {
        Color(((b as u32) << 16) | ((g as u32) << 8) | (r as u32))
    }

    pub fn to_hex(&self) -> String {
        format!("#{:06X}", self.0)
    }

    pub fn r(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    pub fn g(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    pub fn b(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Margin {
    pub left: HwpUnit,
    pub top: HwpUnit,
    pub right: HwpUnit,
    pub bottom: HwpUnit,
}

impl Margin {
    pub fn new(left: HwpUnit, top: HwpUnit, right: HwpUnit, bottom: HwpUnit) -> Self {
        Margin {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn zero() -> Self {
        Margin {
            left: HwpUnit(0),
            top: HwpUnit(0),
            right: HwpUnit(0),
            bottom: HwpUnit(0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: HwpUnit,
    pub y: HwpUnit,
}

impl Position {
    pub fn new(x: HwpUnit, y: HwpUnit) -> Self {
        Position { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Size {
    pub width: HwpUnit,
    pub height: HwpUnit,
}

impl Size {
    pub fn new(width: HwpUnit, height: HwpUnit) -> Self {
        Size { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rect {
    pub x0: HwpUnit,
    pub y0: HwpUnit,
    pub x1: HwpUnit,
    pub y1: HwpUnit,
}

impl Rect {
    pub fn new(x0: HwpUnit, y0: HwpUnit, x1: HwpUnit, y1: HwpUnit) -> Self {
        Rect { x0, y0, x1, y1 }
    }

    pub fn width(&self) -> HwpUnit {
        HwpUnit(self.x1.0 - self.x0.0)
    }

    pub fn height(&self) -> HwpUnit {
        HwpUnit(self.y1.0 - self.y0.0)
    }
}

/// Alignment enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Alignment {
    Left = 0,
    Right = 1,
    Center = 2,
    Justify = 3,
    Distribute = 4,
}

impl Alignment {
    pub fn from_u16(val: u16) -> Option<Self> {
        match val {
            0 => Some(Alignment::Left),
            1 => Some(Alignment::Right),
            2 => Some(Alignment::Center),
            3 => Some(Alignment::Justify),
            4 => Some(Alignment::Distribute),
            _ => None,
        }
    }

    pub fn to_odt_str(&self) -> &'static str {
        match self {
            Alignment::Left => "left",
            Alignment::Right => "right",
            Alignment::Center => "center",
            Alignment::Justify => "justify",
            Alignment::Distribute => "distribute",
        }
    }
}

/// Vertical alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VAlignment {
    Top = 0,
    Center = 1,
    Bottom = 2,
}

impl VAlignment {
    pub fn from_u16(val: u16) -> Option<Self> {
        match val {
            0 => Some(VAlignment::Top),
            1 => Some(VAlignment::Center),
            2 => Some(VAlignment::Bottom),
            _ => None,
        }
    }

    pub fn to_odt_str(&self) -> &'static str {
        match self {
            VAlignment::Top => "top",
            VAlignment::Center => "middle",
            VAlignment::Bottom => "bottom",
        }
    }
}

/// Line style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LineStyle {
    None = 0,
    Solid = 1,
    Dotted = 2,
    Dashed = 3,
    DashDot = 4,
    DashDotDot = 5,
    Double = 6,
    Wave = 7,
}

impl LineStyle {
    pub fn from_u16(val: u16) -> Option<Self> {
        match val {
            0 => Some(LineStyle::None),
            1 => Some(LineStyle::Solid),
            2 => Some(LineStyle::Dotted),
            3 => Some(LineStyle::Dashed),
            4 => Some(LineStyle::DashDot),
            5 => Some(LineStyle::DashDotDot),
            6 => Some(LineStyle::Double),
            7 => Some(LineStyle::Wave),
            _ => None,
        }
    }

    pub fn to_odt_str(&self) -> &'static str {
        match self {
            LineStyle::None => "none",
            LineStyle::Solid => "solid",
            LineStyle::Dotted => "dotted",
            LineStyle::Dashed => "dashed",
            LineStyle::DashDot => "dash-dot",
            LineStyle::DashDotDot => "dash-dot-dot",
            LineStyle::Double => "double",
            LineStyle::Wave => "wave",
        }
    }
}
