/**
 * Credentials used for FS ops.
 * Similar to Linux's cred struct. See https://github.com/torvalds/linux/blob/master/include/linux/cred.h
 */
// export class Cred {
// 	constructor(public uid: number, public gid: number, public suid: number, public sgid: number, public euid: number, public egid: number) {}

// 	public static Root = new Cred(0, 0, 0, 0, 0, 0);
// }

// uid;		/* real UID of the task */
// gid;		/* real GID of the task */
// suid;		/* saved UID of the task */
// sgid;		/* saved GID of the task */
// euid;		/* effective UID of the task */
// egid;		/* effective GID of the task */

#[derive(Clone, Debug, PartialEq)]
pub struct Cred {
    pub uid: u32,
    pub gid: u32,
    pub euid: u32,
    pub egid: u32,
    pub suid: u32,
    pub sgid: u32,
}

impl Cred {
    pub fn root() -> Cred {
        Cred {
            uid: 0,
            gid: 0,
            euid: 0,
            egid: 0,
            suid: 0,
            sgid: 0,
        }
    }
}
