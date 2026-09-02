use serde::{Serialize, Deserialize};
use strum_macros::Display;
use thiserror::Error;
use reqwest::StatusCode;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("File I/O error: {0}")]
    File(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("FHIR resource requires a 'resourceType' field")]
    MissingResourceType,
    #[error("Validation failed with status {0}")]
    ValidationFailed(StatusCode),
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Display)]
pub enum Severity {
    #[default]
    #[serde(rename = "information")]
    Information,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "fatal")]
    Fatal,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub severity: Severity,
    pub code: String,
    pub diagnostics: String,
    pub details: Option<Details>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Details {
    pub text: String,
}
