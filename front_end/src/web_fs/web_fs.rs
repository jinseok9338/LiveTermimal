use std::{path::PathBuf, time::SystemTime};

use serde::Deserialize;
use wasm_bindgen::JsValue;
use web_sys::window;

use crate::utils::commands::commands_context_hook::CONFIG;

#[derive(Deserialize, Debug)]
pub struct Group {
    pub name: String,
}

impl Group {
    pub fn new(name: String) -> Group {
        Group { name }
    }
}

impl Default for Group {
    fn default() -> Self {
        Group::new(CONFIG.group.to_owned())
    }
}
#[derive(Deserialize, Debug)]
pub struct Owener {
    pub name: String,
}

impl Owener {
    pub fn new(name: String) -> Owener {
        Owener { name }
    }
}

impl Default for Owener {
    fn default() -> Self {
        Owener::new(CONFIG.name.to_owned())
    }
}

#[derive(Deserialize, Debug)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub append: bool,
}
#[derive(Deserialize, Debug)]
pub enum FileType {
    Txt,
    Json,
    Png,
    Directory,
}
#[derive(Deserialize, Debug)]
pub struct WebFile {
    name: String,
    path: PathBuf,
    permissions: Permissions,
    created: SystemTime,
    modified: SystemTime,
    file_type: FileType,
    owner: Owener,
    group: Group,
    is_directory: bool,
}

impl WebFile {
    pub fn new(
        name: String,
        path: String,
        permissions: Permissions,
        file_type: FileType,
        is_directory: bool,
    ) -> WebFile {
        let path = PathBuf::from(path);
        let created = SystemTime::now();
        let modified = SystemTime::now();
        let owner = Owener::default();
        let group = Group::default();

        WebFile {
            name,
            path,
            permissions,
            created,
            modified,
            file_type,
            owner,
            group,
            is_directory,
        }
    }

    pub fn try_exists(&self) -> Result<bool, JsValue> {
        let window = window().expect("should have a window in this context");
        let local_storage = window.local_storage()?.expect("localStorage not enabled");
        let path = WebFile::canonicalize(&self);
        match local_storage.get_item(&path)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    /// Canonicalizes the path of the WebFile.
    ///
    /// This function checks if the path contains any invalid characters or sequences.
    /// Otherwise, it returns the canonicalized path as a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::WebFile;
    ///
    /// let file = WebFile::new("path/to/file".to_string());
    /// assert_eq!(file.canonicalize().unwrap(), "path/to/file");
    ///
    /// let file = WebFile::new("path/../file".to_string());
    /// assert_eq!(file.canonicalize().unwrap(), "file");
    ///
    /// let file = WebFile::new("path/with/./chars".to_string());
    /// assert_eq!(file.canonicalize().unwrap(), "path/with/chars");
    /// ```
    ///
    pub fn canonicalize(&self) -> String {
        // this will only /foo/bar/../baz -> /foo/baz
        let components = Self::normalize_path(self.path.clone());
        format!("{}", components.join("/")) // Reassemble the path
    }

    fn normalize_path(p: PathBuf) -> Vec<String> {
        let p = p.display().to_string();
        let mut components: Vec<String> = Vec::new();
        for component in p.split("/") {
            match component {
                "" | "." => {} // Ignore empty components and "."
                ".." => {
                    components.pop();
                } // Remove the last component for ".."
                _ => components.push(component.to_string()), // Add the component to the path
            }
        }
        components
    }

    fn can_read(&self) -> bool {
        self.permissions.read
    }

    //Read the entire contents of a file into a bytes vector.
    pub fn read(&self) -> Result<Vec<u8>, String> {
        todo!()
    }

    pub fn read_to_string(&self) -> Result<String, String> {
        todo!()
    }

    pub fn copy(from: PathBuf, to: PathBuf) -> Result<u64, String> {
        // first of all check if the file exists from the from path

        // if not throw error
        // check if the file in the from path has read permission
        // if not throw error

        // find the file in the to path
        // if the file path exists in the to path check if it has write permission
        // if not throw error
        // if the file doesn't exists then create one with read and write permission

        // copy the file content of the from the from path to the to path. and overwrite it.
        todo!()
    }
}

impl From<String> for WebFile {
    fn from(s: String) -> Self {
        let web_file: WebFile = serde_json::from_str(&s).unwrap();
        web_file
    }
}

impl ToString for WebFile {
    fn to_string(&self) -> String {
        format!(
            "WebFile {{ name: {}, path: {}, permissions: {:?}, created: {:?}, modified: {:?}, file_type: {:?}, owner: {:?}, group: {:?}, is_directory: {:?} }}",
            self.name,self.path.to_string_lossy(), self.permissions, self.created, self.modified, self.file_type, self.owner, self.group, self.is_directory
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize_valid_path() {
        let path = "/foo/bar/../baz".to_owned();
        let result = "/foo/baz".to_owned();
        let path = PathBuf::from(path);
        dbg!(&path);
        assert_eq!(
            format!("/{}", WebFile::normalize_path(path).join("/")),
            result
        );
    }

    #[test]
    fn test_canonicalize_path2() {
        // test /goo/bar/./../baz
        let path = "/goo/bar/./../baz".to_owned();
        let result = "/goo/baz".to_owned();
        let path = PathBuf::from(path);
        dbg!(&path);
        assert_eq!(
            format!("/{}", WebFile::normalize_path(path).join("/")),
            result
        )
    }
}
