use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(default)]
    pub resource_name: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub metadata: Option<PersonMetadata>,
    #[serde(default)]
    pub names: Vec<Name>,
    #[serde(default)]
    pub nicknames: Vec<Nickname>,
    #[serde(default)]
    pub email_addresses: Vec<EmailAddress>,
    #[serde(default)]
    pub phone_numbers: Vec<PhoneNumber>,
    #[serde(default)]
    pub addresses: Vec<Address>,
    #[serde(default)]
    pub organizations: Vec<Organization>,
    #[serde(default)]
    pub birthdays: Vec<Birthday>,
    #[serde(default)]
    pub urls: Vec<Url>,
    #[serde(default)]
    pub photos: Vec<Photo>,
    #[serde(default)]
    pub biographies: Vec<Biography>,
    #[serde(default)]
    pub relations: Vec<Relation>,
    #[serde(default)]
    pub events: Vec<PersonEvent>,
    #[serde(default)]
    pub memberships: Vec<Membership>,
    #[serde(default)]
    pub im_clients: Vec<ImClient>,
    #[serde(default)]
    pub user_defined: Vec<UserDefined>,
    #[serde(default)]
    pub occupations: Vec<Occupation>,
    #[serde(default)]
    pub genders: Vec<Gender>,
    #[serde(default)]
    pub locations: Vec<Location>,
    #[serde(default)]
    pub sip_addresses: Vec<SipAddress>,
    #[serde(default)]
    pub external_ids: Vec<ExternalId>,
    #[serde(default)]
    pub file_ases: Vec<FileAs>,
    #[serde(default)]
    pub misc_keywords: Vec<MiscKeyword>,
    #[serde(default)]
    pub client_data: Vec<ClientData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonMetadata {
    #[serde(default)]
    pub sources: Vec<Source>,
    #[serde(default)]
    pub previous_resource_names: Vec<String>,
    #[serde(default)]
    pub linked_people_resource_names: Vec<String>,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "type", default)]
    pub source_type: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default)]
    pub profile_metadata: Option<ProfileMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileMetadata {
    #[serde(default)]
    pub object_type: Option<String>,
    #[serde(default)]
    pub user_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldMetadata {
    #[serde(default)]
    pub primary: bool,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub source: Option<Source>,
    #[serde(default)]
    pub source_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub display_name_last_first: Option<String>,
    #[serde(default)]
    pub unstructured_name: Option<String>,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    #[serde(default)]
    pub middle_name: Option<String>,
    #[serde(default)]
    pub honorific_prefix: Option<String>,
    #[serde(default)]
    pub honorific_suffix: Option<String>,
    #[serde(default)]
    pub phonetic_full_name: Option<String>,
    #[serde(default)]
    pub phonetic_family_name: Option<String>,
    #[serde(default)]
    pub phonetic_given_name: Option<String>,
    #[serde(default)]
    pub phonetic_middle_name: Option<String>,
    #[serde(default)]
    pub phonetic_honorific_prefix: Option<String>,
    #[serde(default)]
    pub phonetic_honorific_suffix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nickname {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type", default)]
    pub nickname_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailAddress {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type", default)]
    pub email_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneNumber {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub canonical_form: Option<String>,
    #[serde(rename = "type", default)]
    pub phone_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub formatted_value: Option<String>,
    #[serde(rename = "type", default)]
    pub address_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
    #[serde(default)]
    pub po_box: Option<String>,
    #[serde(default)]
    pub street_address: Option<String>,
    #[serde(default)]
    pub extended_address: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(rename = "type", default)]
    pub org_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub phonetic_name: Option<String>,
    #[serde(default)]
    pub department: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub job_description: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub start_date: Option<DateValue>,
    #[serde(default)]
    pub end_date: Option<DateValue>,
    #[serde(default)]
    pub current: bool,
    #[serde(default)]
    pub cost_center: Option<String>,
    #[serde(default)]
    pub full_time_equivalent_millipercent: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Birthday {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub date: Option<DateValue>,
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateValue {
    #[serde(default)]
    pub year: Option<i64>,
    #[serde(default)]
    pub month: Option<i64>,
    #[serde(default)]
    pub day: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type", default)]
    pub url_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Biography {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub person: Option<String>,
    #[serde(rename = "type", default)]
    pub relation_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonEvent {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub date: Option<DateValue>,
    #[serde(rename = "type", default)]
    pub event_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub contact_group_membership: Option<ContactGroupMembership>,
    #[serde(default)]
    pub domain_membership: Option<DomainMembership>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactGroupMembership {
    #[serde(default)]
    pub contact_group_id: Option<String>,
    #[serde(default)]
    pub contact_group_resource_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainMembership {
    #[serde(default)]
    pub in_viewer_domain: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImClient {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(rename = "type", default)]
    pub im_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub formatted_protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDefined {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Occupation {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gender {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub formatted_value: Option<String>,
    #[serde(default)]
    pub address_me_as: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type", default)]
    pub location_type: Option<String>,
    #[serde(default)]
    pub current: bool,
    #[serde(default)]
    pub building_id: Option<String>,
    #[serde(default)]
    pub floor: Option<String>,
    #[serde(default)]
    pub floor_section: Option<String>,
    #[serde(default)]
    pub desk_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SipAddress {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type", default)]
    pub sip_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalId {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type", default)]
    pub id_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileAs {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiscKeyword {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type", default)]
    pub keyword_type: Option<String>,
    #[serde(default)]
    pub formatted_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientData {
    #[serde(default)]
    pub metadata: Option<FieldMetadata>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactGroup {
    #[serde(default)]
    pub resource_name: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub metadata: Option<ContactGroupMetadata>,
    #[serde(default)]
    pub group_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub formatted_name: Option<String>,
    #[serde(default)]
    pub member_resource_names: Vec<String>,
    #[serde(default)]
    pub member_count: Option<i64>,
    #[serde(default)]
    pub client_data: Vec<ClientData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactGroupMetadata {
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default)]
    pub deleted: bool,
}
