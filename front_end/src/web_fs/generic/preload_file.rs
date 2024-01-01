use std::{path::PathBuf, time::SystemTime};

use crate::web_fs::{
    emulation::shared::get_mount,
    errors::ApiError,
    file::{File, FileFlag, ReadData},
    file_system::FileSystem,
    stat::Stats,
    utils::system_time_to_ms,
};

pub struct PreloadFile {
    fs: Box<dyn FileSystem>,
    pos: u64,
    path: PathBuf,
    stat: Stats,
    flag: FileFlag,
    buffer: Vec<u8>,
    dirty: bool,
}
impl PreloadFile {
    pub fn new(
        fs: Box<dyn FileSystem>,
        path: PathBuf,
        stat: Stats,
        flag: FileFlag,
        contents: Option<Vec<u8>>,
    ) -> Self {
        let buffer = contents.unwrap_or_else(Vec::new);
        if buffer.len() != stat.size as usize && flag.is_readable() {
            panic!("Invalid buffer: Buffer is {} long, yet Stats object specifies that file is {} long.", buffer.len(), stat.size);
        }
        Self {
            fs,
            pos: 0,
            path,
            stat,
            flag,
            buffer,
            dirty: false,
        }
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn get_stats(&self) -> &Stats {
        &self.stat
    }

    pub fn get_flags(&self) -> &FileFlag {
        &self.flag
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn advanced_pos(&mut self, delta: u64) -> u64 {
        let pos = self.pos.clone();
        self.pos = pos + delta;
        self.pos
    }

    pub fn set_pos(&mut self, new_pos: u64) {
        self.pos = new_pos
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn reset_dirty(&mut self) {
        self.dirty = false
    }
}

impl File for PreloadFile {
    fn get_pos(&self) -> Result<u64, ApiError> {
        if self.flag.is_appendable() {
            return Ok(self.stat.size);
        }

        Ok(self.pos)
    }

    fn stat(&self) -> Result<crate::web_fs::stat::Stats, ApiError> {
        Ok(self.stat.clone())
    }

    fn stat_sync(&self) -> Result<crate::web_fs::stat::Stats, ApiError> {
        Ok(self.stat.clone())
    }

    fn close(&self) -> Result<(), ApiError> {
        self.close_sync()
    }

    fn close_sync(&self) -> Result<(), ApiError> {
        let path = PathBuf::from("");
        Err(ApiError::enotsup(path))
    }

    fn truncate(&mut self, len: u64) -> Result<(), ApiError> {
        self.truncate_sync(len)?;
        let mount = get_mount("/")?;
        if !mount.metadata().synchronous && self.flag.is_synchronous() {
            self.sync()?
        }
        let path_buf = self.path.clone();
        Err(ApiError::enotsup(path_buf))
    }

    fn truncate_sync(&mut self, len: u64) -> Result<(), ApiError> {
        self.dirty = true; // Assuming _dirty is a field in YourFile
        if !self.flag.is_writeable() {
            let path_buf = self.path.clone();
            return Err(ApiError::enotsup(path_buf));
        }

        self.stat.mtime_ms = system_time_to_ms(SystemTime::now()); // Assuming stat is a struct with a mtime field
        if len > self.buffer.len() as u64 {
            let buf = vec![0; (len - self.buffer.len() as u64) as usize];
            // Write will set @_stat.size for us.
            self.write_sync(&buf, 0, buf.len(), Some(self.buffer.len() as u64))?;

            if self.flag.is_synchronous() && get_mount("/")?.metadata().synchronous {
                self.sync_sync()?;
            }

            return Ok(());
        }

        self.stat.size = len;
        // Truncate buffer to 'len'.
        let mut new_buf = vec![0; len as usize];
        new_buf[..len as usize].copy_from_slice(&self.buffer);
        self.buffer = new_buf;

        if self.flag.is_synchronous() && get_mount("/")?.metadata().synchronous {
            self.sync_sync()?;
        }

        Ok(())
    }

    fn sync(&self) -> Result<(), ApiError> {
        self.sync_sync()
    }

    fn sync_sync(&self) -> Result<(), ApiError> {
        let path = PathBuf::from("");
        Err(ApiError::enotsup(path))
    }

    /// Write buffer to the file.
    ///
    /// Note that it is unsafe to use `fs.write` multiple times on the same file
    /// without waiting for the callback.
    ///
    /// # Arguments
    ///
    /// * `buffer`: Buffer containing the data to write to the file.
    /// * `offset`: Offset in the buffer to start reading data from.
    /// * `length`: The amount of bytes to write to the file.
    /// * `position`: Offset from the beginning of the file where this data should be written.
    ///               If `position` is `None`, the data will be written at the current position.
    ///
    /// # Returns
    ///
    /// The result is a `Result` containing the number of bytes written into the file on success,
    /// or an `ApiError` on failure.
    ///
    fn write(
        &mut self,
        buffer: &[u8],
        offset: usize,
        length: usize,
        position: Option<u64>,
    ) -> Result<usize, ApiError> {
        self.write_sync(buffer, offset, length, position)
    }

    /// Write buffer to the file synchronously.
    ///
    /// Note that it is unsafe to use `fs.writeSync` multiple times on the same file
    /// without waiting for the callback.
    ///
    /// # Arguments
    ///
    /// * `buffer`: Buffer containing the data to write to the file.
    /// * `offset`: Offset in the buffer to start reading data from.
    /// * `length`: The amount of bytes to write to the file.
    /// * `position`: Offset from the beginning of the file where this data should be written.
    ///               If `position` is `None`, the data will be written at the current position.
    ///
    /// # Returns
    ///
    /// The result is the number of bytes written into the file.
    fn write_sync(
        &mut self,
        buffer: &[u8],
        offset: usize,
        length: usize,
        position: Option<u64>,
    ) -> Result<usize, ApiError> {
        self.dirty = true;

        let position = position.unwrap_or_else(|| self.get_pos().unwrap());

        if !self.flag.is_writeable() {
            let path_buf = self.path.clone();
            return Err(ApiError::enotsup(path_buf));
        }

        let end_fp = position + length as u64;

        if end_fp > self.stat.size {
            self.stat.size = end_fp;

            if end_fp > self.buffer.len() as u64 {
                // Extend the buffer!
                let mut new_buff = vec![0; end_fp as usize];
                new_buff.copy_from_slice(&self.buffer);
                self.buffer = new_buff;
            }
        }

        let len = {
            let path_buf = self.path.clone();
            let target_slice = &mut self.buffer[position as usize..];
            let source_slice = &buffer[offset..offset + length];

            // Ensure the target slice has enough capacity
            if target_slice.len() < source_slice.len() {
                return Err(ApiError::enotsup(path_buf)); // or handle the error accordingly
            }

            target_slice[..source_slice.len()].copy_from_slice(source_slice);
            source_slice.len()
        };

        self.stat.mtime_ms = system_time_to_ms(SystemTime::now()); // Assuming stat is a struct with a mtime field

        if self.flag.is_synchronous() {
            self.sync_sync()?;
        }

        self.set_pos(position + len as u64);
        Ok(len)
    }

    fn read(
        &mut self,
        buffer: &mut [u8],
        offset: usize,
        length: usize,
        position: Option<u64>,
    ) -> Result<ReadData, ApiError> {
        let result = self.read_sync(buffer, offset, length, position)?;

        Ok(ReadData {
            bytes_read: result as u64,
            buffer: buffer.to_vec(),
        })
    }

    fn read_sync(
        &mut self,
        buffer: &mut [u8],
        offset: usize,
        mut length: usize,
        position: Option<u64>,
    ) -> Result<usize, ApiError> {
        if self.flag.is_readable() {}
        let position = position.unwrap_or_else(|| self.get_pos().unwrap());
        let end_read = position + length as u64;
        if end_read > self.stat.size {
            length = (self.stat.size - position) as usize;
        }
        let rv = {
            let src_slice = &self.buffer[position as usize..(position as usize + length) as usize];
            let dest_slice = &mut buffer[offset..offset + length];

            dest_slice.copy_from_slice(src_slice);
            dest_slice.len()
        };

        self.stat.atime_ms = system_time_to_ms(SystemTime::now());
        self.pos = position + length as u64;
        Ok(rv)
    }

    fn datasync(&self) -> Result<(), ApiError> {
        self.sync()
    }

    fn datasync_sync(&self) -> Result<(), ApiError> {
        self.sync_sync()
    }

    fn chown(&mut self, uid: u32, gid: u32) -> Result<(), ApiError> {
        self.chown_sync(uid, gid)
    }

    fn chown_sync(&mut self, uid: u32, gid: u32) -> Result<(), ApiError> {
        if !self.fs.metadata().supports_properties {
            return Err(ApiError::enotsup(self.path.clone()));
        }
        self.dirty = true;
        self.stat.chown(uid, gid);
        self.sync_sync()?;
        Ok(())
    }

    fn chmod(&mut self, mode: u32) -> Result<(), ApiError> {
        self.chmod_sync(mode)
    }

    fn chmod_sync(&mut self, mode: u32) -> Result<(), ApiError> {
        if !self.fs.metadata().supports_properties {
            return Err(ApiError::enotsup(self.path.clone()));
        }
        self.dirty = true;
        self.stat.chmod(mode);
        self.sync_sync()?;
        Ok(())
    }

    fn utimes(
        &self,
        _atime: std::time::SystemTime,
        _mtime: std::time::SystemTime,
    ) -> Result<(), ApiError> {
        return Err(ApiError::enotsup(self.path.clone()));
    }

    fn utimes_sync(
        &self,
        _atime: std::time::SystemTime,
        _mtime: std::time::SystemTime,
    ) -> Result<(), ApiError> {
        return Err(ApiError::enotsup(self.path.clone()));
    }
}
