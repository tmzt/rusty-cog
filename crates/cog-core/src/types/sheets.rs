use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spreadsheet {
    #[serde(default)]
    pub spreadsheet_id: Option<String>,
    #[serde(default)]
    pub properties: Option<SpreadsheetProperties>,
    #[serde(default)]
    pub sheets: Vec<Sheet>,
    #[serde(default)]
    pub named_ranges: Vec<NamedRange>,
    #[serde(default)]
    pub spreadsheet_url: Option<String>,
    #[serde(default)]
    pub developer_metadata: Vec<DeveloperMetadata>,
    #[serde(default)]
    pub data_sources: Vec<serde_json::Value>,
    #[serde(default)]
    pub data_source_schedules: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadsheetProperties {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(default)]
    pub auto_recalc: Option<String>,
    #[serde(default)]
    pub time_zone: Option<String>,
    #[serde(default)]
    pub default_format: Option<CellFormat>,
    #[serde(default)]
    pub iterative_calculation_settings: Option<IterativeCalculationSettings>,
    #[serde(default)]
    pub spreadsheet_theme: Option<SpreadsheetTheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IterativeCalculationSettings {
    #[serde(default)]
    pub max_iterations: Option<i64>,
    #[serde(default)]
    pub convergence_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadsheetTheme {
    #[serde(default)]
    pub primary_font_family: Option<String>,
    #[serde(default)]
    pub theme_colors: Vec<ThemeColorPair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColorPair {
    #[serde(default)]
    pub color_type: Option<String>,
    #[serde(default)]
    pub color: Option<ColorStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    #[serde(default)]
    pub properties: Option<SheetProperties>,
    #[serde(default)]
    pub data: Vec<GridData>,
    #[serde(default)]
    pub merges: Vec<GridRange>,
    #[serde(default)]
    pub conditional_formats: Vec<serde_json::Value>,
    #[serde(default)]
    pub filter_views: Vec<serde_json::Value>,
    #[serde(default)]
    pub protected_ranges: Vec<serde_json::Value>,
    #[serde(default)]
    pub basic_filter: Option<serde_json::Value>,
    #[serde(default)]
    pub charts: Vec<serde_json::Value>,
    #[serde(default)]
    pub banded_ranges: Vec<serde_json::Value>,
    #[serde(default)]
    pub developer_metadata: Vec<DeveloperMetadata>,
    #[serde(default)]
    pub row_count: Option<i64>,
    #[serde(default)]
    pub column_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetProperties {
    #[serde(default)]
    pub sheet_id: Option<i64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub index: Option<i64>,
    #[serde(default)]
    pub sheet_type: Option<String>,
    #[serde(default)]
    pub grid_properties: Option<GridProperties>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub tab_color: Option<ColorStyle>,
    #[serde(default)]
    pub tab_color_style: Option<ColorStyle>,
    #[serde(default)]
    pub right_to_left: bool,
    #[serde(default)]
    pub data_source_sheet_properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridProperties {
    #[serde(default)]
    pub row_count: Option<i64>,
    #[serde(default)]
    pub column_count: Option<i64>,
    #[serde(default)]
    pub frozen_row_count: Option<i64>,
    #[serde(default)]
    pub frozen_column_count: Option<i64>,
    #[serde(default)]
    pub hide_gridlines: bool,
    #[serde(default)]
    pub row_group_control_after: bool,
    #[serde(default)]
    pub column_group_control_after: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridData {
    #[serde(default)]
    pub start_row: Option<i64>,
    #[serde(default)]
    pub start_column: Option<i64>,
    #[serde(default)]
    pub row_data: Vec<RowData>,
    #[serde(default)]
    pub row_metadata: Vec<DimensionProperties>,
    #[serde(default)]
    pub column_metadata: Vec<DimensionProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowData {
    #[serde(default)]
    pub values: Vec<CellData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionProperties {
    #[serde(default)]
    pub hidden_by_filter: bool,
    #[serde(default)]
    pub hidden_by_user: bool,
    #[serde(default)]
    pub pixel_size: Option<i64>,
    #[serde(default)]
    pub developer_metadata: Vec<DeveloperMetadata>,
    #[serde(default)]
    pub data_source_column_reference: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellData {
    #[serde(default)]
    pub user_entered_value: Option<ExtendedValue>,
    #[serde(default)]
    pub effective_value: Option<ExtendedValue>,
    #[serde(default)]
    pub formatted_value: Option<String>,
    #[serde(default)]
    pub user_entered_format: Option<CellFormat>,
    #[serde(default)]
    pub effective_format: Option<CellFormat>,
    #[serde(default)]
    pub hyperlink: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub text_format_runs: Vec<TextFormatRun>,
    #[serde(default)]
    pub data_validation: Option<DataValidationRule>,
    #[serde(default)]
    pub pivot_table: Option<serde_json::Value>,
    #[serde(default)]
    pub data_source_table: Option<serde_json::Value>,
    #[serde(default)]
    pub data_source_formula: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedValue {
    #[serde(default)]
    pub number_value: Option<f64>,
    #[serde(default)]
    pub string_value: Option<String>,
    #[serde(default)]
    pub bool_value: Option<bool>,
    #[serde(default)]
    pub formula_value: Option<String>,
    #[serde(default)]
    pub error_value: Option<ErrorValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorValue {
    #[serde(rename = "type", default)]
    pub error_type: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextFormatRun {
    #[serde(default)]
    pub start_index: Option<i64>,
    #[serde(default)]
    pub format: Option<TextFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextFormat {
    #[serde(default)]
    pub foreground_color: Option<ColorStyle>,
    #[serde(default)]
    pub foreground_color_style: Option<ColorStyle>,
    #[serde(default)]
    pub font_family: Option<String>,
    #[serde(default)]
    pub font_size: Option<i64>,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub strikethrough: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub link: Option<LinkValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkValue {
    #[serde(default)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataValidationRule {
    #[serde(default)]
    pub condition: Option<BooleanCondition>,
    #[serde(default)]
    pub input_message: Option<String>,
    #[serde(default)]
    pub strict: bool,
    #[serde(default)]
    pub show_custom_ui: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanCondition {
    #[serde(rename = "type", default)]
    pub condition_type: Option<String>,
    #[serde(default)]
    pub values: Vec<ConditionValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionValue {
    #[serde(default)]
    pub relative_date: Option<String>,
    #[serde(default)]
    pub user_entered_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellFormat {
    #[serde(default)]
    pub number_format: Option<NumberFormat>,
    #[serde(default)]
    pub background_color: Option<ColorValue>,
    #[serde(default)]
    pub background_color_style: Option<ColorStyle>,
    #[serde(default)]
    pub borders: Option<Borders>,
    #[serde(default)]
    pub padding: Option<Padding>,
    #[serde(default)]
    pub horizontal_alignment: Option<String>,
    #[serde(default)]
    pub vertical_alignment: Option<String>,
    #[serde(default)]
    pub wrap_strategy: Option<String>,
    #[serde(default)]
    pub text_direction: Option<String>,
    #[serde(default)]
    pub text_format: Option<TextFormat>,
    #[serde(default)]
    pub hyperlink_display_type: Option<String>,
    #[serde(default)]
    pub text_rotation: Option<TextRotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFormat {
    #[serde(rename = "type", default)]
    pub format_type: Option<String>,
    #[serde(default)]
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorValue {
    #[serde(default)]
    pub red: Option<f64>,
    #[serde(default)]
    pub green: Option<f64>,
    #[serde(default)]
    pub blue: Option<f64>,
    #[serde(default)]
    pub alpha: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorStyle {
    #[serde(default)]
    pub rgb_color: Option<ColorValue>,
    #[serde(default)]
    pub theme_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Borders {
    #[serde(default)]
    pub top: Option<Border>,
    #[serde(default)]
    pub bottom: Option<Border>,
    #[serde(default)]
    pub left: Option<Border>,
    #[serde(default)]
    pub right: Option<Border>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Border {
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default)]
    pub width: Option<i64>,
    #[serde(default)]
    pub color: Option<ColorValue>,
    #[serde(default)]
    pub color_style: Option<ColorStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Padding {
    #[serde(default)]
    pub top: Option<i64>,
    #[serde(default)]
    pub bottom: Option<i64>,
    #[serde(default)]
    pub left: Option<i64>,
    #[serde(default)]
    pub right: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextRotation {
    #[serde(default)]
    pub angle: Option<i64>,
    #[serde(default)]
    pub vertical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridRange {
    #[serde(default)]
    pub sheet_id: Option<i64>,
    #[serde(default)]
    pub start_row_index: Option<i64>,
    #[serde(default)]
    pub end_row_index: Option<i64>,
    #[serde(default)]
    pub start_column_index: Option<i64>,
    #[serde(default)]
    pub end_column_index: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRange {
    #[serde(default)]
    pub range: Option<String>,
    #[serde(default)]
    pub major_dimension: Option<String>,
    #[serde(default)]
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateRequest {
    #[serde(default)]
    pub requests: Vec<serde_json::Value>,
    #[serde(default)]
    pub include_spreadsheet_in_response: bool,
    #[serde(default)]
    pub response_ranges: Vec<String>,
    #[serde(default)]
    pub response_include_grid_data: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedRange {
    #[serde(default)]
    pub named_range_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub range: Option<GridRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperMetadata {
    #[serde(default)]
    pub metadata_id: Option<i64>,
    #[serde(default)]
    pub metadata_key: Option<String>,
    #[serde(default)]
    pub metadata_value: Option<String>,
    #[serde(default)]
    pub location: Option<DeveloperMetadataLocation>,
    #[serde(default)]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperMetadataLocation {
    #[serde(default)]
    pub location_type: Option<String>,
    #[serde(default)]
    pub spreadsheet: bool,
    #[serde(default)]
    pub sheet_id: Option<i64>,
    #[serde(default)]
    pub dimension_range: Option<serde_json::Value>,
}
