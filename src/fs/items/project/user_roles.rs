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

    let user_roles = project.user_roles(&mut fs.client.connector);

    if user_roles.is_some() {
        let json: String = user_roles.unwrap().into();
        let attr =
            create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.attr(&constants::DEFAULT_TTL, &attr);
    }
}

fn read(fs: &mut GoodDataFS, inode: inode::Inode, reply: ReplyData, offset: u64, size: u32) {
    println!("GoodDataFS::read() - Reading {}",
             constants::USER_ROLES_JSON_FILENAME);

    let project: &object::Project = &project_from_inode(fs, inode);

    let user_roles = project.user_roles(&mut fs.client.connector);
    if user_roles.is_some() {
        let json: String = user_roles.unwrap().into();
        reply.data(helpers::read_bytes(&json, offset, size));
    }
}

pub const ITEM: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Ldm as u8,
    reserved: constants::ReservedFile::RolesJson as u8,
    item_type: FileType::RegularFile,
    path: constants::USER_ROLES_JSON_FILENAME,
    getattr: getattr,
    read: read,
};
