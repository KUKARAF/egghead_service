use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Userscript {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub prompt: String,
    pub url: String,
    pub script_code: String,
    pub violentmonkey_metadata: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct UserscriptWithDevices {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub url: String,
    pub script_code: String,
    pub violentmonkey_metadata: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub device_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DeviceAssignment {
    pub device_id: String,
    pub device_name: String,
}
