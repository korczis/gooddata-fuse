use fuse::{FileType, ReplyDirectory};

use super::super::inode;

// Project Folder Item
pub struct ProjectItem {
    pub category: u8,
    pub reserved: u8,
    pub item_type: FileType,
    pub path: &'static str,
}

impl ProjectItem {
    pub fn readdir(&self, project_idx: u16, offset: &u64, mut reply: &mut ReplyDirectory) {
        let inode = inode::Inode {
            project: project_idx + 1,
            category: self.category,
            item: 0,
            reserved: self.reserved,
        };

        let fileinode: u64 = inode.into();
        println!("Folder::readdir() - Adding inode {} - {:?}, project {}, \
                  path {}",
                 fileinode,
                 &inode,
                 project_idx,
                 self.path);

        reply.add(fileinode, *offset, self.item_type, self.path);
    }
}
