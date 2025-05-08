use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartVpnRequest {
    pub address: Option<String>,
    pub dns_server: Option<String>,
    pub routes: Option<Vec<String>>,
    pub http_proxy_host: Option<String>,
    pub http_proxy_port: Option<u16>,
    pub allowed_applications: Option<Vec<String>>,
    pub disallowed_applications: Option<Vec<String>>,
    pub mtu: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub app_name: String,
    pub package_name: String,
    pub app_icon: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppListResponse {
    pub value: Vec<AppInfo>,
}
