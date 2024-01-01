use std::{
    error::Error,
    path::{self, Path, PathBuf},
    time::SystemTime,
};

use futures::Future;
use tokio::sync::oneshot;

use super::{
    cred::Cred,
    errors::{ApiError, ErrorCode},
    file::{ActionType, File, FileFlag},
    global_constants::BufferEncoding,
    stat::Stats,
};

pub type BFSOneArgCallback = dyn Fn(Option<&dyn Error>) -> Result<(), Box<dyn Error>>;
pub type BFSCallback<T> = dyn Fn(Option<&dyn Error>, Option<T>) -> Result<(), Box<dyn Error>>;
pub type BFSThreeArgCallback<T, U> =
    dyn Fn(Option<&dyn Error>, Option<T>, Option<U>) -> Result<(), Box<dyn Error>>;

// Assuming the equivalent of Buffer in Rust is Vec<u8>
pub type FileContents = Vec<u8>;

// Metadata about a FileSystem
pub struct FileSystemMetadata {
    // The name of the FS
    pub name: String,

    // Whether the FS is readonly or not
    pub readonly: bool,

    // Does the FS support synchronous operations
    pub synchronous: bool,

    // Does the FS support properties
    pub supports_properties: bool,

    // Does the FS support links
    pub supports_links: bool,

    // The total space
    pub total_space: u64,

    // The available space
    pub free_space: u64,
}

pub trait FileSystem: Send + Sync {
    fn name(&self) -> &str;
    fn metadata(&self) -> FileSystemMetadata;
    fn when_ready(&self) -> Box<dyn Future<Output = Result<Box<dyn FileSystem>, ApiError>>>;
    fn access(
        &self,
        p: &str,
        mode: u32,
        cred: &Cred,
    ) -> Box<dyn Future<Output = Result<(), ApiError>>>;
    fn access_sync(&self, p: &str, mode: u32, cred: &Cred) -> Result<(), ApiError>;
    fn rename(&self, old_path: &str, new_path: &str, cred: &Cred) -> Result<(), ApiError>;
    fn rename_sync(&self, old_path: &str, new_path: &str, cred: &Cred) -> Result<(), ApiError>;
    fn stat_sync(&self, p: &str, cred: &Cred) -> Result<Stats, ApiError>;
    /// Asynchronous file open.
    ///
    /// * `p` - Path to the file.
    /// * `flag` - Flags handling the complexity of various file modes.
    /// * `mode` - Mode to use to open the file. Can be ignored if the filesystem doesn't support permissions.
    /// * `cred` - Credentials for the operation.
    fn open(
        &self,
        p: String,
        flag: FileFlag,
        mode: u32,
        cred: Cred,
    ) -> Box<dyn Future<Output = Result<Box<dyn File>, ApiError>>>;

    /// Synchronous file open.
    ///
    /// * `p` - Path to the file.
    /// * `flag` - Flags handling the complexity of various file modes.
    /// * `mode` - Mode to use to open the file. Can be ignored if the filesystem doesn't support permissions.
    /// * `cred` - Credentials for the operation.
    fn open_sync(
        &self,
        p: &str,
        flag: FileFlag,
        mode: u32,
        cred: &Cred,
    ) -> Result<Box<dyn File>, ApiError>;

    /// Asynchronous `unlink`.
    ///
    /// * `p` - Path to the file to unlink.
    /// * `cred` - Credentials for the operation.
    fn unlink(&self, p: &str, cred: &Cred) -> Result<(), ApiError>;

    /// Synchronous `unlink`.
    ///
    /// * `p` - Path to the file to unlink.
    /// * `cred` - Credentials for the operation.
    fn unlink_sync(&self, p: &str, cred: &Cred) -> Result<(), ApiError>;

    /// Asynchronous `rmdir`.
    ///
    /// * `p` - Path to the directory to remove.
    /// * `cred` - Credentials for the operation.
    fn rmdir(&self, p: &str, cred: &Cred) -> Result<(), ApiError>;
    /// Synchronous `rmdir`.
    ///
    /// * `p` - Path to the directory to remove.
    /// * `cred` - Credentials for the operation.
    fn rmdir_sync(&self, p: &str, cred: &Cred) -> Result<(), ApiError>;

    /// Asynchronous `mkdir`.
    ///
    /// * `p` - Path to the directory to create.
    /// * `mode` - Mode to make the directory using. Can be ignored if the filesystem doesn't support permissions.
    /// * `cred` - Credentials for the operation.
    fn mkdir(&self, p: &str, mode: u32, cred: &Cred) -> Result<(), ApiError>;

    /// Synchronous `mkdir`.
    ///
    /// * `p` - Path to the directory to create.
    /// * `mode` - Mode to make the directory using. Can be ignored if the filesystem doesn't support permissions.
    /// * `cred` - Credentials for the operation.
    fn mkdir_sync(&self, p: &str, mode: u32, cred: &Cred) -> Result<(), ApiError>;

    /// Asynchronous `readdir`. Reads the contents of a directory.
    ///
    /// * `p` - Path to the directory.
    /// * `cred` - Credentials for the operation.
    fn readdir(&self, p: &str, cred: &Cred) -> Result<Vec<String>, ApiError>;

    /// Synchronous `readdir`. Reads the contents of a directory.
    ///
    /// * `p` - Path to the directory.
    /// * `cred` - Credentials for the operation.
    fn readdir_sync(&self, p: &str, cred: &Cred) -> Result<Vec<String>, ApiError>;

    // **SUPPLEMENTAL INTERFACE METHODS**
    // File or directory operations
    /// Test whether or not the given path exists by checking with the file system.
    ///
    /// * `p` - Path to check for existence.
    /// * `cred` - Credentials for the operation.
    fn exists(&self, p: &str, cred: &Cred) -> Result<bool, ApiError>;

    /// Test whether or not the given path exists by checking with the file system.
    ///
    /// * `p` - Path to check for existence.
    /// * `cred` - Credentials for the operation.
    fn exists_sync(&self, p: &str, cred: &Cred) -> Result<bool, ApiError>;

    /// Asynchronous `realpath`.
    ///
    /// * `p` - Path to resolve.
    /// * `cred` - Credentials for the operation.
    fn realpath(&self, p: &str, cred: &Cred) -> Result<String, ApiError>;

    /// Synchronous `realpath`.
    ///
    /// * `p` - Path to resolve.
    /// * `cred` - Credentials for the operation.
    fn realpath_sync(&self, p: &str, cred: &Cred) -> Result<String, ApiError>;
    /// Asynchronous `truncate`.
    ///
    /// * `p` - Path to the file to truncate.
    /// * `len` - New size of the file.
    /// * `cred` - Credentials for the operation.
    fn truncate(&self, p: &str, len: u64, cred: &Cred) -> Result<(), ApiError>;

    /// Synchronous `truncate`.
    ///
    /// * `p` - Path to the file to truncate.
    /// * `len` - New size of the file.
    /// * `cred` - Credentials for the operation.
    fn truncate_sync(&self, p: &str, len: u64, cred: &Cred) -> Result<(), ApiError>;

    /// Asynchronously reads the entire contents of a file.
    ///
    /// * `fname` - Path to the file to read.
    /// * `encoding` - If non-null, the file's contents should be decoded into a string using that encoding.
    /// * `flag` - FileFlag specifying how to open the file.
    /// * `cred` - Credentials for the operation.
    fn read_file(
        &self,
        fname: &str,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        cred: &Cred,
    ) -> Result<FileContents, ApiError>;

    /// Synchronously reads the entire contents of a file.
    ///
    /// * `fname` - Path to the file to read.
    /// * `encoding` - If non-null, the file's contents should be decoded into a string using that encoding.
    /// * `flag` - FileFlag specifying how to open the file.
    /// * `cred` - Credentials for the operation.
    fn read_file_sync(
        &self,
        fname: &str,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        cred: &Cred,
    ) -> Result<FileContents, ApiError>;

    /// Asynchronously writes data to a file, replacing the file if it already exists.
    ///
    /// * `fname` - Path to the file to write.
    /// * `data` - Data to write to the file.
    /// * `encoding` - If non-null, the encoding to use for writing the data.
    /// * `flag` - FileFlag specifying how to open the file.
    /// * `mode` - Mode to use to open the file. Can be ignored if the filesystem doesn't support permissions.
    /// * `cred` - Credentials for the operation.
    fn write_file(
        &self,
        fname: &str,
        data: FileContents,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        mode: u32,
        cred: &Cred,
    ) -> Result<(), ApiError>;

    /// Synchronously writes data to a file, replacing the file if it already exists.
    ///
    /// * `fname` - Path to the file to write.
    /// * `data` - Data to write to the file.
    /// * `encoding` - If non-null, the encoding to use for writing the data.
    /// * `flag` - FileFlag specifying how to open the file.
    /// * `mode` - Mode to use to open the file. Can be ignored if the filesystem doesn't support permissions.
    /// * `cred` - Credentials for the operation.
    fn write_file_sync(
        &self,
        fname: &str,
        data: FileContents,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        mode: u32,
        cred: &Cred,
    ) -> Result<(), ApiError>;

    /// Asynchronously appends data to a file, creating the file if it does not yet exist.
    ///
    /// * `fname` - Path to the file to append to.
    /// * `data` - Data to append to the file.
    /// * `encoding` - If non-null, the encoding to use for appending the data.
    /// * `flag` - FileFlag specifying how to open the file.
    /// * `mode` - Mode to use to open the file. Can be ignored if the filesystem doesn't support permissions.
    /// * `cred` - Credentials for the operation.
    fn append_file(
        &self,
        fname: &str,
        data: FileContents,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        mode: u32,
        cred: &Cred,
    ) -> Result<(), ApiError>;
    fn append_file_sync(
        &self,
        fname: &str,
        data: FileContents,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        mode: u32,
        cred: Cred,
    ) -> Result<(), ApiError>;
    // **OPTIONAL INTERFACE METHODS*
    // Property operations
    // This isn't always possible on some filesystem types (e.g. Dropbox).

    // Asynchronous `chmod`.
    fn chmod(&self, p: &str, mode: u32, cred: Cred) -> Result<(), ApiError>;

    // Synchronous `chmod`.
    fn chmod_sync(&self, p: &str, mode: u32, cred: Cred) -> Result<(), ApiError>;

    // Asynchronous `chown`.
    fn chown(&self, p: &str, new_uid: u32, new_gid: u32, cred: Cred) -> Result<(), ApiError>;

    // Synchronous `chown`.
    fn chown_sync(&self, p: &str, new_uid: u32, new_gid: u32, cred: Cred) -> Result<(), ApiError>;

    // Change file timestamps of the file referenced by the supplied path.
    fn utimes(
        &self,
        p: &str,
        atime: SystemTime,
        mtime: SystemTime,
        cred: Cred,
    ) -> Result<(), ApiError>;

    // Change file timestamps of the file referenced by the supplied path.
    fn utimes_sync(
        &self,
        p: &str,
        atime: SystemTime,
        mtime: SystemTime,
        cred: Cred,
    ) -> Result<(), ApiError>;

    // Symlink operations
    // Symlinks aren't always supported.

    // Asynchronous `link`.
    fn link(&self, srcpath: &str, dstpath: &str, cred: Cred) -> Result<(), ApiError>;

    // Synchronous `link`.
    fn link_sync(&self, srcpath: &str, dstpath: &str, cred: Cred) -> Result<(), ApiError>;

    // Asynchronous `symlink`.
    fn symlink(
        &self,
        srcpath: &str,
        dstpath: &str,
        file_type: &str,
        cred: Cred,
    ) -> Result<(), ApiError>;

    // Synchronous `symlink`.
    fn symlink_sync(
        &self,
        srcpath: &str,
        dstpath: &str,
        file_type: &str,
        cred: Cred,
    ) -> Result<(), ApiError>;

    // Asynchronous readlink.
    fn readlink(&self, p: &str, cred: Cred) -> Result<String, ApiError>;

    // Synchronous readlink.
    fn readlink_sync(&self, p: &str, cred: Cred) -> Result<String, ApiError>;
}

/**
 * Opens the file at path p with the given flag. The file must exist.
 * @param p The path to open.
 * @param flag The flag to use when opening the file.
 */
pub fn open_file(_p: String, _flag: FileFlag, _cred: Cred) -> Result<Box<dyn File>, ApiError> {
    let path = PathBuf::from("");
    Err(ApiError::enotsup(path))
}

pub fn stat(p: PathBuf, cred: Cred) -> Result<Stats, ApiError> {
    todo!()
}

fn create_file(
    _p: String,
    _flag: FileFlag,
    _mode: u32,
    _cred: Cred,
) -> Result<Box<dyn File>, ApiError> {
    let path = PathBuf::from("");
    Err(ApiError::enotsup(path))
}

pub struct BaseFileSystem {
    name: String,
}
impl BaseFileSystem {}
impl FileSystem for BaseFileSystem {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn metadata(&self) -> FileSystemMetadata {
        // Your logic to obtain metadata
        FileSystemMetadata {
            name: String::from("BaseFileSystem"),
            readonly: false,
            synchronous: false,
            supports_properties: false,
            supports_links: false,
            total_space: 0,
            free_space: 0,
        }
    }

    fn when_ready(&self) -> Box<dyn Future<Output = Result<Box<dyn FileSystem>, ApiError>>> {
        let name = self.name.clone();
        let future = async move { Ok(Box::new(BaseFileSystem { name }) as Box<dyn FileSystem>) };
        Box::new(future)
    }

    fn access(
        &self,
        _p: &str,
        _mode: u32,
        _cred: &Cred,
    ) -> Box<dyn Future<Output = Result<(), ApiError>>> {
        let path = PathBuf::from("");
        Box::new(async move { Err(ApiError::enotsup(path)) })
    }

    fn access_sync(&self, _p: &str, _mode: u32, _cred: &Cred) -> Result<(), ApiError> {
        let path = PathBuf::from("");
        Err(ApiError::enotsup(path))
    }

    fn rename(&self, _old_path: &str, _new_path: &str, _cred: &Cred) -> Result<(), ApiError> {
        let path = PathBuf::from("");
        Err(ApiError::enotsup(path))
    }

    fn rename_sync(&self, old_path: &str, new_path: &str, cred: &Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn stat_sync(&self, p: &str, cred: &Cred) -> Result<Stats, ApiError> {
        todo!()
    }

    fn open(
        &self,
        p: String,
        flag: FileFlag,
        mode: u32,
        cred: Cred,
    ) -> Box<dyn Future<Output = Result<Box<dyn File>, ApiError>>> {
        let action = flag.path_exists_action();
        match action {
            ActionType::NOP => Box::new(async move { open_file(p, flag, cred.clone()) }),
            ActionType::TRUNCATE_FILE => Box::new(async move {
                let mut file = open_file(p, flag, cred.clone())?;
                file.truncate(0).expect("Failed to truncate file");
                file.sync().expect("Failed to sync file");
                Ok(file)
            }),
            ActionType::CREATE_FILE => {
                let path_buf = PathBuf::from("");
                Box::new(async move { Err(ApiError::eexist(path_buf.clone())) })
            }
            ActionType::THROW_EXCEPTION => {
                let path_not_exists_action = flag.path_not_exists_action();
                match path_not_exists_action {
                    ActionType::NOP | ActionType::THROW_EXCEPTION | ActionType::TRUNCATE_FILE => {
                        let path_buf = PathBuf::from("");
                        Box::new(async move { Err(ApiError::enoent(path_buf.clone())) })
                    }
                    ActionType::CREATE_FILE => Box::new(async move {
                        let cred = cred.clone();
                        let cred_clone = cred.clone();
                        let path_buf = PathBuf::from("");
                        let path = Path::new(p.as_str());
                        let dir = path
                            .parent()
                            .ok_or_else(|| ApiError::enoent(path_buf.clone()))?;
                        let path_buf = PathBuf::from("");
                        let dir_buf = dir.to_path_buf();

                        let parent_stat = stat(dir_buf, cred)?;
                        if !parent_stat.is_directory() {
                            return Err(ApiError::enoent(path_buf.clone()));
                        }
                        let file = create_file(p, flag, mode, cred_clone);
                        let file = file.unwrap();
                        return Ok(file);
                    }),
                }
            }
        }
    }

    fn open_sync(
        &self,
        p: &str,
        flag: FileFlag,
        mode: u32,
        cred: &Cred,
    ) -> Result<Box<dyn File>, ApiError> {
        todo!()
    }

    fn unlink(&self, p: &str, cred: &Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn unlink_sync(&self, p: &str, cred: &Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn rmdir(&self, p: &str, cred: &Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn rmdir_sync(&self, p: &str, cred: &Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn mkdir(&self, p: &str, mode: u32, cred: &Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn mkdir_sync(&self, p: &str, mode: u32, cred: &Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn readdir(&self, p: &str, cred: &Cred) -> Result<Vec<String>, ApiError> {
        todo!()
    }

    fn readdir_sync(&self, p: &str, cred: &Cred) -> Result<Vec<String>, ApiError> {
        todo!()
    }

    fn exists(&self, p: &str, cred: &Cred) -> Result<bool, ApiError> {
        todo!()
    }

    fn exists_sync(&self, p: &str, cred: &Cred) -> Result<bool, ApiError> {
        todo!()
    }

    fn realpath(&self, p: &str, cred: &Cred) -> Result<String, ApiError> {
        todo!()
    }

    fn realpath_sync(&self, p: &str, cred: &Cred) -> Result<String, ApiError> {
        todo!()
    }

    fn truncate(&self, p: &str, len: u64, cred: &Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn truncate_sync(&self, p: &str, len: u64, cred: &Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn read_file(
        &self,
        fname: &str,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        cred: &Cred,
    ) -> Result<FileContents, ApiError> {
        todo!()
    }

    fn read_file_sync(
        &self,
        fname: &str,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        cred: &Cred,
    ) -> Result<FileContents, ApiError> {
        todo!()
    }

    fn write_file(
        &self,
        fname: &str,
        data: FileContents,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        mode: u32,
        cred: &Cred,
    ) -> Result<(), ApiError> {
        todo!()
    }

    fn write_file_sync(
        &self,
        fname: &str,
        data: FileContents,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        mode: u32,
        cred: &Cred,
    ) -> Result<(), ApiError> {
        todo!()
    }

    fn append_file(
        &self,
        fname: &str,
        data: FileContents,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        mode: u32,
        cred: &Cred,
    ) -> Result<(), ApiError> {
        todo!()
    }

    fn append_file_sync(
        &self,
        fname: &str,
        data: FileContents,
        encoding: Option<BufferEncoding>,
        flag: FileFlag,
        mode: u32,
        cred: Cred,
    ) -> Result<(), ApiError> {
        todo!()
    }

    fn chmod(&self, p: &str, mode: u32, cred: Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn chmod_sync(&self, p: &str, mode: u32, cred: Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn chown(&self, p: &str, new_uid: u32, new_gid: u32, cred: Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn chown_sync(&self, p: &str, new_uid: u32, new_gid: u32, cred: Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn utimes(
        &self,
        p: &str,
        atime: SystemTime,
        mtime: SystemTime,
        cred: Cred,
    ) -> Result<(), ApiError> {
        todo!()
    }

    fn utimes_sync(
        &self,
        p: &str,
        atime: SystemTime,
        mtime: SystemTime,
        cred: Cred,
    ) -> Result<(), ApiError> {
        todo!()
    }

    fn link(&self, srcpath: &str, dstpath: &str, cred: Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn link_sync(&self, srcpath: &str, dstpath: &str, cred: Cred) -> Result<(), ApiError> {
        todo!()
    }

    fn symlink(
        &self,
        srcpath: &str,
        dstpath: &str,
        file_type: &str,
        cred: Cred,
    ) -> Result<(), ApiError> {
        todo!()
    }

    fn symlink_sync(
        &self,
        srcpath: &str,
        dstpath: &str,
        file_type: &str,
        cred: Cred,
    ) -> Result<(), ApiError> {
        todo!()
    }

    fn readlink(&self, p: &str, cred: Cred) -> Result<String, ApiError> {
        todo!()
    }

    fn readlink_sync(&self, p: &str, cred: Cred) -> Result<String, ApiError> {
        todo!()
    }
}
