use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonProfile {
    #[serde(default)]
    pub resource_name: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub names: Vec<PersonName>,
    #[serde(default)]
    pub email_addresses: Vec<PersonEmail>,
    #[serde(default)]
    pub phone_numbers: Vec<PersonPhone>,
    #[serde(default)]
    pub photos: Vec<PersonPhoto>,
    #[serde(default)]
    pub organizations: Vec<PersonOrganization>,
    #[serde(default)]
    pub relations: Vec<PersonRelation>,
    #[serde(default)]
    pub addresses: Vec<PersonAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonName {
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    #[serde(default)]
    pub middle_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonEmail {
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type", default)]
    pub email_type: Option<String>,
    #[serde(default)]
    pub form_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonPhone {
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type", default)]
    pub phone_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonPhoto {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonOrganization {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub department: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonRelation {
    #[serde(default)]
    pub person: Option<String>,
    #[serde(rename = "type", default)]
    pub relation_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonAddress {
    #[serde(default)]
    pub formatted_value: Option<String>,
    #[serde(rename = "type", default)]
    pub address_type: Option<String>,
    #[serde(default)]
    pub street_address: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub postal_code: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
}
