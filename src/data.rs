use crate::{gameobject::GameObjectChanges, Debug, GameObject, Time};

use std::sync::Mutex;
use lazy_static::lazy_static;

/// # DataStruct
///
/// This struct stores all the data we need to run our game.
/// 
/// It contains structs that point to timing and debugging information.
#[repr(C)]
pub struct DataStruct{
    pub time: Time,
    pub debug: Debug,
}

// A local store of all data present in unity (that we can interact with)
// 
// We then get, and send any state changes to the unity engine. We prefer
// rust changes over unity changes, as rust is more likely to be the source
// of truth.
pub struct LocalData{
    gameobjects: Vec<GameObject>, // All gameobjects in the scene
    changes: Vec<GameObjectChanges>, // All changes to gameobjects - to be sent to unity
    data: Option<DataStruct>, // A reference to all data in the game like time and debug information
}

lazy_static!{
    pub static ref LOCAL_DATA: Mutex<LocalData> = Mutex::new(LocalData{
        gameobjects: Vec::new(),
        changes: Vec::new(),
        data: None,
    });
}

// Safe extern wrappers to set the data
#[no_mangle]
pub extern "C" fn set_data(data: DataStruct){
    let mut local_data = LOCAL_DATA.lock().unwrap();
    local_data.data = Some(data);
}

// Data is a one-way street - we only get data from unity
// However, for changes, we need to send them back to unity

// Send changes to unity - return a reference to the changes
#[no_mangle]
pub extern "C" fn send_changes() -> Vec<GameObjectChanges>{
    let mut local_data = LOCAL_DATA.lock().unwrap();
    let changes = local_data.changes.clone();
    local_data.changes.clear();
    changes
}