use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub group_key: Option<EntityKey>,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default)]
    pub labels: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityKey {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub preferred_member_key: Option<EntityKey>,
    #[serde(rename = "type", default)]
    pub member_type: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default)]
    pub roles: Vec<MemberRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberRole {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub expiry_detail: Option<ExpiryDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpiryDetail {
    #[serde(default)]
    pub expire_time: Option<String>,
}
