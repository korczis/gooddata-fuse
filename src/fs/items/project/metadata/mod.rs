use fuse::{FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request};

use fs::GoodDataFS;
use fs::helpers::create_inode_directory_attributes;
use fs::inode;
use fs::item;
use fs::constants;

pub mod reports;

use std::path::Path;


fn getattr(_fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let attr = create_inode_directory_attributes(ino);
    reply.attr(&constants::DEFAULT_TTL, &attr);
}

fn lookup(fs: &mut GoodDataFS, _req: &Request, parent: u64, _name: &Path, reply: ReplyEntry) {
    let inode_parent = inode::Inode::deserialize(parent);
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::Metadata as u8,
        item: 0,
        reserved: constants::ReservedFile::KeepMe as u8,
    });

    let attr = create_inode_directory_attributes(inode);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn read(_fs: &mut GoodDataFS, _inode: inode::Inode, _reply: ReplyData, _offset: u64, _size: u32) {}

pub fn readdir(_fs: &mut GoodDataFS,
               _req: &Request,
               ino: u64,
               _fh: u64,
               in_offset: u64,
               mut reply: ReplyDirectory) {
    let inode = inode::Inode::deserialize(ino);
    let mut offset = in_offset;
    if offset == 0 {
        // Attributes
        let inode = inode::Inode {
            project: inode.project,
            category: constants::Category::MetadataAttributes as u8,
            item: 0,
            reserved: constants::ReservedFile::KeepMe as u8,
        };
        let fileinode: u64 = inode.into();
        reply.add(fileinode,
                  offset,
                  FileType::Directory,
                  constants::PROJECT_METADATA_ATTRIBUTES_DIR);
        offset += 1;

        // Facts
        let inode = inode::Inode {
            project: inode.project,
            category: constants::Category::MetadataFacts as u8,
            item: 0,
            reserved: constants::ReservedFile::KeepMe as u8,
        };
        let fileinode: u64 = inode.into();
        reply.add(fileinode,
                  offset,
                  FileType::Directory,
                  constants::PROJECT_METADATA_FACTS_DIR);
        offset += 1;

        // Metrics
        let inode = inode::Inode {
            project: inode.project,
            category: constants::Category::MetadataMetrics as u8,
            item: 0,
            reserved: constants::ReservedFile::KeepMe as u8,
        };
        let fileinode: u64 = inode.into();
        reply.add(fileinode,
                  offset,
                  FileType::Directory,
                  constants::PROJECT_METADATA_METRICS_DIR);
        offset += 1;

        // Reports
        let inode = inode::Inode {
            project: inode.project,
            category: constants::Category::MetadataReports as u8,
            item: 0,
            reserved: constants::ReservedFile::KeepMe as u8,
        };
        let fileinode: u64 = inode.into();
        reply.add(fileinode,
                  offset,
                  FileType::Directory,
                  constants::PROJECT_METADATA_REPORTS_DIR);
        offset += 1;

        // Report Definitions
        let inode = inode::Inode {
            project: inode.project,
            category: constants::Category::MetadataReportDefinition as u8,
            item: 0,
            reserved: constants::ReservedFile::KeepMe as u8,
        };
        let fileinode: u64 = inode.into();
        reply.add(fileinode,
                  offset,
                  FileType::Directory,
                  constants::PROJECT_METADATA_REPORT_DEFINITIONS_DIR);

        // offset += 1;

        reply.ok();
    }
}

pub const ITEM: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Metadata as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_METADATA_DIR,

    getattr: getattr,
    lookup: lookup,
    read: read,
};
