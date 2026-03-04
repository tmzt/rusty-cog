use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[serde(default)]
    pub form_id: Option<String>,
    #[serde(default)]
    pub info: Option<FormInfo>,
    #[serde(default)]
    pub settings: Option<FormSettings>,
    #[serde(default)]
    pub items: Vec<FormItem>,
    #[serde(default)]
    pub revision_id: Option<String>,
    #[serde(default)]
    pub responder_uri: Option<String>,
    #[serde(default)]
    pub linked_sheet_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormInfo {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub document_title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormSettings {
    #[serde(default)]
    pub quiz_settings: Option<QuizSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizSettings {
    #[serde(default)]
    pub is_quiz: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormItem {
    #[serde(default)]
    pub item_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub question_item: Option<QuestionItem>,
    #[serde(default)]
    pub question_group_item: Option<QuestionGroupItem>,
    #[serde(default)]
    pub page_break_item: Option<PageBreakItem>,
    #[serde(default)]
    pub text_item: Option<TextItem>,
    #[serde(default)]
    pub image_item: Option<ImageItem>,
    #[serde(default)]
    pub video_item: Option<VideoItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionItem {
    #[serde(default)]
    pub question: Option<Question>,
    #[serde(default)]
    pub image: Option<FormImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionGroupItem {
    #[serde(default)]
    pub questions: Vec<Question>,
    #[serde(default)]
    pub image: Option<FormImage>,
    #[serde(default)]
    pub grid: Option<Grid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
    #[serde(default)]
    pub columns: Option<ChoiceQuestion>,
    #[serde(default)]
    pub shuffle_questions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageBreakItem {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextItem {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageItem {
    #[serde(default)]
    pub image: Option<FormImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoItem {
    #[serde(default)]
    pub video: Option<FormVideo>,
    #[serde(default)]
    pub caption: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormImage {
    #[serde(default)]
    pub content_uri: Option<String>,
    #[serde(default)]
    pub alt_text: Option<String>,
    #[serde(default)]
    pub properties: Option<MediaProperties>,
    #[serde(default)]
    pub source_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormVideo {
    #[serde(default)]
    pub youtube_uri: Option<String>,
    #[serde(default)]
    pub properties: Option<MediaProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaProperties {
    #[serde(default)]
    pub alignment: Option<String>,
    #[serde(default)]
    pub width: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    #[serde(default)]
    pub question_id: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub grading: Option<Grading>,
    #[serde(default)]
    pub choice_question: Option<ChoiceQuestion>,
    #[serde(default)]
    pub text_question: Option<TextQuestion>,
    #[serde(default)]
    pub scale_question: Option<ScaleQuestion>,
    #[serde(default)]
    pub date_question: Option<DateQuestion>,
    #[serde(default)]
    pub time_question: Option<TimeQuestion>,
    #[serde(default)]
    pub file_upload_question: Option<FileUploadQuestion>,
    #[serde(default)]
    pub row_question: Option<RowQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grading {
    #[serde(default)]
    pub point_value: Option<i64>,
    #[serde(default)]
    pub correct_answers: Option<CorrectAnswers>,
    #[serde(default)]
    pub when_right: Option<Feedback>,
    #[serde(default)]
    pub when_wrong: Option<Feedback>,
    #[serde(default)]
    pub general_feedback: Option<Feedback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectAnswers {
    #[serde(default)]
    pub answers: Vec<CorrectAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectAnswer {
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feedback {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub material: Vec<ExtraMaterial>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraMaterial {
    #[serde(default)]
    pub link: Option<TextLink>,
    #[serde(default)]
    pub video: Option<FormVideo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextLink {
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub display_text: Option<String>,
}

/// Represents the type of question via its variant fields on `Question`.
/// This enum is provided as a convenience for pattern-matching.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuestionType {
    ChoiceQuestion,
    TextQuestion,
    ScaleQuestion,
    DateQuestion,
    TimeQuestion,
    FileUploadQuestion,
    RowQuestion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceQuestion {
    #[serde(rename = "type", default)]
    pub choice_type: Option<String>,
    #[serde(default)]
    pub options: Vec<ChoiceOption>,
    #[serde(default)]
    pub shuffle: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceOption {
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub image: Option<FormImage>,
    #[serde(default)]
    pub is_other: bool,
    #[serde(default)]
    pub go_to_action: Option<String>,
    #[serde(default)]
    pub go_to_section_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextQuestion {
    #[serde(default)]
    pub paragraph: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleQuestion {
    #[serde(default)]
    pub low: Option<i64>,
    #[serde(default)]
    pub high: Option<i64>,
    #[serde(default)]
    pub low_label: Option<String>,
    #[serde(default)]
    pub high_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateQuestion {
    #[serde(default)]
    pub include_time: bool,
    #[serde(default)]
    pub include_year: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeQuestion {
    #[serde(default)]
    pub duration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadQuestion {
    #[serde(default)]
    pub folder_id: Option<String>,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub max_files: Option<i64>,
    #[serde(default)]
    pub max_file_size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowQuestion {
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormResponse {
    #[serde(default)]
    pub form_id: Option<String>,
    #[serde(default)]
    pub response_id: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub last_submitted_time: Option<String>,
    #[serde(default)]
    pub respondent_email: Option<String>,
    #[serde(default)]
    pub total_score: Option<f64>,
    #[serde(default)]
    pub answers: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormAnswer {
    #[serde(default)]
    pub question_id: Option<String>,
    #[serde(default)]
    pub grade: Option<AnswerGrade>,
    #[serde(default)]
    pub text_answers: Option<TextAnswers>,
    #[serde(default)]
    pub file_upload_answers: Option<FileUploadAnswers>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnswerGrade {
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub correct: bool,
    #[serde(default)]
    pub feedback: Option<Feedback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextAnswers {
    #[serde(default)]
    pub answers: Vec<TextAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextAnswer {
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadAnswers {
    #[serde(default)]
    pub answers: Vec<FileUploadAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadAnswer {
    #[serde(default)]
    pub file_id: Option<String>,
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
}
