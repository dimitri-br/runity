mod vector3;
mod transform;
mod math;
mod data;
mod gameobject;
mod string;


// Define a NULL ptr for our string. This will help us
// prevent allocating an invalid string
pub const NULL: *mut c_char = null::<c_char>() as *mut c_char;

use libc::c_char;
use std::ptr::null;
use std::ffi::CString;

pub use vector3::Vector3;
pub use transform::Transform;
pub use gameobject::GameObject;
pub use math::Math;
pub use data::DataStruct;
pub use string::String;


/* We now define some functions */
#[no_mangle]
pub extern fn awake(mut _data: DataStruct) -> DataStruct{
    _data
}

#[no_mangle]
pub extern fn start(mut data: DataStruct) -> DataStruct{
    data.transform.position = Vector3::translate(data.transform.position, Vector3::new(0.0, 5.0, 0.0));
    data
}

#[no_mangle]
pub extern fn update(mut data: DataStruct) -> DataStruct{
    let pos_to_go_towards =  Vector3::new(0.0, 1000.0, 0.0);
    data.transform.position = Vector3::lerp(data.transform.position, pos_to_go_towards, 0.00015);

    let tag = String::new(CString::new("Player").unwrap()).unwrap();
    data.game_object.get_gameobject_from_tag(tag);
    data
}