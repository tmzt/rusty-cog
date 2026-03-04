use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(default)]
    pub script_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default)]
    pub creator: Option<GoogleUser>,
    #[serde(default)]
    pub last_modify_user: Option<GoogleUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleUser {
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptFile {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type", default)]
    pub file_type: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub last_modify_user: Option<GoogleUser>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default)]
    pub function_set: Option<FunctionSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionSet {
    #[serde(default)]
    pub values: Vec<GoogleFunction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleFunction {
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptContent {
    #[serde(default)]
    pub script_id: Option<String>,
    #[serde(default)]
    pub files: Vec<ScriptFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionRequest {
    pub function: String,
    #[serde(default)]
    pub parameters: Vec<serde_json::Value>,
    #[serde(default)]
    pub dev_mode: bool,
    #[serde(default)]
    pub session_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionResponse {
    #[serde(default)]
    pub done: bool,
    #[serde(default)]
    pub response: Option<ExecutionResult>,
    #[serde(default)]
    pub error: Option<ExecutionError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionResult {
    #[serde(default)]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionError {
    #[serde(default)]
    pub error_message: Option<String>,
    #[serde(default)]
    pub error_type: Option<String>,
    #[serde(default)]
    pub script_stack_trace_elements: Vec<ScriptStackTraceElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptStackTraceElement {
    #[serde(default)]
    pub function: Option<String>,
    #[serde(default)]
    pub line_number: Option<i64>,
}
