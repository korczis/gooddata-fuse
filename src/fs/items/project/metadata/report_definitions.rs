use fuse::{FileType, ReplyAttr, ReplyEntry, Request};

use fs::constants;
use fs::GoodDataFS;
use fs::item;
use fs::not_implemeted;
use object;

use std::path::Path;

fn getattr(fs: &mut GoodDataFS, req: &Request, ino: u64, reply: ReplyAttr) {
    super::helpers::getattr(fs, req, ino, reply)
}

fn lookup(fs: &mut GoodDataFS, req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
    super::helpers::lookup(fs, req, parent, name, reply, &ITEM)
}

pub const ITEM: item::ProjectItem = item::ProjectItem {
    category: constants::Category::MetadataReportDefinition as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: object::report_definition::NAME,

    getattr: getattr,
    lookup: lookup,
    read: not_implemeted::read,
};
