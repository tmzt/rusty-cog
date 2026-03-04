use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub starred: bool,
    #[serde(default)]
    pub trashed: bool,
    #[serde(default)]
    pub explicitly_trashed: bool,
    #[serde(default)]
    pub parents: Vec<String>,
    #[serde(default)]
    pub properties: Option<serde_json::Value>,
    #[serde(default)]
    pub app_properties: Option<serde_json::Value>,
    #[serde(default)]
    pub spaces: Vec<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub web_content_link: Option<String>,
    #[serde(default)]
    pub web_view_link: Option<String>,
    #[serde(default)]
    pub icon_link: Option<String>,
    #[serde(default)]
    pub thumbnail_link: Option<String>,
    #[serde(default)]
    pub viewed_by_me: bool,
    #[serde(default)]
    pub viewed_by_me_time: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub modified_time: Option<String>,
    #[serde(default)]
    pub modified_by_me_time: Option<String>,
    #[serde(default)]
    pub shared_with_me_time: Option<String>,
    #[serde(default)]
    pub sharing_user: Option<FileUser>,
    #[serde(default)]
    pub owners: Vec<FileUser>,
    #[serde(default)]
    pub last_modifying_user: Option<FileUser>,
    #[serde(default)]
    pub shared: bool,
    #[serde(default)]
    pub owned_by_me: bool,
    #[serde(default)]
    pub writers_can_share: bool,
    #[serde(default)]
    pub permissions: Vec<Permission>,
    #[serde(default)]
    pub folder_color_rgb: Option<String>,
    #[serde(default)]
    pub original_filename: Option<String>,
    #[serde(default)]
    pub full_file_extension: Option<String>,
    #[serde(default)]
    pub file_extension: Option<String>,
    #[serde(default)]
    pub md5_checksum: Option<String>,
    #[serde(default)]
    pub sha1_checksum: Option<String>,
    #[serde(default)]
    pub sha256_checksum: Option<String>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub quota_bytes_used: Option<String>,
    #[serde(default)]
    pub head_revision_id: Option<String>,
    #[serde(default)]
    pub content_hints: Option<ContentHints>,
    #[serde(default)]
    pub image_media_metadata: Option<ImageMediaMetadata>,
    #[serde(default)]
    pub video_media_metadata: Option<VideoMediaMetadata>,
    #[serde(default)]
    pub capabilities: Option<serde_json::Value>,
    #[serde(default)]
    pub is_app_authorized: bool,
    #[serde(default)]
    pub drive_id: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUser {
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub me: bool,
    #[serde(default)]
    pub permission_id: Option<String>,
    #[serde(default)]
    pub email_address: Option<String>,
    #[serde(default)]
    pub photo_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHints {
    #[serde(default)]
    pub thumbnail: Option<Thumbnail>,
    #[serde(default)]
    pub indexable_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageMediaMetadata {
    #[serde(default)]
    pub width: Option<i64>,
    #[serde(default)]
    pub height: Option<i64>,
    #[serde(default)]
    pub rotation: Option<i64>,
    #[serde(default)]
    pub camera_make: Option<String>,
    #[serde(default)]
    pub camera_model: Option<String>,
    #[serde(default)]
    pub exposure_time: Option<f64>,
    #[serde(default)]
    pub aperture: Option<f64>,
    #[serde(default)]
    pub focal_length: Option<f64>,
    #[serde(default)]
    pub iso_speed: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMediaMetadata {
    #[serde(default)]
    pub width: Option<i64>,
    #[serde(default)]
    pub height: Option<i64>,
    #[serde(default)]
    pub duration_millis: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileList {
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub incomplete_search: bool,
    #[serde(default)]
    pub files: Vec<File>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(rename = "type", default)]
    pub permission_type: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub email_address: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub photo_link: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub allow_file_discovery: bool,
    #[serde(default)]
    pub pending_owner: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedDrive {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub color_rgb: Option<String>,
    #[serde(default)]
    pub background_image_link: Option<String>,
    #[serde(default)]
    pub theme_id: Option<String>,
    #[serde(default)]
    pub capabilities: Option<serde_json::Value>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub restrictions: Option<SharedDriveRestrictions>,
    #[serde(default)]
    pub org_unit_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedDriveRestrictions {
    #[serde(default)]
    pub admin_managed_restrictions: bool,
    #[serde(default)]
    pub copy_requires_writer_permission: bool,
    #[serde(default)]
    pub domain_users_only: bool,
    #[serde(default)]
    pub drive_members_only: bool,
    #[serde(default)]
    pub sharing_folders_requires_organizer_permission: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct About {
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub user: Option<FileUser>,
    #[serde(default)]
    pub storage_quota: Option<StorageQuota>,
    #[serde(default)]
    pub import_formats: Option<serde_json::Value>,
    #[serde(default)]
    pub export_formats: Option<serde_json::Value>,
    #[serde(default)]
    pub max_import_sizes: Option<serde_json::Value>,
    #[serde(default)]
    pub max_upload_size: Option<String>,
    #[serde(default)]
    pub app_installed: bool,
    #[serde(default)]
    pub folder_color_palette: Vec<String>,
    #[serde(default)]
    pub can_create_drives: bool,
    #[serde(default)]
    pub can_create_team_drives: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageQuota {
    #[serde(default)]
    pub limit: Option<String>,
    #[serde(default)]
    pub usage: Option<String>,
    #[serde(default)]
    pub usage_in_drive: Option<String>,
    #[serde(default)]
    pub usage_in_drive_trash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(rename = "type", default)]
    pub change_type: Option<String>,
    #[serde(default)]
    pub time: Option<String>,
    #[serde(default)]
    pub removed: bool,
    #[serde(default)]
    pub file_id: Option<String>,
    #[serde(default)]
    pub file: Option<File>,
    #[serde(default)]
    pub drive_id: Option<String>,
    #[serde(default)]
    pub drive: Option<SharedDrive>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parents: Vec<String>,
    #[serde(default)]
    pub starred: Option<bool>,
    #[serde(default)]
    pub properties: Option<serde_json::Value>,
    #[serde(default)]
    pub app_properties: Option<serde_json::Value>,
    #[serde(default)]
    pub folder_color_rgb: Option<String>,
    #[serde(default)]
    pub content_hints: Option<ContentHints>,
    #[serde(default)]
    pub writers_can_share: Option<bool>,
    #[serde(default)]
    pub copy_requires_writer_permission: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportFormat {
    pub source: String,
    #[serde(default)]
    pub targets: Vec<String>,
}
