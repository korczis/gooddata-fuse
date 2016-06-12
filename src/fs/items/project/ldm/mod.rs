use fuse::{FileType, ReplyDirectory, Request};

use fs::GoodDataFS;

use super::super::super::inode;
use super::super::super::constants;

pub fn readdir(_fs: &mut GoodDataFS,
               _req: &Request,
               _ino: u64,
               _fh: u64,
               _in_offset: u64,
               mut reply: ReplyDirectory) {
    reply.ok();
}
