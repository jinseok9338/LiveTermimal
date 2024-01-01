//export const mounts: Map<string, FileSystem> = new Map();
// export function getMount(mountPoint: string): FileSystem {
// 	return mounts.get(mountPoint);
// }

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::web_fs::{errors::ApiError, file_system::FileSystem};
use lazy_static::lazy_static;

lazy_static! {
    static ref MOUNT: Arc<Mutex<HashMap<&'static str, Arc<dyn FileSystem>>>> = {
        let map = HashMap::new();
        Arc::new(Mutex::new(map))
    };
}

pub fn get_mount(mount_point: &str) -> Result<Arc<dyn FileSystem>, ApiError> {
    // Acquire the lock on the map
    let path_buf = PathBuf::from(mount_point);
    let map = MOUNT.lock().map_err(|_| ApiError::enotsup(path_buf))?;

    // Try to get the value from the map
    if let Some(value) = map.get(mount_point) {
        // If found, clone the value and return it
        Ok(value.clone())
    } else {
        // If not found, return an error
        let path_buf = PathBuf::from(mount_point);
        Err(ApiError::enotsup(path_buf))
    }
}
