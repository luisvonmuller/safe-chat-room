use super::Room;

/** TODO:
 * Check if theres any room with this id... lal (Probably not but things like to fail on my life tho)
 *
 */
pub fn new(client: isize, clerk: isize) -> Room {
    use rand::rngs::OsRng;
    use rand::RngCore;
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);

    Room {
        client,
        clerk,
        id: OsRng.next_u64(), //  room_id: &mut rnd_gen, // maybe its a buffer?
    }
}
