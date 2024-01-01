use std::{collections::HashMap, io, path::PathBuf, sync::Mutex, time::SystemTime};

use super::{
    errors::{ApiError, ErrorCode},
    stat::Stats,
};
use lazy_static::lazy_static;

pub struct ReadData {
    pub bytes_read: u64,
    pub buffer: Vec<u8>,
}

lazy_static! {
    static ref FLAG_CACHE: Mutex<HashMap<String, FileFlag>> = Mutex::new(HashMap::new());
}
#[derive(Debug, PartialEq, Eq)]
pub enum ActionType {
    // Indicates that the code should not do anything.
    NOP,
    // Indicates that the code should throw an exception.
    THROW_EXCEPTION,
    // Indicates that the code should truncate the file, but only if it is a file.
    TRUNCATE_FILE,
    // Indicates that the code should create the file.
    CREATE_FILE,
}

/// Represents one of the following file flags. A convenience object.
///
/// - `'r'` - Open file for reading. An exception occurs if the file does not exist.
/// - `'r+'` - Open file for reading and writing. An exception occurs if the file does not exist.
/// - `'rs'` - Open file for reading in synchronous mode. Instructs the filesystem to not cache writes.
/// - `'rs+'` - Open file for reading and writing, and opens the file in synchronous mode.
/// - `'w'` - Open file for writing. The file is created (if it does not exist) or truncated (if it exists).
/// - `'wx'` - Like 'w' but opens the file in exclusive mode.
/// - `'w+'` - Open file for reading and writing. The file is created (if it does not exist) or truncated (if it exists).
/// - `'wx+'` - Like 'w+' but opens the file in exclusive mode.
/// - `'a'` - Open file for appending. The file is created if it does not exist.
/// - `'ax'` - Like 'a' but opens the file in exclusive mode.
/// - `'a+'` - Open file for reading and appending. The file is created if it does not exist.
/// - `'ax+'` - Like 'a+' but opens the file in exclusive mode.
///
/// Exclusive mode ensures that the file path is newly created.
///

#[derive(Clone)]
pub struct FileFlag {
    flag_str: String,
}

impl FileFlag {
    /// This should never be called directly.
    ///
    /// * `flag_str`: The string representing the mode
    ///
    /// # Panics
    ///
    /// This function will return ApiError when the mode string is invalid.

    pub fn new(flag_str: String) -> Result<Self, ApiError> {
        let valid_flag_str = vec![
            "r", "r+", "rs", "rs+", "w", "wx", "w+", "wx+", "a", "ax", "a+", "ax+",
        ];
        if !valid_flag_str.contains(&flag_str.as_str()) {
            return Err(ApiError::new(
                &ErrorCode::EINVAL,
                format!("Invalid flag: {}", flag_str),
                None,
                None,
            ));
        }

        Ok(Self { flag_str })
    }

    /// Get an object representing the given file flag.
    ///
    /// * `flag_str`: The string representing the flag
    ///
    /// Returns the `FileFlag` object representing the flag.
    ///
    /// # Panics
    ///
    /// This function will return `ApiError` when the flag string is invalid.
    pub fn get_file_flag(flag_str: String) -> Result<Self, ApiError> {
        let mut flag_cache = FLAG_CACHE.lock().unwrap();
        // Check cache first
        if !flag_cache.contains_key(&flag_str) {
            let file_flag = FileFlag::new(flag_str.clone())?;
            flag_cache.insert(flag_str.clone(), file_flag);
        }
        Ok(flag_cache.get(&flag_str).unwrap().clone())
    }

    pub fn get_flag_str(&self) -> String {
        self.flag_str.clone()
    }

    /// Returns true if the file is readable.
    pub fn is_readable(&self) -> bool {
        self.flag_str.contains('r') || self.flag_str.contains('+')
    }
    /// Returns true if the file is writable.
    pub fn is_writeable(&self) -> bool {
        self.flag_str.contains('w') || self.flag_str.contains('a') || self.flag_str.contains('+')
    }
    /// Returns true if the file is executable.
    pub fn is_executable(&self) -> bool {
        self.flag_str.contains('x')
    }

    /// Returns true if the file mode should truncate.
    pub fn is_truncating(&self) -> bool {
        self.flag_str.contains('w')
    }

    /// Returns true if the file is appendable.
    pub fn is_appendable(&self) -> bool {
        self.flag_str.contains('a')
    }

    /// Returns true if the file is open in synchronous mode.
    pub fn is_synchronous(&self) -> bool {
        self.flag_str.contains('s')
    }

    /// Returns true if the file is open in exclusive mode.
    pub fn is_exclusive(&self) -> bool {
        self.flag_str.contains('x')
    }

    /// Returns one of the static fields on this object that indicates the
    /// appropriate response to the path existing.
    pub fn path_exists_action(&self) -> ActionType {
        if self.is_exclusive() {
            ActionType::THROW_EXCEPTION
        } else if self.is_truncating() {
            ActionType::TRUNCATE_FILE
        } else {
            ActionType::NOP
        }
    }

    /// Returns one of the static fields on this object that indicates the
    /// appropriate response to the path not existing.
    pub fn path_not_exists_action(&self) -> ActionType {
        if (self.is_writeable() || self.is_appendable()) && self.flag_str != "r+" {
            ActionType::CREATE_FILE
        } else {
            ActionType::THROW_EXCEPTION
        }
    }
    /// Get the equivalent mode (0b0xxx: read, write, execute)
    /// Note: Execute will always be 0
    pub fn get_mode(&self) -> u32 {
        let mut mode = 0;
        mode |= self.is_readable() as u32;
        mode |= (self.is_writeable() as u32) << 1;
        mode |= (self.is_executable() as u32) << 2;
        mode
    }
}

pub trait File {
    fn get_pos(&self) -> Result<u64, ApiError>;
    fn stat(&self) -> Result<Stats, ApiError>;
    fn stat_sync(&self) -> Result<Stats, ApiError>;
    fn close(&self) -> Result<(), ApiError>;
    fn close_sync(&self) -> Result<(), ApiError>;
    fn truncate(&mut self, len: u64) -> Result<(), ApiError>;
    fn truncate_sync(&mut self, len: u64) -> Result<(), ApiError>;
    fn sync(&self) -> Result<(), ApiError>;
    fn sync_sync(&self) -> Result<(), ApiError>;
    fn write(
        &mut self,
        buffer: &[u8],
        offset: usize,
        length: usize,
        position: Option<u64>,
    ) -> Result<usize, ApiError>;
    fn write_sync(
        &mut self,
        buffer: &[u8],
        offset: usize,
        length: usize,
        position: Option<u64>,
    ) -> Result<usize, ApiError>;
    fn read(
        &mut self,
        buffer: &mut [u8],
        offset: usize,
        length: usize,
        position: Option<u64>,
    ) -> Result<ReadData, ApiError>;
    fn read_sync(
        &mut self,
        buffer: &mut [u8],
        offset: usize,
        length: usize,
        position: Option<u64>,
    ) -> Result<usize, ApiError>;
    fn datasync(&self) -> Result<(), ApiError>;
    fn datasync_sync(&self) -> Result<(), ApiError>;
    fn chown(&mut self, uid: u32, gid: u32) -> Result<(), ApiError>;
    fn chown_sync(&mut self, uid: u32, gid: u32) -> Result<(), ApiError>;
    fn chmod(&mut self, mode: u32) -> Result<(), ApiError>;
    fn chmod_sync(&mut self, mode: u32) -> Result<(), ApiError>;
    fn utimes(&self, atime: SystemTime, mtime: SystemTime) -> Result<(), ApiError>;
    fn utimes_sync(&self, atime: SystemTime, mtime: SystemTime) -> Result<(), ApiError>;
}

pub struct BaseFile;
impl BaseFile {
    pub fn sync(&self) -> Result<(), ApiError> {
        let path = PathBuf::from("/");
        Err(ApiError::enoent(path))
    }

    pub fn sync_sync(&self) -> Result<(), ApiError> {
        let path = PathBuf::from("/");
        Err(ApiError::enoent(path))
    }

    pub fn datasync(&self) -> Result<(), ApiError> {
        self.sync()
    }

    pub fn datasync_sync(&self) -> Result<(), ApiError> {
        self.sync_sync()
    }

    pub fn chown(&self, _uid: u32, _gid: u32) -> Result<(), ApiError> {
        let path = PathBuf::from("/");
        Err(ApiError::enoent(path))
    }

    pub fn chown_sync(&self, _uid: u32, _gid: u32) -> Result<(), ApiError> {
        let path = PathBuf::from("/");
        Err(ApiError::enoent(path))
    }

    pub fn chmod(&self, _mode: u32) -> Result<(), ApiError> {
        let path = PathBuf::from("/");
        Err(ApiError::enoent(path))
    }

    pub fn chmod_sync(&self, _mode: u32) -> Result<(), ApiError> {
        let path = PathBuf::from("/");
        Err(ApiError::enoent(path))
    }

    pub fn utimes(&self, _atime: SystemTime, _mtime: SystemTime) -> Result<(), ApiError> {
        let path = PathBuf::from("/");
        Err(ApiError::enoent(path))
    }

    pub fn utimes_sync(&self, _atime: SystemTime, _mtime: SystemTime) -> Result<(), ApiError> {
        let path = PathBuf::from("/");
        Err(ApiError::enoent(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn test_file_flag_creation() {
        // Test valid flags
        let valid_flags = vec![
            "r", "r+", "rs", "rs+", "w", "wx", "w+", "wx+", "a", "ax", "a+", "ax+",
        ];
        for flag in valid_flags {
            assert!(FileFlag::new(flag.to_string()).is_ok());
        }

        // Test invalid flag
        let invalid_flag = "invalid_flag";
        assert!(FileFlag::new(invalid_flag.to_string()).is_err());
    }

    #[test]
    fn test_file_flag_methods() {
        let file_flag = FileFlag::new("r+".to_string()).unwrap();

        assert_eq!(file_flag.is_readable(), true);
        assert_eq!(file_flag.is_writeable(), true);
        assert_eq!(file_flag.is_truncating(), false);
        assert_eq!(file_flag.is_appendable(), false);
        assert_eq!(file_flag.is_synchronous(), false);
        assert_eq!(file_flag.is_exclusive(), false);
        assert_eq!(file_flag.path_exists_action(), ActionType::NOP);
        assert_eq!(
            file_flag.path_not_exists_action(),
            ActionType::THROW_EXCEPTION
        );
        assert_eq!(file_flag.get_mode(), 0b011);
    }

    #[test]
    fn test_file_flag_cache() {
        let _ = FileFlag::get_file_flag("r".to_string());
        {
            let flag_cache = FLAG_CACHE.lock().unwrap();
            assert!(flag_cache.contains_key("r"));
        }
    }

    #[test]
    fn test_base_file_sync() {
        let base_file = BaseFile;

        // Test async sync
        assert!(base_file.sync().is_err());

        // Test sync sync
        assert!(base_file.sync_sync().is_err());
    }

    #[test]
    fn test_base_file_datasync() {
        let base_file = BaseFile;

        // Test async datasync
        assert!(base_file.datasync().is_err());

        // Test sync datasync
        assert!(base_file.datasync_sync().is_err());
    }

    #[test]
    fn test_base_file_chown() {
        let base_file = BaseFile;

        // Test async chown
        assert!(base_file.chown(0, 0).is_err());

        // Test sync chown
        assert!(base_file.chown_sync(0, 0).is_err());
    }

    #[test]
    fn test_base_file_chmod() {
        let base_file = BaseFile;

        // Test async chmod
        assert!(base_file.chmod(0).is_err());

        // Test sync chmod
        assert!(base_file.chmod_sync(0).is_err());
    }

    #[test]
    fn test_base_file_utimes() {
        let base_file = BaseFile;
        let now = UNIX_EPOCH + Duration::from_secs(1234567890);

        // Test async utimes
        assert!(base_file.utimes(now, now).is_err());

        // Test sync utimes
        assert!(base_file.utimes_sync(now, now).is_err());
    }
}
