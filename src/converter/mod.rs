use crate::common::HwpResult;
use crate::model::{Block, Document, Inline};

/// Convert document model to XML string (intermediate XHWP format)
pub fn convert_to_xml(doc: &Document) -> HwpResult<String> {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<HwpDoc>\n");

    // DocInfo section
    xml.push_str("  <DocInfo>\n");
    xml.push_str("    <IdMappings>\n");

    // FaceNames
    for (idx, font) in doc.fonts.iter().enumerate() {
        xml.push_str(&format!(
            "      <FaceName id=\"{}\" name=\"{}\"/>\n",
            idx,
            escape_xml(font)
        ));
    }

    // CharShapes
    for char_shape in &doc.char_shapes {
        xml.push_str(&format!(
            "      <CharShape id=\"{}\" font_id=\"{}\" size=\"{}\" bold=\"{}\" italic=\"{}\" color=\"{}\"/>\n",
            char_shape.id,
            char_shape.font_id,
            char_shape.font_size.0,
            char_shape.bold,
            char_shape.italic,
            char_shape.color.to_hex()
        ));
    }

    // ParaShapes
    for para_shape in &doc.para_shapes {
        xml.push_str(&format!(
            "      <ParaShape id=\"{}\" alignment=\"{}\" indent_left=\"{}\" indent_right=\"{}\" indent_first=\"{}\"/>\n",
            para_shape.id,
            para_shape.alignment.to_odt_str(),
            para_shape.indent_left.0,
            para_shape.indent_right.0,
            para_shape.indent_first.0
        ));
    }

    // Styles
    for style in &doc.styles {
        xml.push_str(&format!(
            "      <Style id=\"{}\" name=\"{}\" type=\"{}\"/>\n",
            style.id,
            escape_xml(&style.name),
            match style.style_type {
                crate::model::StyleType::Paragraph => "paragraph",
                crate::model::StyleType::Character => "character",
                crate::model::StyleType::Table => "table",
                crate::model::StyleType::List => "list",
            }
        ));
    }

    // BorderFills
    for border_fill in &doc.border_fills {
        xml.push_str(&format!(
            "      <BorderFill id=\"{}\" fill_type=\"{:?}\" fill_color=\"{}\"/>\n",
            border_fill.id,
            border_fill.fill_type,
            border_fill.fill_color.to_hex()
        ));
    }

    xml.push_str("    </IdMappings>\n");
    xml.push_str("  </DocInfo>\n");

    // BodyText section
    xml.push_str("  <BodyText>\n");
    for (idx, section) in doc.sections.iter().enumerate() {
        xml.push_str(&format!("    <Section id=\"{}\">\n", idx));
        for block in &section.blocks {
            append_block_xml(&mut xml, block, 6);
        }
        xml.push_str("    </Section>\n");
    }
    xml.push_str("  </BodyText>\n");

    xml.push_str("</HwpDoc>\n");
    Ok(xml)
}

fn append_block_xml(xml: &mut String, block: &Block, indent: usize) {
    let indent_str = " ".repeat(indent);
    match block {
        Block::Paragraph(para) => {
            xml.push_str(&format!(
                "{}<Paragraph id=\"{}\" style_id=\"{}\" char_shape_id=\"{}\">\n",
                indent_str, para.id, para.style_id, para.char_shape_id
            ));
            for inline in &para.inlines {
                append_inline_xml(xml, inline, indent + 2);
            }
            xml.push_str(&format!("{}</Paragraph>\n", indent_str));
        }
        Block::Table(table) => {
            xml.push_str(&format!(
                "{}<Table id=\"{}\" rows=\"{}\" cols=\"{}\">\n",
                indent_str, table.id, table.rows, table.cols
            ));
            for cell in &table.cells {
                xml.push_str(&format!(
                    "{}  <Cell row=\"{}\" col=\"{}\" row_span=\"{}\" col_span=\"{}\">\n",
                    indent_str, cell.row, cell.col, cell.row_span, cell.col_span
                ));
                for content in &cell.content {
                    append_block_xml(xml, content, indent + 4);
                }
                xml.push_str(&format!("{}  </Cell>\n", indent_str));
            }
            xml.push_str(&format!("{}</Table>\n", indent_str));
        }
        Block::Shape(shape) => {
            xml.push_str(&format!(
                "{}<Shape id=\"{}\" type=\"{:?}\"/>\n",
                indent_str, shape.id, shape.shape_type
            ));
        }
    }
}

fn append_inline_xml(xml: &mut String, inline: &Inline, indent: usize) {
    let indent_str = " ".repeat(indent);
    match inline {
        Inline::Text(text_run) => {
            xml.push_str(&format!(
                "{}<Text char_shape_id=\"{}\">{}</Text>\n",
                indent_str,
                text_run.char_shape_id,
                escape_xml(&text_run.text)
            ));
        }
        Inline::Control(ctrl) => {
            xml.push_str(&format!("{}<Control type=\"{:?}\"/>\n", indent_str, ctrl));
        }
        Inline::Field(field) => {
            xml.push_str(&format!("{}<Field type=\"{:?}\"/>\n", indent_str, field));
        }
    }
}

fn escape_xml(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&apos;"),
            _ => result.push(c),
        }
    }
    result
}
