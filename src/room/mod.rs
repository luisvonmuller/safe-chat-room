use crate::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Room {
    // Result
    pub client: isize, // id
    pub clerk: isize,  // id
    pub id: u64,       // room_id:  Room Id (Temporary)
}

pub mod create;

pub async fn handler(path: String, rooms: Arc<Mutex<Option<Vec<Room>>>>) -> Result<String, ()> {
    Ok("Hello there".to_string())
}
