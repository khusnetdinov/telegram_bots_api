use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#writeaccessallowed
/// This object represents a service message about a user allowing a bot to write messages after adding it to the attachment menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method requestWriteAccess.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WriteAccessAllowed {
    from_request: Option<bool>,
    web_app_name: Option<String>,
    from_attachment_menu: Option<bool>,
}
