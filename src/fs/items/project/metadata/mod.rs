use fuse::{FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request};

use fs::GoodDataFS;
use fs::helpers::create_inode_directory_attributes;
use fs::inode;
use fs::item;
use fs::items::project::project_from_inode;
use fs::constants;
use helpers;
use object;

pub mod attributes;
pub mod facts;
pub mod metrics;
pub mod report_definitions;
pub mod reports;

use std::path::Path;

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
            // JSON FACTS
            let project: &object::Project = &project_from_inode(fs, inode);

            let fact = &project.facts(&mut fs.client.connector, false)
                .objects
                .items[inode.item as usize];

            let json: String = fact.clone().into();
            reply.data(helpers::read_bytes(&json, offset, size));
        }
        x if x == constants::Category::MetadataMetrics as u8 => {
            // JSON METRICS
            let project: &object::Project = &project_from_inode(fs, inode);

            let metric = &project.metrics(&mut fs.client.connector, false)
                .objects
                .items[inode.item as usize];

            let json: String = metric.clone().into();
            reply.data(helpers::read_bytes(&json, offset, size));
        }
        x if x == constants::Category::MetadataReports as u8 => {
            // JSON REPORT
            let project: &object::Project = &project_from_inode(fs, inode);

            let report = &project.reports(&mut fs.client.connector, false)
                .objects
                .items[inode.item as usize];

            let json: String = report.clone().into();
            reply.data(helpers::read_bytes(&json, offset, size));
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
