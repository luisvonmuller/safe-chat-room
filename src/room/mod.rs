use crate::*;
use axum::extract::Extension;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Room {
    // Result
    pub client: isize, // id
    pub clerk: isize,  // id
    pub id: u64,       // room_id:  Room Id (Temporary)
}

pub mod create;

pub async fn handler(path: &str, rooms: Arc<Mutex<Option<Vec<Room>>>>) -> &'static str {
    "Hello there"
}
