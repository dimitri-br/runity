mod vector3;
mod transform;
mod math;
mod data;
mod gameobject;
mod string;


// Define a NULL ptr for our string. This will help us
// prevent allocating an invalid string
pub const NULL: *mut c_char = null::<c_char>() as *mut c_char;

pub fn free_ptr(mut _ptr: *mut i8){
    _ptr = NULL;
}

use libc::{c_char};
use std::{ffi::CString, ptr::null};

pub use vector3::Vector3;
pub use transform::Transform;
pub use gameobject::GameObject;
pub use math::Math;
pub use data::DataStruct;
pub use string::String;



/* We now define some functions
   These are core functions that if not included,
   will not be run.
*/

// This function is called when the script is first run
#[no_mangle]
pub extern "C" fn awake(mut _data: DataStruct) -> DataStruct{
    _data
}

// This function is called after awake, use it to initialize values and setup the rest of the script
#[no_mangle]
pub extern "C" fn start(mut data: DataStruct) -> DataStruct{
    data.transform.position = Vector3::translate(data.transform.position, Vector3::new(0.0, 5.0, 0.0));
    data
}

// This function is run every frame
#[no_mangle]
pub extern "C" fn update(mut data: DataStruct) -> DataStruct{
    let tag = String::from_str("Player").unwrap(); // Get the tag for "Player"

    let player_obj = data.game_object.get_gameobject_from_tag(tag); // Get the game object associated with the tag

    let pos_to_go_towards = player_obj.transform.position; // get the position of the player

    data.transform.position = Vector3::lerp(data.transform.position, pos_to_go_towards, 0.00015); // move towards the player using lerp

    data
}