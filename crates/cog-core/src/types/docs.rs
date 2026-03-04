use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(default)]
    pub document_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<Body>,
    #[serde(default)]
    pub headers: Option<serde_json::Value>,
    #[serde(default)]
    pub footers: Option<serde_json::Value>,
    #[serde(default)]
    pub footnotes: Option<serde_json::Value>,
    #[serde(default)]
    pub document_style: Option<serde_json::Value>,
    #[serde(default)]
    pub named_styles: Option<serde_json::Value>,
    #[serde(default)]
    pub named_ranges: Option<serde_json::Value>,
    #[serde(default)]
    pub lists: Option<serde_json::Value>,
    #[serde(default)]
    pub inline_objects: Option<serde_json::Value>,
    #[serde(default)]
    pub positioned_objects: Option<serde_json::Value>,
    #[serde(default)]
    pub revision_id: Option<String>,
    #[serde(default)]
    pub suggestions_view_mode: Option<String>,
    #[serde(default)]
    pub tabs: Vec<DocumentTab>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentTab {
    #[serde(default)]
    pub tab_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<Body>,
    #[serde(default)]
    pub headers: Option<serde_json::Value>,
    #[serde(default)]
    pub footers: Option<serde_json::Value>,
    #[serde(default)]
    pub document_style: Option<serde_json::Value>,
    #[serde(default)]
    pub inline_objects: Option<serde_json::Value>,
    #[serde(default)]
    pub positioned_objects: Option<serde_json::Value>,
    #[serde(default)]
    pub tab_properties: Option<TabProperties>,
    #[serde(default)]
    pub child_tabs: Vec<DocumentTab>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabProperties {
    #[serde(default)]
    pub tab_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub parent_tab_id: Option<String>,
    #[serde(default)]
    pub index: Option<i64>,
    #[serde(default)]
    pub nesting_level: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    #[serde(default)]
    pub content: Vec<StructuralElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralElement {
    #[serde(default)]
    pub start_index: Option<i64>,
    #[serde(default)]
    pub end_index: Option<i64>,
    #[serde(default)]
    pub paragraph: Option<Paragraph>,
    #[serde(default)]
    pub section_break: Option<SectionBreak>,
    #[serde(default)]
    pub table: Option<Table>,
    #[serde(default)]
    pub table_of_contents: Option<TableOfContents>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionBreak {
    #[serde(default)]
    pub section_style: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    #[serde(default)]
    pub rows: Option<i64>,
    #[serde(default)]
    pub columns: Option<i64>,
    #[serde(default)]
    pub table_rows: Vec<serde_json::Value>,
    #[serde(default)]
    pub table_style: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableOfContents {
    #[serde(default)]
    pub content: Vec<StructuralElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paragraph {
    #[serde(default)]
    pub elements: Vec<ParagraphElement>,
    #[serde(default)]
    pub paragraph_style: Option<ParagraphStyle>,
    #[serde(default)]
    pub bullet: Option<Bullet>,
    #[serde(default)]
    pub suggested_paragraph_style_changes: Option<serde_json::Value>,
    #[serde(default)]
    pub suggested_bullet_changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphStyle {
    #[serde(default)]
    pub heading_id: Option<String>,
    #[serde(default)]
    pub named_style_type: Option<String>,
    #[serde(default)]
    pub alignment: Option<String>,
    #[serde(default)]
    pub direction: Option<String>,
    #[serde(default)]
    pub line_spacing: Option<f64>,
    #[serde(default)]
    pub spacing_mode: Option<String>,
    #[serde(default)]
    pub space_above: Option<Dimension>,
    #[serde(default)]
    pub space_below: Option<Dimension>,
    #[serde(default)]
    pub indent_first_line: Option<Dimension>,
    #[serde(default)]
    pub indent_start: Option<Dimension>,
    #[serde(default)]
    pub indent_end: Option<Dimension>,
    #[serde(default)]
    pub keep_lines_together: bool,
    #[serde(default)]
    pub keep_with_next: bool,
    #[serde(default)]
    pub avoid_widow_and_orphan: bool,
    #[serde(default)]
    pub shading: Option<Shading>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    #[serde(default)]
    pub magnitude: Option<f64>,
    #[serde(default)]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shading {
    #[serde(default)]
    pub background_color: Option<OptionalColor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalColor {
    #[serde(default)]
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    #[serde(default)]
    pub rgb_color: Option<RgbColor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RgbColor {
    #[serde(default)]
    pub red: Option<f64>,
    #[serde(default)]
    pub green: Option<f64>,
    #[serde(default)]
    pub blue: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bullet {
    #[serde(default)]
    pub list_id: Option<String>,
    #[serde(default)]
    pub nesting_level: Option<i64>,
    #[serde(default)]
    pub text_style: Option<TextStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphElement {
    #[serde(default)]
    pub start_index: Option<i64>,
    #[serde(default)]
    pub end_index: Option<i64>,
    #[serde(default)]
    pub text_run: Option<TextRun>,
    #[serde(default)]
    pub auto_text: Option<AutoText>,
    #[serde(default)]
    pub page_break: Option<PageBreak>,
    #[serde(default)]
    pub column_break: Option<ColumnBreak>,
    #[serde(default)]
    pub footnote_reference: Option<FootnoteReference>,
    #[serde(default)]
    pub horizontal_rule: Option<HorizontalRule>,
    #[serde(default)]
    pub inline_object_element: Option<InlineObjectElement>,
    #[serde(default)]
    pub person: Option<PersonElement>,
    #[serde(default)]
    pub rich_link: Option<RichLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextRun {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub text_style: Option<TextStyle>,
    #[serde(default)]
    pub suggested_insertion_ids: Vec<String>,
    #[serde(default)]
    pub suggested_deletion_ids: Vec<String>,
    #[serde(default)]
    pub suggested_text_style_changes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle {
    #[serde(default)]
    pub bold: Option<bool>,
    #[serde(default)]
    pub italic: Option<bool>,
    #[serde(default)]
    pub underline: Option<bool>,
    #[serde(default)]
    pub strikethrough: Option<bool>,
    #[serde(default)]
    pub small_caps: Option<bool>,
    #[serde(default)]
    pub font_size: Option<Dimension>,
    #[serde(default)]
    pub foreground_color: Option<OptionalColor>,
    #[serde(default)]
    pub background_color: Option<OptionalColor>,
    #[serde(default)]
    pub link: Option<Link>,
    #[serde(default)]
    pub baseline_offset: Option<String>,
    #[serde(default)]
    pub weighted_font_family: Option<WeightedFontFamily>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedFontFamily {
    #[serde(default)]
    pub font_family: Option<String>,
    #[serde(default)]
    pub weight: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub bookmark_id: Option<String>,
    #[serde(default)]
    pub heading_id: Option<String>,
    #[serde(default)]
    pub tab_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoText {
    #[serde(rename = "type", default)]
    pub auto_text_type: Option<String>,
    #[serde(default)]
    pub suggested_insertion_ids: Vec<String>,
    #[serde(default)]
    pub suggested_deletion_ids: Vec<String>,
    #[serde(default)]
    pub text_style: Option<TextStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageBreak {
    #[serde(default)]
    pub text_style: Option<TextStyle>,
    #[serde(default)]
    pub suggested_insertion_ids: Vec<String>,
    #[serde(default)]
    pub suggested_deletion_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnBreak {
    #[serde(default)]
    pub text_style: Option<TextStyle>,
    #[serde(default)]
    pub suggested_insertion_ids: Vec<String>,
    #[serde(default)]
    pub suggested_deletion_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FootnoteReference {
    #[serde(default)]
    pub footnote_id: Option<String>,
    #[serde(default)]
    pub footnote_number: Option<String>,
    #[serde(default)]
    pub text_style: Option<TextStyle>,
    #[serde(default)]
    pub suggested_insertion_ids: Vec<String>,
    #[serde(default)]
    pub suggested_deletion_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalRule {
    #[serde(default)]
    pub text_style: Option<TextStyle>,
    #[serde(default)]
    pub suggested_insertion_ids: Vec<String>,
    #[serde(default)]
    pub suggested_deletion_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineObjectElement {
    #[serde(default)]
    pub inline_object_id: Option<String>,
    #[serde(default)]
    pub text_style: Option<TextStyle>,
    #[serde(default)]
    pub suggested_insertion_ids: Vec<String>,
    #[serde(default)]
    pub suggested_deletion_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonElement {
    #[serde(default)]
    pub person_id: Option<String>,
    #[serde(default)]
    pub person_properties: Option<PersonProperties>,
    #[serde(default)]
    pub suggested_insertion_ids: Vec<String>,
    #[serde(default)]
    pub suggested_deletion_ids: Vec<String>,
    #[serde(default)]
    pub text_style: Option<TextStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonProperties {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichLink {
    #[serde(default)]
    pub rich_link_id: Option<String>,
    #[serde(default)]
    pub rich_link_properties: Option<RichLinkProperties>,
    #[serde(default)]
    pub suggested_insertion_ids: Vec<String>,
    #[serde(default)]
    pub suggested_deletion_ids: Vec<String>,
    #[serde(default)]
    pub text_style: Option<TextStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichLinkProperties {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedRange {
    #[serde(default)]
    pub named_range_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub ranges: Vec<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    #[serde(default)]
    pub segment_id: Option<String>,
    #[serde(default)]
    pub tab_id: Option<String>,
    #[serde(default)]
    pub start_index: Option<i64>,
    #[serde(default)]
    pub end_index: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineObject {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub inline_object_properties: Option<InlineObjectProperties>,
    #[serde(default)]
    pub suggested_inline_object_properties_changes: Option<serde_json::Value>,
    #[serde(default)]
    pub suggested_insertion_id: Option<String>,
    #[serde(default)]
    pub suggested_deletion_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineObjectProperties {
    #[serde(default)]
    pub embedded_object: Option<EmbeddedObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedObject {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub embedded_object_border: Option<serde_json::Value>,
    #[serde(default)]
    pub size: Option<Size>,
    #[serde(default)]
    pub margin_top: Option<Dimension>,
    #[serde(default)]
    pub margin_bottom: Option<Dimension>,
    #[serde(default)]
    pub margin_left: Option<Dimension>,
    #[serde(default)]
    pub margin_right: Option<Dimension>,
    #[serde(default)]
    pub image_properties: Option<ImageProperties>,
    #[serde(default)]
    pub linked_content_reference: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    #[serde(default)]
    pub height: Option<Dimension>,
    #[serde(default)]
    pub width: Option<Dimension>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageProperties {
    #[serde(default)]
    pub content_uri: Option<String>,
    #[serde(default)]
    pub source_uri: Option<String>,
    #[serde(default)]
    pub brightness: Option<f64>,
    #[serde(default)]
    pub contrast: Option<f64>,
    #[serde(default)]
    pub transparency: Option<f64>,
    #[serde(default)]
    pub crop_properties: Option<CropProperties>,
    #[serde(default)]
    pub angle: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropProperties {
    #[serde(default)]
    pub offset_left: Option<f64>,
    #[serde(default)]
    pub offset_right: Option<f64>,
    #[serde(default)]
    pub offset_top: Option<f64>,
    #[serde(default)]
    pub offset_bottom: Option<f64>,
    #[serde(default)]
    pub angle: Option<f64>,
}
