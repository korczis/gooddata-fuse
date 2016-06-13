use fuse::{FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request};

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::create_inode_directory_attributes;
use fs::inode;
use fs::item;
use fs::items::project::project_from_inode;
use object;

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

fn read(_fs: &mut GoodDataFS, _inode: inode::Inode, _reply: ReplyData, _offset: u64, _size: u32) {}

pub fn readdir(fs: &mut GoodDataFS,
               _req: &Request,
               ino: u64,
               _fh: u64,
               in_offset: u64,
               mut reply: ReplyDirectory) {
    let inode = inode::Inode::deserialize(ino);
    let project: &object::Project = &project_from_inode(fs, ino);
    let report_items = project.reports(&mut fs.client.connector);

    let mut offset = in_offset;
    if offset + 1 < report_items.objects.items.len() as u64 {
        for item in report_items.objects.items.into_iter().skip(offset as usize) {
            let name = format!("{}.json", item.report.meta.identifier.unwrap());

            // Reports
            let inode = inode::Inode {
                project: inode.project,
                category: constants::Category::MetadataReports as u8,
                item: offset as u32,
                reserved: 1,
            };
            let fileinode: u64 = inode.into();
            reply.add(fileinode, offset, FileType::RegularFile, &name);

            println!("Adding inode {:?}, name {:?}", inode, &name);

            offset += 1;
        }
    }

    reply.ok();
}

pub const NAME: &'static str = "reports";

pub const ITEM: item::ProjectItem = item::ProjectItem {
    category: constants::Category::MetadataReports as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: NAME,

    getattr: getattr,
    lookup: lookup,
    read: read,
};
