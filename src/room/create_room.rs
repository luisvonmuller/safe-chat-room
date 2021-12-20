use super::Room;

pub fn init(client: isize, clerk: isize) -> Room {
    use rand::rngs::OsRng;
    use rand::RngCore;
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);

    Room {
        client,
        clerk,
        room_id: OsRng.next_u64(), //  room_id: &mut rnd_gen, // maybe its a buffer?
    }
}
