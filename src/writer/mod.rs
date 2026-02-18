use crate::common::HwpResult;
use crate::model::Document;
use std::collections::HashMap;

mod zip_utils;
use zip_utils::write_zip_stored;

/// Generate ODT (OpenDocument Text) file from document model
pub fn generate_odt(doc: &Document) -> HwpResult<Vec<u8>> {
    let mut entries: Vec<(&str, Vec<u8>)> = Vec::new();

    // 1. mimetype (must be first, uncompressed, no ZIP header)
    entries.push((
        "mimetype",
        b"application/vnd.oasis.opendocument.text".to_vec(),
    ));

    // 2. META-INF/manifest.xml
    let manifest = generate_manifest();
    entries.push(("META-INF/manifest.xml", manifest.into_bytes()));

    // 3. content.xml
    let content = generate_content_xml(doc)?;
    entries.push(("content.xml", content.into_bytes()));

    // 4. styles.xml
    let styles = generate_styles_xml(doc)?;
    entries.push(("styles.xml", styles.into_bytes()));

    // 5. settings.xml
    let settings = generate_settings_xml();
    entries.push(("settings.xml", settings.into_bytes()));

    // 6. meta.xml
    let meta = generate_meta_xml();
    entries.push(("meta.xml", meta.into_bytes()));

    // Package as ZIP
    let odt_data = write_zip_stored(&entries)?;
    Ok(odt_data)
}

fn generate_manifest() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<manifest:manifest xmlns:manifest="urn:oasis:names:tc:opendocument:xmlns:manifest:1.0" manifest:version="1.2">
  <manifest:file-entry manifest:media-type="application/vnd.oasis.opendocument.text" manifest:full-path="/"/>
  <manifest:file-entry manifest:media-type="text/xml" manifest:full-path="content.xml"/>
  <manifest:file-entry manifest:media-type="text/xml" manifest:full-path="styles.xml"/>
  <manifest:file-entry manifest:media-type="text/xml" manifest:full-path="settings.xml"/>
  <manifest:file-entry manifest:media-type="text/xml" manifest:full-path="meta.xml"/>
</manifest:manifest>"#
        .to_string()
}

fn generate_content_xml(doc: &Document) -> HwpResult<String> {
    let mut xml = String::new();
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-content
  xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
  xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
  xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0"
  xmlns:draw="urn:oasis:names:tc:opendocument:xmlns:drawing:1.0"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  office:version="1.2">
  <office:scripts/>
  <office:font-face-decls>"#,
    );

    // Add font faces
    for (idx, font_name) in doc.fonts.iter().enumerate() {
        xml.push_str(&format!(
            r#"
    <style:font-face style:name="F{}" svg:font-family="'{}'"
      style:font-family-generic="swiss" style:font-pitch="variable"/>"#,
            idx, font_name
        ));
    }

    xml.push_str(
        r#"
  </office:font-face-decls>
  <office:automatic-styles/>
  <office:body>
    <office:text>"#,
    );

    // Generate document content
    for section in &doc.sections {
        for block in &section.blocks {
            generate_block_content(&mut xml, block)?;
        }
    }

    xml.push_str(
        r#"
    </office:text>
  </office:body>
</office:document-content>"#,
    );
    Ok(xml)
}

fn generate_block_content(xml: &mut String, block: &crate::model::Block) -> HwpResult<()> {
    use crate::model::Block;

    match block {
        Block::Paragraph(para) => {
            xml.push_str(&format!(
                r#"
      <text:p text:style-name="P{}">"#,
                para.style_id
            ));

            for inline in &para.inlines {
                generate_inline_content(xml, inline)?;
            }

            xml.push_str("\n      </text:p>");
        }
        Block::Table(table) => {
            xml.push_str(&format!(
                r#"
      <table:table table:name="Table{}" table:style-name="Table1">"#,
                table.id
            ));

            // Group cells by row
            let mut rows: HashMap<usize, Vec<_>> = HashMap::new();
            for cell in &table.cells {
                rows.entry(cell.row).or_insert_with(Vec::new).push(cell);
            }

            for row_idx in 0..table.rows {
                xml.push_str("\n        <table:table-row>\n");
                if let Some(cells) = rows.get(&row_idx) {
                    for cell in cells {
                        xml.push_str(r#"          <table:table-cell table:value-type="string">"#);
                        for content in &cell.content {
                            generate_block_content(xml, content)?;
                        }
                        xml.push_str("\n          </table:table-cell>\n");
                    }
                }
                xml.push_str("        </table:table-row>");
            }

            xml.push_str("\n      </table:table>");
        }
        Block::Shape(_shape) => {
            // Shape handling - simplified for now
        }
    }

    Ok(())
}

fn generate_inline_content(xml: &mut String, inline: &crate::model::Inline) -> HwpResult<()> {
    use crate::model::Inline;

    match inline {
        Inline::Text(text_run) => {
            xml.push_str(&format!(
                r#"
        <text:span text:style-name="T{}">{}</text:span>"#,
                text_run.char_shape_id,
                escape_xml_for_content(&text_run.text)
            ));
        }
        Inline::Control(_ctrl) => {
            // Control handling
        }
        Inline::Field(field) => match field {
            crate::model::Field::PageNumber => {
                xml.push_str(r#"<text:page-number/>"#);
            }
            crate::model::Field::PageCount => {
                xml.push_str(r#"<text:page-count/>"#);
            }
            crate::model::Field::Date => {
                xml.push_str(r#"<text:date/>"#);
            }
            crate::model::Field::Time => {
                xml.push_str(r#"<text:time/>"#);
            }
            _ => {}
        },
    }

    Ok(())
}

fn generate_styles_xml(doc: &Document) -> HwpResult<String> {
    let mut xml = String::new();
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-styles
  xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
  xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0"
  xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
  xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0"
  xmlns:fo="urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0"
  xmlns:svg="urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0"
  office:version="1.2">
  <office:font-face-decls>"#,
    );

    // Add font faces
    for (idx, font_name) in doc.fonts.iter().enumerate() {
        xml.push_str(&format!(
            r#"
    <style:font-face style:name="F{}" svg:font-family="'{}'"
      style:font-family-generic="swiss" style:font-pitch="variable"/>"#,
            idx, font_name
        ));
    }

    xml.push_str(
        r#"
  </office:font-face-decls>
  <office:styles>
    <style:default-style style:family="paragraph">
      <style:paragraph-properties/>
      <style:text-properties/>
    </style:default-style>"#,
    );

    // Add character styles
    for char_shape in &doc.char_shapes {
        xml.push_str(&format!(
            r#"
    <style:style style:name="T{}" style:family="text">
      <style:text-properties style:font-name="F{}" fo:font-size="{}pt" fo:color="{}"{}/>"#,
            char_shape.id,
            char_shape.font_id,
            char_shape.font_size.to_pt(),
            char_shape.color.to_hex(),
            if char_shape.bold {
                " fo:font-weight=\"bold\""
            } else {
                ""
            }
        ));
        xml.push_str("\n    </style:style>");
    }

    // Add paragraph styles
    for para_shape in &doc.para_shapes {
        xml.push_str(&format!(
            r#"
    <style:style style:name="P{}" style:family="paragraph">
      <style:paragraph-properties fo:text-align="{}" fo:margin-left="{:?}mm" fo:margin-right="{:?}mm"/>"#,
            para_shape.id,
            para_shape.alignment.to_odt_str(),
            para_shape.indent_left.to_mm(),
            para_shape.indent_right.to_mm()
        ));
        xml.push_str("\n    </style:style>");
    }

    xml.push_str(
        r#"
  </office:styles>
  <office:automatic-styles>
    <style:style style:name="Table1" style:family="table">
      <style:table-properties table:border-model="collapsing"/>
    </style:style>
  </office:automatic-styles>
  <office:master-styles>
    <style:master-page style:name="Standard" style:page-layout-name="pm1"/>
  </office:master-styles>
</office:document-styles>"#,
    );

    Ok(xml)
}

fn generate_settings_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-settings
  xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
  office:version="1.2">
  <office:settings/>
</office:document-settings>"#
        .to_string()
}

fn generate_meta_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-meta
  xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
  xmlns:meta="urn:oasis:names:tc:opendocument:xmlns:meta:1.0"
  xmlns:dc="http://purl.org/dc/elements/1.1/"
  office:version="1.2">
  <office:meta>
    <meta:generator>HanCon/Rust</meta:generator>
  </office:meta>
</office:document-meta>"#
        .to_string()
}

fn escape_xml_for_content(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            _ => result.push(c),
        }
    }
    result
}
