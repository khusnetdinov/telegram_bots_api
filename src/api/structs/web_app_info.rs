use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#webappinfo>
/// Describes a Web App.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebAppInfo {
    pub url: String,
}
