use byteorder::{ByteOrder, LittleEndian};

use super::{
    emulation::constants::{S_IFDIR, S_IFLNK, S_IFREG},
    stat::{FileType, Stats},
};

pub struct Inode {
    pub id: String,
    pub size: u32,
    pub mode: u16,
    pub a_time: f64, // Note: Changed to f64
    pub m_time: f64, // Note: Changed to f64
    pub c_time: f64, // Note: Changed to f64
    pub uid: u32,
    pub gid: u32,
}

impl Inode {
    pub fn new(
        id: String,
        size: u32,
        mode: FileType,
        a_time: f64,
        m_time: f64,
        c_time: f64,
        uid: u32,
        gid: u32,
    ) -> Inode {
        let mode = match mode {
            FileType::FILE => S_IFREG as u16,
            FileType::DIRECTORY => S_IFDIR as u16,
            FileType::SYMLINK => S_IFLNK as u16,
        };
        Inode {
            id,
            size,
            mode,
            a_time,
            m_time,
            c_time,
            uid,
            gid,
        }
    }
    /// Converts the buffer into an Inode.
    pub fn from_buffer(buffer: &[u8]) -> Result<Self, &'static str> {
        if buffer.len() < 50 {
            // Note: Increased size to 50
            return Err("Buffer is too small");
        }

        let id = String::from_utf8_lossy(&buffer[38..38 + 4]).into_owned();
        let size = LittleEndian::read_u32(&buffer[0..4]);
        let mode = LittleEndian::read_u16(&buffer[4..6]);
        let a_time = LittleEndian::read_f64(&buffer[6..14]); // Note: Changed to 8 bytes
        let m_time = LittleEndian::read_f64(&buffer[14..22]); // Note: Changed to 8 bytes
        let c_time = LittleEndian::read_f64(&buffer[22..30]); // Note: Changed to 8 bytes
        let uid = LittleEndian::read_u32(&buffer[30..34]);
        let gid = LittleEndian::read_u32(&buffer[34..38]);

        Ok(Self {
            id,
            size,
            mode,
            a_time,
            m_time,
            c_time,
            uid,
            gid,
        })
    }

    /// Handy function that converts the Inode to a Node Stats object.
    pub fn to_stats(&self) -> Stats {
        // FILE = S_IFREG,
        // DIRECTORY = S_IFDIR,
        // SYMLINK = S_IFLNK,
        let file_type = if (self.mode & 0xf000) == S_IFDIR as u16 {
            FileType::DIRECTORY
        } else {
            FileType::FILE
        };

        Stats::new(
            file_type,
            self.size as u64,
            Some(self.mode as u32),
            Some(self.a_time as u64),
            Some(self.m_time as u64),
            Some(self.c_time as u64),
            None,
            Some(self.uid),
            Some(self.gid),
        )
    }
    /// Get the size of this Inode, in bytes.
    pub fn get_size(&self) -> usize {
        // ASSUMPTION: ID is ASCII (1 byte per char).
        38 + self.id.len()
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = vec![0; self.get_size()];

        LittleEndian::write_u32(&mut buffer[0..4], self.size);
        LittleEndian::write_u16(&mut buffer[4..6], self.mode); // Adjusted index range
        LittleEndian::write_f64(&mut buffer[6..14], self.a_time);
        LittleEndian::write_f64(&mut buffer[14..22], self.m_time);
        LittleEndian::write_f64(&mut buffer[22..30], self.c_time);
        LittleEndian::write_u32(&mut buffer[30..34], self.uid);
        LittleEndian::write_u32(&mut buffer[34..38], self.gid);
        buffer[38..38 + self.id.len()].copy_from_slice(self.id.as_bytes());

        buffer
    }

    // ...

    /// Updates the Inode using information from the stats object. Used by file
    /// systems at sync time.
    ///
    /// Returns `true` if any changes have occurred.
    pub fn update(&mut self, stats: &Stats) -> bool {
        let mut has_changed = false;

        if self.size != stats.size as u32 {
            self.size = stats.size as u32;
            has_changed = true;
        }

        if self.mode != stats.mode as u16 {
            self.mode = stats.mode as u16;
            has_changed = true;
        }

        let atime = stats.atime();
        let atime = match atime.duration_since(std::time::UNIX_EPOCH) {
            Ok(n) => n.as_secs() as f64,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        if self.a_time != atime {
            self.a_time = atime;
            has_changed = true;
        }

        let mtime = stats.mtime();
        let mtime = match mtime.duration_since(std::time::UNIX_EPOCH) {
            Ok(n) => n.as_secs() as f64,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        if self.m_time != mtime {
            self.m_time = mtime;
            has_changed = true;
        }

        let ctime = stats.ctime();
        let ctime = match ctime.duration_since(std::time::UNIX_EPOCH) {
            Ok(n) => n.as_secs() as f64,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        if self.c_time != ctime {
            self.c_time = ctime;
            has_changed = true;
        }

        if self.uid != stats.uid {
            self.uid = stats.uid;
            has_changed = true;
        }

        if self.gid != stats.gid {
            self.gid = stats.gid;
            has_changed = true;
        }

        has_changed
    }
    // Check if this item is a file
    pub fn is_file(&self) -> bool {
        // FILE = S_IFREG,
        // DIRECTORY = S_IFDIR,
        // SYMLINK = S_IFLNK,
        (self.mode & 0xf000) == S_IFREG as u16
    }

    // Check if this item is a directory
    pub fn is_directory(&self) -> bool {
        // FILE = S_IFREG,
        // DIRECTORY = S_IFDIR,
        // SYMLINK = S_IFLNK,
        (self.mode & 0xf000) == S_IFDIR as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_from_buffer() {
        // Prepare a buffer with known values
        let mut buffer = vec![0; 50]; // Note: Increased size to 50
        LittleEndian::write_u32(&mut buffer[0..4], 123); // size
        LittleEndian::write_u16(&mut buffer[4..6], 456); // mode
        LittleEndian::write_f64(&mut buffer[6..14], 789.0); // a_time
        LittleEndian::write_f64(&mut buffer[14..22], 321.0); // m_time
        LittleEndian::write_f64(&mut buffer[22..30], 654.0); // c_time
        LittleEndian::write_u32(&mut buffer[30..34], 987); // uid
        LittleEndian::write_u32(&mut buffer[34..38], 135); // gid
        buffer[38..42].copy_from_slice(b"abcd"); // id

        // Call from_buffer
        let inode = Inode::from_buffer(&buffer).unwrap();

        // Check the values
        assert_eq!(inode.size, 123);
        assert_eq!(inode.mode, 456);
        assert_eq!(inode.a_time, 789.0);
        assert_eq!(inode.m_time, 321.0);
        assert_eq!(inode.c_time, 654.0);
        assert_eq!(inode.uid, 987);
        assert_eq!(inode.gid, 135);
        assert_eq!(inode.id, "abcd");
    }

    #[test]

    fn test_to_buffer() {
        // Create an Inode with known values
        let inode = Inode {
            id: "abcd".to_string(),
            size: 123,
            mode: 456,
            a_time: 789.0, // Note: Changed to f64
            m_time: 321.0, // Note: Changed to f64
            c_time: 654.0, // Note: Changed to f64
            uid: 987,
            gid: 135,
        };

        // Call to_buffer
        let buffer = inode.to_buffer();

        // Check the values
        assert_eq!(LittleEndian::read_u32(&buffer[0..4]), 123);
        assert_eq!(LittleEndian::read_u16(&buffer[4..6]), 456);
        assert_eq!(LittleEndian::read_f64(&buffer[6..14]), 789.0);
        assert_eq!(LittleEndian::read_f64(&buffer[14..22]), 321.0);
        assert_eq!(LittleEndian::read_f64(&buffer[22..30]), 654.0);
        assert_eq!(LittleEndian::read_u32(&buffer[30..34]), 987);
        assert_eq!(LittleEndian::read_u32(&buffer[34..38]), 135);
        assert_eq!(&buffer[38..42], b"abcd");
    }

    #[test]
    fn test_to_stats() {
        // Create an Inode with known values
        let inode = Inode {
            id: "abcd".to_string(),
            size: 123,
            mode: 0o644,   // Adjust the mode according to your FileType enum
            a_time: 789.0, // Note: Changed to f64
            m_time: 321.0, // Note: Changed to f64
            c_time: 654.0, // Note: Changed to f64
            uid: 987,
            gid: 135,
        };

        // Call to_stats
        let stats = inode.to_stats();

        // Check the values
        assert_eq!(stats.size, 123);
        assert_eq!(stats.mode & 0o777, 0o644);
        assert_eq!(stats.uid, 987);
        assert_eq!(stats.gid, 135);
        assert_eq!(
            stats.atime(),
            SystemTime::UNIX_EPOCH + Duration::from_millis(789)
        );
        assert_eq!(
            stats.mtime(),
            SystemTime::UNIX_EPOCH + Duration::from_millis(321)
        )
        // Add more assertions based on your specific requirements
    }

    #[test]
    fn test_update() {
        let mut inode = Inode {
            id: String::from("test"),
            size: 4,
            mode: 0o755,
            a_time: 1000.0,
            m_time: 2218914892.0,
            c_time: 2218914892.0,
            uid: 305419896,
            gid: 2018915346,
        };

        let new_stats = Stats::new(
            FileType::FILE,
            8,
            Some(0o644 as u32),
            Some(2000),
            Some(3000),
            Some(4000),
            None,
            Some(123),
            Some(456),
        );

        let has_changed = inode.update(&new_stats);

        assert!(has_changed);
        assert_eq!(inode.size, 8);
        // filter out the permission bits first
        assert_eq!(inode.mode & 0o777, 0o644);
        assert_eq!(inode.a_time, 2.0);
        assert_eq!(inode.m_time, 3.0);
        assert_eq!(inode.c_time, 4.0);
        assert_eq!(inode.uid, 123);
        assert_eq!(inode.gid, 456);
    }

    #[test]
    fn test_is_file() {
        let file_inode = Inode::new(
            String::from("test"),
            4,
            FileType::FILE,
            1000.0,
            2218914892.0,
            2218914892.0,
            305419896,
            2018915346,
        );

        assert!(file_inode.is_file());
        assert!(!file_inode.is_directory());
    }

    #[test]
    fn test_is_directory() {
        let dir_inode = Inode::new(
            String::from("test"),
            4,
            FileType::DIRECTORY,
            1000.0,
            2218914892.0,
            2218914892.0,
            305419896,
            2018915346,
        );

        assert!(!dir_inode.is_file());
        assert!(dir_inode.is_directory());
    }
}
