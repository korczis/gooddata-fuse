use fuse::{FileType, ReplyData, ReplyDirectory, Request};

use fs::constants;
use fs::GoodDataFS;
use fs::inode;
use fs::items::project::project_from_inode;

use object;

#[allow(dead_code)]
pub fn read(_fs: &mut GoodDataFS,
            _req: &Request,
            _ino: u64,
            _fh: u64,
            _offset: u64,
            _size: u32,
            _reply: ReplyData) {
}

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
