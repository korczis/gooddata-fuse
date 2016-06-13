use fuse::{FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request};

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::create_inode_directory_attributes;
use fs::inode;
use fs::item;

use std::path::Path;

fn getattr(_fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let attr = create_inode_directory_attributes(ino);
    reply.attr(&constants::DEFAULT_TTL, &attr);
}

fn lookup(_fs: &mut GoodDataFS, _req: &Request, parent: u64, _name: &Path, reply: ReplyEntry) {
    let inode_parent = inode::Inode::deserialize(parent);
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::Ldm as u8,
        item: 0,
        reserved: constants::ReservedFile::KeepMe as u8,
    });

    let attr = create_inode_directory_attributes(inode);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn read(_fs: &mut GoodDataFS, _inode: inode::Inode, _reply: ReplyData, _offset: u64, _size: u32) {}

pub fn readdir(_fs: &mut GoodDataFS,
               _req: &Request,
               _ino: u64,
               _fh: u64,
               _in_offset: u64,
               reply: ReplyDirectory) {
    reply.ok();
}

pub const ITEM: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_LDM_DIR,

    getattr: getattr,
    lookup: lookup,
    read: read,
};
