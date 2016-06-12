use rustc_serialize::json;

use fuse::{FileType, ReplyAttr, ReplyData, Request};

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::create_inode_file_attributes;
use fs::inode;
use helpers;
use object;

use super::project_from_inode;
use super::super::item;

fn getattr(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let project: &object::Project = &project_from_inode(fs, ino);
    let json = json::as_pretty_json(project).to_string();

    let attr = create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
    reply.attr(&constants::DEFAULT_TTL, &attr);
}

fn read(fs: &mut GoodDataFS, inode: inode::Inode, reply: ReplyData, offset: u64, size: u32) {
    println!("GoodDataFS::read() - Reading {}",
             constants::PROJECT_JSON_FILENAME);

    let project: &object::Project = &project_from_inode(fs, inode);
    let json = json::as_pretty_json(project).to_string();
    reply.data(helpers::read_bytes(&json, offset, size));
}

pub const ITEM: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::ProjectJson as u8,
    item_type: FileType::RegularFile,
    path: constants::PROJECT_JSON_FILENAME,
    getattr: getattr,
    read: read,
};
