use fuse::{FileType, ReplyAttr, ReplyData, ReplyDirectory, Request};

use fs::GoodDataFS;
use fs::inode;
use fs::item;
use fs::constants;

fn read(_fs: &mut GoodDataFS, _inode: inode::Inode, _reply: ReplyData, _offset: u64, _size: u32) {}

pub fn readdir(_fs: &mut GoodDataFS,
               _req: &Request,
               _ino: u64,
               _fh: u64,
               _in_offset: u64,
               reply: ReplyDirectory) {
    reply.ok();
}

fn getattr(_fs: &mut GoodDataFS, _req: &Request, _ino: u64, _reply: ReplyAttr) {}
pub const ITEM: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_LDM_DIR,

    getattr: getattr,
    read: read,
};
