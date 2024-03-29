use crate::{GameObject, Transform, Time, Debug};

/// # DataStruct
///
/// This struct can be thought to be similar to a monobehaviour
/// in unity, and stores the objects transform and gameobject info.
#[repr(C)]
pub struct DataStruct{
    pub transform: Transform,
    pub game_object: GameObject,
    pub time: Time,
    pub debug: Debug,
}