use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AppConfig {
    pub id: i32,
    pub contract: String,
    pub url: String,
    pub uuid: String,
    pub name: String,
    pub billable: Option<bool>,
    pub ignore_files: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAppConfig {
    pub contract: String,
    pub url: String,
    pub uuid: String,
    pub name: String,
    pub billable: Option<bool>,
    pub ignore_files: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAppConfig {
    pub contract: Option<String>,
    pub url: Option<String>,
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub billable: Option<bool>,
    pub ignore_files: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AppConfigQuery {
    // Pagination
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
    
    // Filters
    pub contract: Option<String>,
    pub url: Option<String>,
    #[serde(default)]
    pub url_match: MatchType,
    pub uuid: Option<String>,
    #[serde(default)]
    pub uuid_match: MatchType,
    pub app_name: Option<String>,
    #[serde(default)]
    pub app_name_match: MatchType,
    pub billable: Option<BillableFilter>,
    pub ignore_files: Option<bool>,
    
    // Sorting
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_order: SortOrder,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MatchType {
    #[default]
    Exact,
    Fuzzy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BillableFilter {
    All,
    Review,  // NULL values
    True,
    False,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    #[default]
    Asc,
    Desc,
}

#[derive(Debug, Serialize)]
pub struct AppConfigListResponse {
    pub data: Vec<AppConfig>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateRequest {
    pub ids: Vec<i32>,
    pub update: UpdateAppConfig,
}
