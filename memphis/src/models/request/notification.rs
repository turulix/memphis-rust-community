use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NotificationRequest<'a> {
    pub title: &'a str,
    pub msg: &'a str,
    #[serde(rename = "type")]
    pub msg_type: &'a str,
    pub code: &'a str,
}
