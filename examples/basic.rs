use runity::{DataStruct, Quaternion, String, Vector3};

/* We now define some functions
   These are core functions that if not included,
   will not be run.
*/
fn main(){

}

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
    let time = data.time; // Extract time related things.

    let tag = String::from("Player"); // Get the tag for "Player". Will panic if it fails to allocate.

    let player_obj = data.game_object.get_gameobject_from_tag(tag); // Get the game object associated with the tag

    let pos_to_go_towards = player_obj.transform.position; // get the position of the player


    data.transform.position = Vector3::lerp(data.transform.position, pos_to_go_towards, 0.0015 * time.delta_time); // move towards the player using lerp, scaling to the timestep

    data.transform.rotation = data.transform.rotation * Quaternion::from_euler(5.0 * time.delta_time, 5.0 * time.delta_time, 5.0 * time.delta_time);

    data
}