use crate::{GameObject, Transform};

/// # DataStruct
///
/// This struct holds all sorts of data to be used by modders,
/// like function pointers and more to interface with unity with.
#[repr(C)]
pub struct DataStruct{
    pub transform: Transform,
    pub game_object: GameObject
}