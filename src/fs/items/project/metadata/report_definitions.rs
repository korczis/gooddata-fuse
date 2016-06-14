use fuse::{FileType, ReplyAttr, ReplyData, ReplyEntry, Request};

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::create_inode_directory_attributes;
use fs::inode;
use fs::item;
use fs::not_implemeted;

use std::path::Path;

fn getattr(_fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let attr = create_inode_directory_attributes(ino);
    reply.attr(&constants::DEFAULT_TTL, &attr);
}

fn lookup(_fs: &mut GoodDataFS, _req: &Request, parent: u64, _name: &Path, reply: ReplyEntry) {
    let inode_parent = inode::Inode::deserialize(parent);
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: ITEM.category,
        item: 0,
        reserved: ITEM.reserved,
    });

    let attr = create_inode_directory_attributes(inode);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

pub const NAME: &'static str = "definitions";

pub const ITEM: item::ProjectItem = item::ProjectItem {
    category: constants::Category::MetadataReportDefinition as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: NAME,

    getattr: getattr,
    lookup: lookup,
    read: not_implemeted::read,
};
