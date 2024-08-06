use crate::constants::error::ErrorCode;
pub struct FileCheckResponse {
    pub status: bool,
    pub code: ErrorCode,
}
