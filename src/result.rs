
pub enum SteamGridDbResult<T> {
    Success(T),
    Error {
        errors: Option<Vec<String>>,
        status: Option<u32>,
    },
}
impl<T> SteamGridDbResult<T> {
    pub fn is_error(&self) -> bool {
        match self {
            SteamGridDbResult::Success(_) => false,
            SteamGridDbResult::Error { .. } => true,
        }
    }
}
