use fuse::{FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request};

use fs::GoodDataFS;
use fs::helpers::create_inode_directory_attributes;
use fs::inode;
use fs::item;
use fs::constants;

pub mod attributes;
pub mod facts;
pub mod metrics;
pub mod report_definitions;
pub mod reports;

use std::path::Path;

pub struct MetadataItem {
    pub category: u8,
    pub reserved: u8,
    pub item: u32,
    pub item_type: FileType,
    pub path: &'static str,
}

pub const ITEM_METADATA_ATTRIBUTES_DIR: MetadataItem = MetadataItem {
    category: constants::Category::MetadataAttributes as u8,
    item: 0,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_METADATA_ATTRIBUTES_DIR,
};

pub const ITEM_METADATA_FACTS_DIR: MetadataItem = MetadataItem {
    category: constants::Category::MetadataFacts as u8,
    item: 0,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_METADATA_FACTS_DIR,
};

pub const ITEM_METADATA_METRICS_DIR: MetadataItem = MetadataItem {
    category: constants::Category::MetadataMetrics as u8,
    item: 0,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_METADATA_METRICS_DIR,
};

pub const ITEM_METADATA_REPORTS_DIR: MetadataItem = MetadataItem {
    category: constants::Category::MetadataReports as u8,
    item: 0,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_METADATA_REPORTS_DIR,
};

pub const ITEM_METADATA_REPORT_DEFINITIONS_DIR: MetadataItem = MetadataItem {
    category: constants::Category::MetadataReportDefinition as u8,
    item: 0,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_METADATA_REPORT_DEFINITIONS_DIR,
};

pub const ITEMS: [MetadataItem; 5] = [ITEM_METADATA_ATTRIBUTES_DIR,
                                      ITEM_METADATA_FACTS_DIR,
                                      ITEM_METADATA_METRICS_DIR,
                                      ITEM_METADATA_REPORTS_DIR,
                                      ITEM_METADATA_REPORT_DEFINITIONS_DIR];


fn getattr(_fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let attr = create_inode_directory_attributes(ino);
    reply.attr(&constants::DEFAULT_TTL, &attr);
}

fn lookup(_fs: &mut GoodDataFS, _req: &Request, parent: u64, _name: &Path, reply: ReplyEntry) {
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

pub fn read(fs: &mut GoodDataFS, inode: inode::Inode, reply: ReplyData, offset: u64, size: u32) {
    match inode.category {
        x if x == constants::Category::Internal as u8 => {}
        x if x == constants::Category::MetadataFacts as u8 => {
            facts::read(fs, inode, reply, offset, size)
        }
        x if x == constants::Category::MetadataMetrics as u8 => {
            metrics::read(fs, inode, reply, offset, size)
        }
        x if x == constants::Category::MetadataReports as u8 => {
            reports::read(fs, inode, reply, offset, size)
        }
        _ => warn!("read() - {:?} - Unknown category", inode),
    }
}

pub fn readdir(_fs: &mut GoodDataFS,
               _req: &Request,
               ino: u64,
               _fh: u64,
               in_offset: u64,
               mut reply: ReplyDirectory) {
    let inode = inode::Inode::deserialize(ino);
    let mut offset = in_offset;
    if offset == 0 {
        for item in ITEMS.into_iter() {
            let inode = inode::Inode {
                project: inode.project,
                category: item.category,
                item: item.item,
                reserved: item.reserved,
            };
            let fileinode: u64 = inode.into();
            reply.add(fileinode, offset, item.item_type, item.path);

            offset += 1;
        }

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
