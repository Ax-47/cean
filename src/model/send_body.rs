use serde::Deserialize;
#[derive(Deserialize)]
pub struct MessagePayload {
    pub content: String,
    pub auth: String,
    pub channel_id: u64,
}
