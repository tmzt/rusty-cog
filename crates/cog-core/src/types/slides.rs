use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Presentation {
    #[serde(default)]
    pub presentation_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(default)]
    pub revision_id: Option<String>,
    #[serde(default)]
    pub page_size: Option<PageSize>,
    #[serde(default)]
    pub slides: Vec<Slide>,
    #[serde(default)]
    pub masters: Vec<SlidePage>,
    #[serde(default)]
    pub layouts: Vec<SlidePage>,
    #[serde(default)]
    pub notes_master: Option<SlidePage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageSize {
    #[serde(default)]
    pub width: Option<Dimension>,
    #[serde(default)]
    pub height: Option<Dimension>,
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
pub struct Slide {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub slide_properties: Option<SlideProperties>,
    #[serde(default)]
    pub page_elements: Vec<PageElement>,
    #[serde(default)]
    pub page_type: Option<String>,
    #[serde(default)]
    pub revision_id: Option<String>,
    #[serde(default)]
    pub page_properties: Option<PageProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlidePage {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub page_type: Option<String>,
    #[serde(default)]
    pub page_elements: Vec<PageElement>,
    #[serde(default)]
    pub page_properties: Option<PageProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideProperties {
    #[serde(default)]
    pub layout_object_id: Option<String>,
    #[serde(default)]
    pub master_object_id: Option<String>,
    #[serde(default)]
    pub notes_page: Option<NotesPage>,
    #[serde(default)]
    pub is_skipped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageProperties {
    #[serde(default)]
    pub page_background_fill: Option<PageBackgroundFill>,
    #[serde(default)]
    pub color_scheme: Option<ColorScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageBackgroundFill {
    #[serde(default)]
    pub property_state: Option<String>,
    #[serde(default)]
    pub solid_fill: Option<SolidFill>,
    #[serde(default)]
    pub stretched_picture_fill: Option<StretchedPictureFill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolidFill {
    #[serde(default)]
    pub color: Option<OpaqueColor>,
    #[serde(default)]
    pub alpha: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpaqueColor {
    #[serde(default)]
    pub rgb_color: Option<RgbColor>,
    #[serde(default)]
    pub theme_color: Option<String>,
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
pub struct StretchedPictureFill {
    #[serde(default)]
    pub content_url: Option<String>,
    #[serde(default)]
    pub size: Option<PageSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorScheme {
    #[serde(default)]
    pub colors: Vec<ThemeColorPair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColorPair {
    #[serde(rename = "type", default)]
    pub color_type: Option<String>,
    #[serde(default)]
    pub color: Option<RgbColor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageElement {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub size: Option<PageSize>,
    #[serde(default)]
    pub transform: Option<AffineTransform>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub shape: Option<Shape>,
    #[serde(default)]
    pub image: Option<ImageElement>,
    #[serde(default)]
    pub video: Option<VideoElement>,
    #[serde(default)]
    pub table: Option<TableElement>,
    #[serde(default)]
    pub line: Option<LineElement>,
    #[serde(default)]
    pub sheets_chart: Option<SheetsChart>,
    #[serde(default)]
    pub element_group: Option<ElementGroup>,
    #[serde(default)]
    pub word_art: Option<WordArt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffineTransform {
    #[serde(default)]
    pub scale_x: Option<f64>,
    #[serde(default)]
    pub scale_y: Option<f64>,
    #[serde(default)]
    pub shear_x: Option<f64>,
    #[serde(default)]
    pub shear_y: Option<f64>,
    #[serde(default)]
    pub translate_x: Option<f64>,
    #[serde(default)]
    pub translate_y: Option<f64>,
    #[serde(default)]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shape {
    #[serde(default)]
    pub shape_type: Option<String>,
    #[serde(default)]
    pub text: Option<TextContent>,
    #[serde(default)]
    pub shape_properties: Option<ShapeProperties>,
    #[serde(default)]
    pub placeholder: Option<Placeholder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeProperties {
    #[serde(default)]
    pub shape_background_fill: Option<ShapeFill>,
    #[serde(default)]
    pub outline: Option<Outline>,
    #[serde(default)]
    pub shadow: Option<Shadow>,
    #[serde(default)]
    pub link: Option<SlideLink>,
    #[serde(default)]
    pub content_alignment: Option<String>,
    #[serde(default)]
    pub autofit: Option<Autofit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeFill {
    #[serde(default)]
    pub property_state: Option<String>,
    #[serde(default)]
    pub solid_fill: Option<SolidFill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outline {
    #[serde(default)]
    pub outline_fill: Option<OutlineFill>,
    #[serde(default)]
    pub weight: Option<Dimension>,
    #[serde(default)]
    pub dash_style: Option<String>,
    #[serde(default)]
    pub property_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutlineFill {
    #[serde(default)]
    pub solid_fill: Option<SolidFill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shadow {
    #[serde(rename = "type", default)]
    pub shadow_type: Option<String>,
    #[serde(default)]
    pub transform: Option<AffineTransform>,
    #[serde(default)]
    pub alignment: Option<String>,
    #[serde(default)]
    pub blur_radius: Option<Dimension>,
    #[serde(default)]
    pub color: Option<OpaqueColor>,
    #[serde(default)]
    pub alpha: Option<f64>,
    #[serde(default)]
    pub rotate_with_shape: bool,
    #[serde(default)]
    pub property_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideLink {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub relative_link: Option<String>,
    #[serde(default)]
    pub page_object_id: Option<String>,
    #[serde(default)]
    pub slide_index: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Autofit {
    #[serde(default)]
    pub autofit_type: Option<String>,
    #[serde(default)]
    pub font_scale: Option<f64>,
    #[serde(default)]
    pub line_spacing_reduction: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Placeholder {
    #[serde(rename = "type", default)]
    pub placeholder_type: Option<String>,
    #[serde(default)]
    pub index: Option<i64>,
    #[serde(default)]
    pub parent_object_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextContent {
    #[serde(default)]
    pub text_elements: Vec<TextElement>,
    #[serde(default)]
    pub lists: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextElement {
    #[serde(default)]
    pub start_index: Option<i64>,
    #[serde(default)]
    pub end_index: Option<i64>,
    #[serde(default)]
    pub paragraph_marker: Option<ParagraphMarker>,
    #[serde(default)]
    pub text_run: Option<SlideTextRun>,
    #[serde(default)]
    pub auto_text: Option<SlideAutoText>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphMarker {
    #[serde(default)]
    pub style: Option<ParagraphStyle>,
    #[serde(default)]
    pub bullet: Option<SlideBullet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphStyle {
    #[serde(default)]
    pub line_spacing: Option<f64>,
    #[serde(default)]
    pub alignment: Option<String>,
    #[serde(default)]
    pub indent_start: Option<Dimension>,
    #[serde(default)]
    pub indent_end: Option<Dimension>,
    #[serde(default)]
    pub indent_first_line: Option<Dimension>,
    #[serde(default)]
    pub space_above: Option<Dimension>,
    #[serde(default)]
    pub space_below: Option<Dimension>,
    #[serde(default)]
    pub spacing_mode: Option<String>,
    #[serde(default)]
    pub direction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideBullet {
    #[serde(default)]
    pub list_id: Option<String>,
    #[serde(default)]
    pub nesting_level: Option<i64>,
    #[serde(default)]
    pub glyph: Option<String>,
    #[serde(default)]
    pub bullet_style: Option<SlideTextStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideTextRun {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub style: Option<SlideTextStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideAutoText {
    #[serde(rename = "type", default)]
    pub auto_text_type: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub style: Option<SlideTextStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideTextStyle {
    #[serde(default)]
    pub background_color: Option<OptionalColorStyle>,
    #[serde(default)]
    pub foreground_color: Option<OptionalColorStyle>,
    #[serde(default)]
    pub bold: Option<bool>,
    #[serde(default)]
    pub italic: Option<bool>,
    #[serde(default)]
    pub font_family: Option<String>,
    #[serde(default)]
    pub font_size: Option<Dimension>,
    #[serde(default)]
    pub link: Option<SlideLink>,
    #[serde(default)]
    pub baseline_offset: Option<String>,
    #[serde(default)]
    pub small_caps: Option<bool>,
    #[serde(default)]
    pub strikethrough: Option<bool>,
    #[serde(default)]
    pub underline: Option<bool>,
    #[serde(default)]
    pub weighted_font_family: Option<WeightedFontFamily>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalColorStyle {
    #[serde(default)]
    pub opaque_color: Option<OpaqueColor>,
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
pub struct ImageElement {
    #[serde(default)]
    pub content_url: Option<String>,
    #[serde(default)]
    pub source_url: Option<String>,
    #[serde(default)]
    pub image_properties: Option<ImageProperties>,
    #[serde(default)]
    pub placeholder: Option<Placeholder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageProperties {
    #[serde(default)]
    pub crop_properties: Option<CropProperties>,
    #[serde(default)]
    pub transparency: Option<f64>,
    #[serde(default)]
    pub brightness: Option<f64>,
    #[serde(default)]
    pub contrast: Option<f64>,
    #[serde(default)]
    pub recolor: Option<serde_json::Value>,
    #[serde(default)]
    pub outline: Option<Outline>,
    #[serde(default)]
    pub shadow: Option<Shadow>,
    #[serde(default)]
    pub link: Option<SlideLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropProperties {
    #[serde(default)]
    pub left_offset: Option<f64>,
    #[serde(default)]
    pub right_offset: Option<f64>,
    #[serde(default)]
    pub top_offset: Option<f64>,
    #[serde(default)]
    pub bottom_offset: Option<f64>,
    #[serde(default)]
    pub angle: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoElement {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub video_properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableElement {
    #[serde(default)]
    pub rows: Option<i64>,
    #[serde(default)]
    pub columns: Option<i64>,
    #[serde(default)]
    pub table_rows: Vec<serde_json::Value>,
    #[serde(default)]
    pub table_columns: Vec<serde_json::Value>,
    #[serde(default)]
    pub horizontal_border_rows: Vec<serde_json::Value>,
    #[serde(default)]
    pub vertical_border_rows: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineElement {
    #[serde(default)]
    pub line_properties: Option<LineProperties>,
    #[serde(default)]
    pub line_type: Option<String>,
    #[serde(default)]
    pub line_category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineProperties {
    #[serde(default)]
    pub line_fill: Option<LineFill>,
    #[serde(default)]
    pub weight: Option<Dimension>,
    #[serde(default)]
    pub dash_style: Option<String>,
    #[serde(default)]
    pub start_arrow: Option<String>,
    #[serde(default)]
    pub end_arrow: Option<String>,
    #[serde(default)]
    pub start_connection: Option<LineConnection>,
    #[serde(default)]
    pub end_connection: Option<LineConnection>,
    #[serde(default)]
    pub link: Option<SlideLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineFill {
    #[serde(default)]
    pub solid_fill: Option<SolidFill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineConnection {
    #[serde(default)]
    pub connected_object_id: Option<String>,
    #[serde(default)]
    pub connection_site_index: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetsChart {
    #[serde(default)]
    pub spreadsheet_id: Option<String>,
    #[serde(default)]
    pub chart_id: Option<i64>,
    #[serde(default)]
    pub content_url: Option<String>,
    #[serde(default)]
    pub sheets_chart_properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementGroup {
    #[serde(default)]
    pub children: Vec<PageElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WordArt {
    #[serde(default)]
    pub rendered_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesPage {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub page_type: Option<String>,
    #[serde(default)]
    pub page_elements: Vec<PageElement>,
    #[serde(default)]
    pub notes_properties: Option<NotesProperties>,
    #[serde(default)]
    pub page_properties: Option<PageProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesProperties {
    #[serde(default)]
    pub speaker_notes_object_id: Option<String>,
}
