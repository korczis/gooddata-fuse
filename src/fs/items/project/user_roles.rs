use fuse::{FileType, ReplyAttr, ReplyData, ReplyEntry, Request};

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::create_inode_file_attributes;
use fs::inode;
use fs::item;
use helpers;
use object;

use super::project_from_inode;

use std::path::Path;

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

fn lookup(fs: &mut GoodDataFS, _req: &Request, parent: u64, _name: &Path, reply: ReplyEntry) {
    let inode_parent = inode::Inode::deserialize(parent);
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::Internal as u8,
        item: 0,
        reserved: constants::ReservedFile::RolesJson as u8,
    });

    let project: &object::Project = &project_from_inode(fs, inode_parent);
    let user_roles = project.user_roles(&mut fs.client.connector);

    if user_roles.is_some() {
        let json: String = user_roles.unwrap().into();
        let attr =
            create_inode_file_attributes(inode, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.entry(&constants::DEFAULT_TTL, &attr, 0);
    }
}

fn read(fs: &mut GoodDataFS, inode: inode::Inode, reply: ReplyData, offset: u64, size: u32) {
    info!("GoodDataFS::read() - Reading {}",
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
    lookup: lookup,
    read: read,
};
