use fuse::{FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request};

use fs::constants;
use fs::GoodDataFS;
use fs::inode;
use fs::item;
use fs::items::project::project_from_inode;
use fs::not_implemeted;
use helpers;
use object;

use std::path::Path;

fn getattr(fs: &mut GoodDataFS, req: &Request, ino: u64, reply: ReplyAttr) {
    super::helpers::getattr(fs, req, ino, reply)
}

fn lookup(fs: &mut GoodDataFS, req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
    super::helpers::lookup(fs, req, parent, name, reply, &ITEM)
}

pub fn read(fs: &mut GoodDataFS, inode: inode::Inode, reply: ReplyData, offset: u64, size: u32) {
    let project: &object::Project = &project_from_inode(fs, inode);

    let fact = &project.facts(&mut fs.client.connector, false)
        .objects
        .items[inode.item as usize];

    let json: String = fact.clone().into();
    reply.data(helpers::read_bytes(&json, offset, size));
}

pub fn readdir(fs: &mut GoodDataFS,
               _req: &Request,
               ino: u64,
               _fh: u64,
               in_offset: u64,
               mut reply: ReplyDirectory) {
    let inode = inode::Inode::deserialize(ino);
    let project: &object::Project = &project_from_inode(fs, ino);
    let report_items = project.facts(&mut fs.client.connector, true);

    let mut offset = in_offset;
    if offset + 1 < report_items.objects.items.len() as u64 {
        for item in report_items.objects.items.into_iter().skip(offset as usize) {
            let name = format!("{}.json", item.fact.meta.identifier.unwrap());

            // Reports
            let inode = inode::Inode {
                project: inode.project,
                category: ITEM.category,
                item: offset as u32,
                reserved: 0,
            };
            let fileinode: u64 = inode.into();
            reply.add(fileinode, offset, FileType::RegularFile, &name);

            info!("Adding inode {:?}, name {:?}", inode, &name);

            offset += 1;
        }
    }

    reply.ok();
}

pub const ITEM: item::ProjectItem = item::ProjectItem {
    category: constants::Category::MetadataFacts as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: object::fact::NAME,

    getattr: getattr,
    lookup: lookup,
    read: not_implemeted::read,
};
