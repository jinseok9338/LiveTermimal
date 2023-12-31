use std::time::{Duration, SystemTime, UNIX_EPOCH};

use byteorder::{LittleEndian, WriteBytesExt};

use super::{
    cred::Cred,
    emulation::constants::{S_IFDIR, S_IFLNK, S_IFMT, S_IFREG},
};

/**
 * Indicates the type of the given file. Applied to 'mode'.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    FILE,
    DIRECTORY,
    SYMLINK,
}

/// Implementation of Node's `Stats`.
///
/// Attribute descriptions are from `man 2 stat`
/// See: http://nodejs.org/api/fs.html#fs_class_fs_stats
/// See: http://man7.org/linux/man-pages/man2/stat.2.html
pub struct Stats {
    // Number of blocks
    pub blocks: u64,

    // File mode
    pub mode: u32,

    // ID of device containing file
    pub dev: u64,

    // Inode number
    pub ino: u64,

    // Device ID (if special file)
    pub rdev: u64,

    // Number of hard links
    pub nlink: u64,

    // Blocksize for file system I/O
    pub blksize: u64,

    // User ID of owner
    pub uid: u32,

    // Group ID of owner
    pub gid: u32,

    // Some file systems stash data on stats objects.
    pub file_data: Vec<u8>,

    // Access time in milliseconds
    pub atime_ms: u64,

    // Modification time in milliseconds
    pub mtime_ms: u64,

    // Change time in milliseconds
    pub ctime_ms: u64,

    // Birth time in milliseconds
    pub birthtime_ms: u64,

    // Size of the file
    pub size: u64,
}

impl Stats {
    /// Provides information about a particular entry in the file system.
    ///
    /// * `item_type`: Type of the item (FILE, DIRECTORY, SYMLINK, or SOCKET)
    /// * `size`: Size of the item in bytes. For directories/symlinks,
    ///   this is normally the size of the struct that represents the item.
    /// * `mode`: Unix-style file mode (e.g. 0o644)
    /// * `atime_ms`: time of last access, in milliseconds since epoch
    /// * `mtime_ms`: time of last modification, in milliseconds since epoch
    /// * `ctime_ms`: time of last time file status was changed, in milliseconds since epoch
    /// * `uid`: the id of the user that owns the file
    /// * `gid`: the id of the group that owns the file
    /// * `birthtime_ms`: time of file creation, in milliseconds since epoch

    pub fn new(
        item_type: FileType,
        size: u64,
        mode: Option<u32>,
        atime_ms: Option<u64>,
        mtime_ms: Option<u64>,
        ctime_ms: Option<u64>,
        birthtime_ms: Option<u64>,
        uid: Option<u32>,
        gid: Option<u32>,
    ) -> Self {
        let current_time: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let atime_ms = atime_ms.unwrap_or(current_time);
        let mtime_ms = mtime_ms.unwrap_or(current_time);
        let ctime_ms = ctime_ms.unwrap_or(current_time);
        let birthtime_ms = birthtime_ms.unwrap_or(current_time);
        let uid = uid.unwrap_or(0);
        let gid = gid.unwrap_or(0);

        let mut mode = Self::get_mode(Some(item_type), mode);
        let block = size / 512;
        let blocks = if size % 512 == 0 { block } else { block + 1 };

        if (mode & S_IFMT) == 0 {
            mode |= match item_type {
                FileType::FILE => S_IFREG as u32,
                FileType::DIRECTORY => S_IFDIR as u32,
                FileType::SYMLINK => S_IFLNK as u32,
            };
        }

        Self {
            blocks,
            mode,
            dev: 0,
            ino: 0,
            rdev: 0,
            nlink: 1,
            blksize: 4096,
            uid,
            gid,
            file_data: vec![],
            atime_ms,
            mtime_ms,
            ctime_ms,
            birthtime_ms,
            size,
        }
    }

    fn get_mode(item_type: Option<FileType>, mode: Option<u32>) -> u32 {
        match mode {
            Some(m) => m,
            None => {
                let base_mode = match item_type.unwrap_or(FileType::DIRECTORY) {
                    FileType::FILE => 0o644,
                    FileType::DIRECTORY => 0o777,
                    FileType::SYMLINK => 0o777, // Adjust this as needed
                };

                // Set the file type bits
                base_mode
                    | match item_type.unwrap_or(FileType::DIRECTORY) {
                        FileType::FILE => S_IFREG as u32,
                        FileType::DIRECTORY => S_IFDIR as u32,
                        FileType::SYMLINK => S_IFLNK as u32,
                    }
            }
        }
    }

    pub fn atime(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_millis(self.atime_ms)
    }

    pub fn mtime(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_millis(self.mtime_ms)
    }

    // Method to get ctime as SystemTime
    pub fn ctime(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_millis(self.ctime_ms)
    }

    // Method to get birthtime as SystemTime
    pub fn birthtime(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_millis(self.birthtime_ms)
    }
    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(32);
        buffer.write_u32::<LittleEndian>(self.size as u32).unwrap();
        buffer.write_u32::<LittleEndian>(self.mode).unwrap();
        let atime_ms = self.atime().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64;
        buffer.write_f64::<LittleEndian>(atime_ms).unwrap();
        let mtime_ms = self.mtime().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64;
        buffer.write_f64::<LittleEndian>(mtime_ms).unwrap();
        let ctime_ms = self.ctime().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64;
        buffer.write_f64::<LittleEndian>(ctime_ms).unwrap();
        buffer.write_u32::<LittleEndian>(self.uid).unwrap();
        buffer.write_u32::<LittleEndian>(self.gid).unwrap();

        buffer
    }

    pub fn is_file(&self) -> bool {
        (self.mode & S_IFMT) == S_IFREG
    }

    pub fn is_directory(&self) -> bool {
        (self.mode & S_IFMT) == S_IFDIR
    }

    pub fn is_symbolic_link(&self) -> bool {
        (self.mode & S_IFMT) == S_IFLNK
    }

    /// Checks if a given user/group has access to this item
    ///
    /// * `mode`: The request access as 4 bits (unused, read, write, execute)
    /// * `uid`: The requesting UID
    /// * `gid`: The requesting GID
    ///
    /// Returns `true` if the request has access, `false` if the request does not

    pub fn has_access(&self, mode: u32, cred: Cred) -> bool {
        if cred.euid == 0 || cred.egid == 0 {
            //Running as root
            return true;
        }
        let perms = self.mode & !S_IFMT;
        let mut u_mode = 0xf;
        let mut g_mode = 0xf;
        let mut w_mode = 0xf;
        if cred.euid == self.uid {
            let u_perms = (0xf00 & perms) >> 8;
            u_mode = (mode ^ u_perms) & mode;
        }

        if cred.egid == self.gid {
            let g_perms = (0xf0 & perms) >> 4;
            g_mode = (mode ^ g_perms) & mode;
        }

        let w_perms = 0xf & perms;
        w_mode = (mode ^ w_perms) & mode;
        let result = u_mode & g_mode & w_mode;
        return result != 0;
    }

    pub fn get_cred(&self, uid: Option<u32>, gid: Option<u32>) -> Cred {
        let actual_uid = uid.unwrap_or(self.uid);
        let actual_gid = gid.unwrap_or(self.gid);

        Cred {
            uid: actual_uid,
            gid: actual_gid,
            euid: self.uid,
            egid: self.gid,
            suid: actual_uid,
            sgid: actual_gid,
        }
    }

    pub fn chmod(&mut self, mode: u32) {
        // Preserve the file type bits and update the permission bits
        self.mode = (self.mode & S_IFMT) | mode;
    }

    pub fn chown(&mut self, uid: u32, gid: u32) {
        if uid <= u32::MAX {
            self.uid = uid;
        }
        if gid <= u32::MAX {
            self.gid = gid;
        }
    }

    // not supported yet
    pub fn is_socket(&self) -> bool {
        //(self.mode & S_IFMT) == S_IFSOCK
        false
    }

    pub fn is_block_device(&self) -> bool {
        // (self.mode & S_IFMT) == S_IFBLK
        false
    }

    pub fn is_character_device(&self) -> bool {
        // (self.mode & S_IFMT) == S_IFCHR
        false
    }

    pub fn is_fifo(&self) -> bool {
        // (self.mode & S_IFMT) == S_IFIFO
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stat_file_type() {
        let stat = Stats::new(
            FileType::FILE,
            100,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert_eq!(stat.is_file(), true);
        assert_eq!(stat.is_directory(), false);
        assert_eq!(stat.is_symbolic_link(), false);
    }

    #[test]
    fn test_new_stat_directory_type() {
        let stat = Stats::new(
            FileType::DIRECTORY,
            100,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert_eq!(stat.is_file(), false);
        assert_eq!(stat.is_directory(), true);
        assert_eq!(stat.is_symbolic_link(), false);
    }

    #[test]
    fn test_new_stat_symlink_type() {
        let stat = Stats::new(
            FileType::SYMLINK,
            100,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert_eq!(stat.is_file(), false);
        assert_eq!(stat.is_directory(), false);
        assert_eq!(stat.is_symbolic_link(), true);
    }

    #[test]
    fn test_has_access_root() {
        let stat = Stats::new(
            FileType::FILE,
            100,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let cred = Cred::root();
        let has_access = stat.has_access(0b1111, cred);
        assert_eq!(has_access, true);
    }
    #[test]
    fn test_has_access_user() {
        let stat = Stats::new(
            FileType::FILE,
            100,
            None,
            None,
            None,
            None,
            None,
            Some(1000),
            None,
        );
        let cred = Cred {
            uid: 1000,
            gid: 1000,
            euid: 1000,
            egid: 1000,
            suid: 1000,
            sgid: 1000,
        };

        // User has access to the file.
        assert!(stat.has_access(0o777, cred));
    }
    #[test]
    fn test_has_access_group() {
        let stat = Stats::new(
            FileType::FILE,
            100,
            None,
            None,
            None,
            None,
            None,
            Some(1000),
            Some(1000),
        );
        let cred = Cred {
            uid: 2000,
            gid: 1000,
            euid: 2000,
            egid: 1000,
            suid: 2000,
            sgid: 1000,
        };

        // Group has access to the file.
        assert!(stat.has_access(0o777, cred));
    }
    #[test]
    fn test_has_access_other() {
        let stat = Stats::new(
            FileType::FILE,
            100,
            None,
            None,
            None,
            None,
            None,
            Some(1000),
            Some(1000),
        );
        let cred = Cred {
            uid: 2000,
            gid: 2000,
            euid: 2000,
            egid: 2000,
            suid: 2000,
            sgid: 2000,
        };

        // Others have access to the file.
        assert!(stat.has_access(0o777, cred));
    }

    #[test]
    fn test_has_access_no_access() {
        let stat = Stats::new(
            FileType::FILE,
            100,
            None,
            None,
            None,
            None,
            None,
            Some(1000),
            Some(1000),
        );
        let cred = Cred {
            uid: 2000,
            gid: 2000,
            euid: 2000,
            egid: 2000,
            suid: 2000,
            sgid: 2000,
        };

        // No access for the specified mode.
        assert!(!stat.has_access(0o000, cred));
    }

    // Add more tests for other functions...
    #[test]
    fn test_new_stat_default_mode() {
        // When mode is not specified, it should default to directory mode.
        let stat = Stats::new(
            FileType::DIRECTORY,
            100,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert_eq!(stat.mode & 0o777, 0o777); // Check for directory type bits + permission bits
    }

    #[test]
    fn test_chmod() {
        let mut stat = Stats::new(
            FileType::FILE,
            100,
            Some(0o644),
            None,
            None,
            None,
            None,
            None,
            None,
        );

        // Change the mode using chmod.
        stat.chmod(0o755);
        assert_eq!(stat.mode & 0o777, 0o755);
        assert_eq!(stat.is_file(), true);
    }

    #[test]
    fn test_chown_valid_uid_gid() {
        let mut stat = Stats::new(
            FileType::FILE,
            100,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        // Change the owner using chown with valid UID and GID.
        stat.chown(1000, 2000);

        assert_eq!(stat.uid, 1000);
        assert_eq!(stat.gid, 2000);
    }

    #[test]
    fn test_chown_invalid_uid_gid() {
        let mut stat = Stats::new(
            FileType::FILE,
            100,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        // Try to change the owner using chown with invalid UID and GID.
        // this will overflow
        stat.chown(u32::MAX.wrapping_add(1), u32::MAX.wrapping_add(1));

        // Owner should remain unchanged.
        assert_eq!(stat.uid, 0);
        assert_eq!(stat.gid, 0);
    }
}
