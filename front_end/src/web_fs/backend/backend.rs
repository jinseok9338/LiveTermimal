use std::collections::HashMap;

use crate::web_fs::file_system::FileSystem;

// Trait for file system options
pub struct InstanceType;
pub type BFSCallback<T> = fn(T);
pub trait BackendOption<T> {
    fn get_type(&self) -> Vec<String>;
    fn is_optional(&self) -> bool;
    fn get_description(&self) -> String;
    fn validate(&self, opt: T) -> Result<(), Box<dyn std::error::Error>>;
}

pub type BackendOptions = HashMap<String, Box<dyn BackendOption<()>>>;

pub trait BaseBackendConstructor<FS: FileSystem> {
    // A name to identify the backend.
    fn name(&self) -> &str;

    // Describes all of the options available for this backend.
    fn options(&self) -> BackendOptions;

    // Whether the backend is available in the current environment.
    // It supports checking synchronously and asynchronously.
    // Sync: Returns 'true' if this backend is available in the current
    // environment. For example, a `localStorage`-backed filesystem will return
    // 'false' if the browser does not support that API.
    // Defaults to 'false', as the FileSystem base class isn't usable alone.
    fn is_available(&self) -> bool;
    fn create_instance(&self) -> Result<Box<FS>, Box<dyn std::error::Error>>;
    fn create_with_options(
        &self,
        options: HashMap<String, Box<dyn BackendOption<()>>>,
    ) -> Result<Box<FS>, Box<dyn std::error::Error>>;
    fn create_with_callback(
        &self,
        callback: BFSCallback<Box<FS>>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn create_with_options_and_callback(
        &self,
        options: HashMap<String, Box<dyn BackendOption<()>>>,
        callback: BFSCallback<Box<FS>>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
