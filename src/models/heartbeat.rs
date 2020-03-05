#[derive(Debug, Serialize)]
pub struct HeartbeatResponse {
    pub status: u16,
    pub message: String,
    pub version: String,
}
