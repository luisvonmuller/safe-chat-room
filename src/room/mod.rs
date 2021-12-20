#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Room {
    // Result
    client: isize, // id
    clerk: isize,  // id
    room_id: u64,  // room_id: &'a mut OsRng, // Room Id (Temporary)
}

pub mod create_room;
