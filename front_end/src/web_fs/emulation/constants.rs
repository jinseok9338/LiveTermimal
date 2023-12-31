// File Access Constants
pub const F_OK: u32 = 0; // Constant for fs.access(). File is visible to the calling process.

// File Access Constants
pub const R_OK: u32 = 4; // Constant for fs.access(). File can be read by the calling process.

// File Access Constants
pub const W_OK: u32 = 2; // Constant for fs.access(). File can be written by the calling process.

// File Access Constants
pub const X_OK: u32 = 1; // Constant for fs.access(). File can be executed by the calling process.

// File Copy Constants
pub const COPYFILE_EXCL: u32 = 1; // Flag indicating the destination file should not be overwritten if it already exists.

// File Copy Constants
pub const COPYFILE_FICLONE: u32 = 2; // Copy operation will attempt to create a copy-on-write reflink.
                                     // If the underlying platform does not support copy-on-write, then a fallback copy mechanism is used.

// File Copy Constants
pub const COPYFILE_FICLONE_FORCE: u32 = 4; // Copy operation will attempt to create a copy-on-write reflink.
                                           // If the underlying platform does not support copy-on-write, then the operation will fail with an error.

// File Open Constants
pub const O_RDONLY: u32 = 0; // Flag indicating to open a file for read-only access.

// File Open Constants
pub const O_WRONLY: u32 = 1; // Flag indicating to open a file for write-only access.

// File Open Constants
pub const O_RDWR: u32 = 2; // Flag indicating to open a file for read-write access.

// File Open Constants
pub const O_CREAT: u32 = 0o100; // Flag indicating to create the file if it does not already exist.

// File Open Constants
pub const O_EXCL: u32 = 0o200; // Flag indicating that opening a file should fail if the O_CREAT flag is set and the file already exists.

// File Open Constants
pub const O_NOCTTY: u32 = 0o400; // Flag indicating that if path identifies a terminal device,
                                 // Opening the path shall not cause that terminal to become the controlling terminal for the process
                                 // (if the process does not already have one).

// File Open Constants
pub const O_TRUNC: u32 = 0o1000; // Flag indicating that if the file exists and is a regular file,
                                 // And the file is opened successfully for write access, its length shall be truncated to zero.

// File Open Constants
pub const O_APPEND: u32 = 0o2000; // Flag indicating that data will be appended to the end of the file.

// File Open Constants
pub const O_DIRECTORY: u32 = 0o200000; // Flag indicating that the open should fail if the path is not a directory.

// File Open Constants
pub const O_NOATIME: u32 = 0o1000000; // Flag indicating reading accesses to the file system will no longer result in
                                      // An update to the atime information associated with the file.
                                      // This flag is available on Linux operating systems only.

// File Open Constants
pub const O_NOFOLLOW: u32 = 0o400000; // Flag indicating that the open should fail if the path is a symbolic link.

// File Open Constants
pub const O_SYNC: u32 = 0o4010000; // Flag indicating that the file is opened for synchronous I/O.

// File Open Constants
pub const O_DSYNC: u32 = 0o10000; // Flag indicating that the file is opened for synchronous I/O with write operations waiting for data integrity.

// File Open Constants
pub const O_SYMLINK: u32 = 0o100000; // Flag indicating to open the symbolic link itself rather than the resource it is pointing to.

// File Open Constants
pub const O_DIRECT: u32 = 0o40000; // Flag indicating to open the file in nonblocking mode when possible.

// File Open Constants
pub const O_NONBLOCK: u32 = 0o4000; // Flag indicating to open the file in nonblocking mode when possible.

// File Type Constants
pub const S_IFMT: u32 = 0o170000; // Constant for fs.Stats mode property for determining a file's type.
                                  // Bit mask used to extract the file type code.

// File Type Constants
pub const S_IFREG: u32 = 0o100000; // Constant for fs.Stats mode property for determining a file's type.
                                   // File type constant for a regular file.

// File Type Constants
pub const S_IFDIR: u32 = 0o40000; // Constant for fs.Stats mode property for determining a file's type.
                                  // File type constant for a directory.

// File Type Constants
pub const S_IFCHR: u32 = 0o20000; // Constant for fs.Stats mode property for determining a file's type.
                                  // File type constant for a character-oriented device file.

// File Type Constants
pub const S_IFBLK: u32 = 0o60000; // Constant for fs.Stats mode property for determining a file's type.
                                  // File type constant for a block-oriented device file.

// File Type Constants
pub const S_IFIFO: u32 = 0o10000; // Constant for fs.Stats mode property for determining a file's type.
                                  // File type constant for a FIFO/pipe.

// File Type Constants
pub const S_IFLNK: u32 = 0o120000; // Constant for fs.Stats mode property for determining a file's type.
                                   // File type constant for a symbolic link.

// File Type Constants
pub const S_IFSOCK: u32 = 0o140000; // Constant for fs.Stats mode property for determining a file's type.
                                    // File type constant for a socket.

// File Mode Constants
pub const S_IRWXU: u32 = 0o700; // Constant for fs.Stats mode property for determining access permissions for a file.
                                // File mode indicating readable, writable and executable by owner.

// File Mode Constants
pub const S_IRUSR: u32 = 0o400; // Constant for fs.Stats mode property for determining access permissions for a file.
                                // File mode indicating readable by owner.

// File Mode Constants
pub const S_IWUSR: u32 = 0o200; // Constant for fs.Stats mode property for determining access permissions for a file.
                                // File mode indicating writable by owner.

// File Mode Constants
pub const S_IXUSR: u32 = 0o100; // Constant for fs.Stats mode property for determining access permissions for a file.
                                // File mode indicating executable by owner.

// File Mode Constants
pub const S_IRWXG: u32 = 0o70; // Constant for fs.Stats mode property for determining access permissions for a file.
                               // File mode indicating readable, writable and executable by group.

// File Mode Constants
pub const S_IRGRP: u32 = 0o40; // Constant for fs.Stats mode property for determining access permissions for a file.
                               // File mode indicating readable by group.

// File Mode Constants
pub const S_IWGRP: u32 = 0o20; // Constant for fs.Stats mode property for determining access permissions for a file.
                               // File mode indicating writable by group.

// File Mode Constants
pub const S_IXGRP: u32 = 0o10; // Constant for fs.Stats mode property for determining access permissions for a file.
                               // File mode indicating executable by group.

// File Mode Constants
pub const S_IRWXO: u32 = 7; // Constant for fs.Stats mode property for determining access permissions for a file.
                            // File mode indicating readable, writable and executable by others.

// File Mode Constants
pub const S_IROTH: u32 = 4; // Constant for fs.Stats mode property for determining access permissions for a file.
                            // File mode indicating readable by others.

// File Mode Constants
pub const S_IWOTH: u32 = 2; // Constant for fs.Stats mode property for determining access permissions for a file.
                            // File mode indicating writable by others.

// File Mode Constants
pub const S_IXOTH: u32 = 1; // Constant for fs.Stats mode property for determining access permissions for a file.
                            // File mode indicating executable by others.
