use crate::room::Room;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rooms(Arc<Mutex<Option<Vec<Room>>>>);
