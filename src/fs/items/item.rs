use fuse::{FileType, ReplyAttr, ReplyData, ReplyDirectory, Request};

use fs::GoodDataFS;

use super::super::inode;

// struct GetattrOp {
//     pub op: Box<Fn(&mut GoodDataFS, &Request, u64, ReplyAttr)>,
// }
//
// impl GetattrOp<F: FnMut(uint) -> uin> {
//     pub fn new(op: Box<Fn(&mut GoodDataFS, &Request, u64, ReplyAttr)>) {
//         GetattrOp { op: op }
//     }
// }

// Project Folder Item
pub struct ProjectItem {
    pub category: u8,
    pub reserved: u8,
    pub item_type: FileType,
    pub path: &'static str,

    // FUSE Functions
    pub getattr: fn(&mut GoodDataFS, &Request, u64, ReplyAttr),
    pub read: fn(&mut GoodDataFS,
                 inode::Inode,
                 ReplyData,
                 offset: u64,
                 size: u32),
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
