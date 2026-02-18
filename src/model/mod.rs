use crate::common::{Alignment, Color, HwpUnit, LineStyle, Margin, Rect};
use serde::{Deserialize, Serialize};

/// Top-level HWP document model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub sections: Vec<Section>,
    pub fonts: Vec<String>,
    pub styles: Vec<Style>,
    pub char_shapes: Vec<CharShape>,
    pub para_shapes: Vec<ParaShape>,
    pub border_fills: Vec<BorderFill>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            sections: Vec::new(),
            fonts: Vec::new(),
            styles: Vec::new(),
            char_shapes: Vec::new(),
            para_shapes: Vec::new(),
            border_fills: Vec::new(),
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

/// Section (SectionDef in HWP)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub blocks: Vec<Block>,
    pub page_width: HwpUnit,
    pub page_height: HwpUnit,
    pub margin_top: HwpUnit,
    pub margin_bottom: HwpUnit,
    pub margin_left: HwpUnit,
    pub margin_right: HwpUnit,
}

impl Section {
    pub fn new() -> Self {
        Section {
            blocks: Vec::new(),
            page_width: HwpUnit(5644),  // 210mm default
            page_height: HwpUnit(7990), // 297mm default
            margin_top: HwpUnit(1440),
            margin_bottom: HwpUnit(1440),
            margin_left: HwpUnit(1440),
            margin_right: HwpUnit(1440),
        }
    }
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

/// Block-level content (Paragraph, Table, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Block {
    Paragraph(Paragraph),
    Table(Table),
    Shape(Shape),
}

/// Paragraph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    pub id: u32,
    pub style_id: u32,
    pub para_shape_id: u32,
    pub char_shape_id: u32,
    pub level: u8,
    pub inlines: Vec<Inline>,
}

impl Paragraph {
    pub fn new(id: u32) -> Self {
        Paragraph {
            id,
            style_id: 0,
            para_shape_id: 0,
            char_shape_id: 0,
            level: 0,
            inlines: Vec::new(),
        }
    }
}

/// Inline content (Text, Control, Field, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Inline {
    Text(TextRun),
    Control(Control),
    Field(Field),
}

/// Text run with consistent character shape
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
    pub char_shape_id: u32,
}

/// Control (embedded object)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Control {
    Table(Table),
    Picture(Picture),
    OLE(OLE),
    TextBox(TextBox),
    Equation(Equation),
}

/// Table control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: u32,
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<TableCell>,
    pub border_fill_id: u32,
}

/// Table cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    pub row: usize,
    pub col: usize,
    pub row_span: usize,
    pub col_span: usize,
    pub content: Vec<Block>, // Cell content (paragraphs, etc.)
}

/// Picture/Image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    pub id: u32,
    pub bindata_id: u32,
    pub filename: String,
    pub rect: Rect,
    pub margin: Margin,
}

/// OLE Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OLE {
    pub id: u32,
    pub bindata_id: u32,
}

/// Text box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBox {
    pub id: u32,
    pub rect: Rect,
    pub margin: Margin,
    pub content: Vec<Block>,
}

/// Equation (formula)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equation {
    pub id: u32,
    pub bindata_id: u32,
}

/// Field (date, page number, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Field {
    Date,
    Time,
    PageNumber,
    PageCount,
    FootnoteNumber,
    Hyperlink(String),
}

/// Generic shape
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub id: u32,
    pub shape_type: ShapeType,
    pub rect: Rect,
}

/// Shape types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShapeType {
    Rectangle,
    Ellipse,
    Arc,
    Polygon,
    Curve,
    Container,
}

/// Character shape (글자 모양)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharShape {
    pub id: u32,
    pub font_id: u32,
    pub font_size: HwpUnit,
    pub bold: bool,
    pub italic: bool,
    pub underline: LineStyle,
    pub strikethrough: bool,
    pub color: Color,
    pub background_color: Color,
}

impl CharShape {
    pub fn new(id: u32) -> Self {
        CharShape {
            id,
            font_id: 0,
            font_size: HwpUnit(200), // 10pt default
            bold: false,
            italic: false,
            underline: LineStyle::None,
            strikethrough: false,
            color: Color(0),                   // Black
            background_color: Color(0xFFFFFF), // White
        }
    }
}

/// Paragraph shape (문단 모양)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParaShape {
    pub id: u32,
    pub alignment: Alignment,
    pub indent_left: HwpUnit,
    pub indent_right: HwpUnit,
    pub indent_first: HwpUnit,
    pub spacing_before: HwpUnit,
    pub spacing_after: HwpUnit,
    pub line_spacing: u16, // in 1/100 of line height
    pub tabs: Vec<TabStop>,
}

impl ParaShape {
    pub fn new(id: u32) -> Self {
        ParaShape {
            id,
            alignment: Alignment::Left,
            indent_left: HwpUnit(0),
            indent_right: HwpUnit(0),
            indent_first: HwpUnit(0),
            spacing_before: HwpUnit(0),
            spacing_after: HwpUnit(0),
            line_spacing: 100,
            tabs: Vec::new(),
        }
    }
}

/// Tab stop definition
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TabStop {
    pub position: HwpUnit,
    pub align: TabAlign,
    pub leader: TabLeader,
}

/// Tab alignment
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TabAlign {
    Left,
    Right,
    Center,
    Decimal,
}

/// Tab leader
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TabLeader {
    None,
    Dot,
    Line,
    Heavy,
    Space,
}

/// Style (스타일)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub id: u32,
    pub name: String,
    pub style_type: StyleType,
    pub parent_id: u32,
    pub char_shape_id: u32,
    pub para_shape_id: u32,
}

/// Style type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StyleType {
    Paragraph = 0,
    Character = 1,
    Table = 2,
    List = 3,
}

impl StyleType {
    pub fn from_u16(val: u16) -> Option<Self> {
        match val {
            0 => Some(StyleType::Paragraph),
            1 => Some(StyleType::Character),
            2 => Some(StyleType::Table),
            3 => Some(StyleType::List),
            _ => None,
        }
    }
}

/// Border and fill (테두리/배경)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderFill {
    pub id: u32,
    pub left: Border,
    pub right: Border,
    pub top: Border,
    pub bottom: Border,
    pub diagonal: Border,
    pub fill_type: FillType,
    pub fill_color: Color,
    pub background_color: Color,
}

/// Border definition
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Border {
    pub style: LineStyle,
    pub width: u8, // in 1/20 mm
    pub color: Color,
}

impl Border {
    pub fn none() -> Self {
        Border {
            style: LineStyle::None,
            width: 0,
            color: Color(0),
        }
    }
}

/// Fill type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FillType {
    None = 0,
    Solid = 1,
    Pattern = 2,
    Gradient = 3,
    Image = 4,
}

impl FillType {
    pub fn from_u16(val: u16) -> Option<Self> {
        match val {
            0 => Some(FillType::None),
            1 => Some(FillType::Solid),
            2 => Some(FillType::Pattern),
            3 => Some(FillType::Gradient),
            4 => Some(FillType::Image),
            _ => None,
        }
    }
}
