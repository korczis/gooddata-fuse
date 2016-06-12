use fuse::{FileType, ReplyDirectory, Request};

use fs::GoodDataFS;

use super::super::super::inode;
use super::super::super::constants;

pub mod reports;

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
