use fuse::{ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory, Request};
use libc::ENOENT;

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::create_inode_file_attributes;
use object;

mod feature_flags;
mod ldm;
mod metadata;
mod project_json;
mod user_permissions;
mod user_roles;

use super::item;
use super::project;

use super::super::inode;

use std::path::Path;

pub const PROJECT_FILES: [item::ProjectItem; 4] =
    [feature_flags::ITEM, project_json::ITEM, user_permissions::ITEM, user_roles::ITEM];

pub const PROJECT_DIRS: [item::ProjectItem; 2] = [ldm::ITEM, metadata::ITEM];

pub const PROJECT_ITEMS: [item::ProjectItem; 6] = [feature_flags::ITEM,
                                                   project_json::ITEM,
                                                   user_permissions::ITEM,
                                                   user_roles::ITEM,
                                                   ldm::ITEM,
                                                   metadata::ITEM];

/// Gets project from inode
pub fn project_from_inode<Type: Into<inode::Inode>>(fs: &GoodDataFS, ino: Type) -> object::Project {
    let inode = ino.into();
    let pid = (inode.project - 1) as usize;

    fs.client().projects().as_ref().unwrap()[pid].clone()
}

pub fn getattr(fs: &mut GoodDataFS, req: &Request, ino: u64, reply: ReplyAttr) {
    let inode = inode::Inode::deserialize(ino);
    println!("fs::project::getattr() {} - {:?}", ino, inode);

    if inode.project > 0 {
        match inode.category {
            x if x == constants::Category::Internal as u8 => {
                let reserved = constants::ReservedFile::from(inode.reserved);
                match reserved {
                    constants::ReservedFile::FeatureFlagsJson => {
                        (feature_flags::ITEM.getattr)(fs, req, ino, reply)
                    }
                    constants::ReservedFile::ProjectJson => {
                        (project_json::ITEM.getattr)(fs, req, ino, reply)
                    }
                    constants::ReservedFile::PermissionsJson => {
                        (user_permissions::ITEM.getattr)(fs, req, ino, reply)
                    }
                    constants::ReservedFile::RolesJson => {
                        (user_roles::ITEM.getattr)(fs, req, ino, reply)
                    }
                    _ => reply.error(ENOENT),
                }
            }
            x if x == constants::Category::Ldm as u8 => (ldm::ITEM.getattr)(fs, req, ino, reply),
            x if x == constants::Category::Metadata as u8 => {
                (metadata::ITEM.getattr)(fs, req, ino, reply)
            }
            x if x == constants::Category::MetadataAttributes as u8 => {
                (metadata::attributes::ITEM.getattr)(fs, req, ino, reply)
            }
            x if x == constants::Category::MetadataFacts as u8 => {
                (metadata::facts::ITEM.getattr)(fs, req, ino, reply)
            }
            x if x == constants::Category::MetadataMetrics as u8 => {
                (metadata::metrics::ITEM.getattr)(fs, req, ino, reply)
            }
            x if x == constants::Category::MetadataReports as u8 => {
                if inode.reserved == constants::ReservedFile::KeepMe as u8 {
                    (metadata::reports::ITEM.getattr)(fs, req, ino, reply)
                } else if inode.reserved == 0 {
                    // JSON REPORT
                    let project: &object::Project = &project_from_inode(fs, ino);

                    let report = &project.reports(&mut fs.client.connector)
                        .objects
                        .items[inode.item as usize];

                    let json: String = report.clone().into();
                    let attr = create_inode_file_attributes(ino,
                                                            json.len() as u64,
                                                            constants::DEFAULT_CREATE_TIME);
                    reply.attr(&constants::DEFAULT_TTL, &attr);
                    println!("Getting attributes (report.json) {:?}", inode);
                } else {
                    println!("N A S T A L   H A P R ! ! !");
                    reply.error(ENOENT);
                }
            }
            x if x == constants::Category::MetadataReportDefinition as u8 => {
                (metadata::report_definitions::ITEM.getattr)(fs, req, ino, reply)
            }
            _ => {
                println!("fs::project::getattr() - not found!");
            }
        }
    } else {
        println!("GoodDataFS::getattr() - Not found inode {:?}", ino);
        reply.error(ENOENT);
    }
}

pub fn lookup(fs: &mut GoodDataFS, req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
    let inode = inode::Inode::deserialize(parent);

    match name.to_str() {
        Some(constants::FEATURE_FLAGS_JSON_FILENAME) => {
            (feature_flags::ITEM.lookup)(fs, req, parent, name, reply)
        }
        Some(constants::PROJECT_JSON_FILENAME) => {
            (project_json::ITEM.lookup)(fs, req, parent, name, reply)
        }
        Some(constants::PROJECT_LDM_DIR) => (ldm::ITEM.lookup)(fs, req, parent, name, reply),
        Some(constants::PROJECT_METADATA_DIR) => {
            (metadata::ITEM.lookup)(fs, req, parent, name, reply)
        }
        Some(constants::PROJECT_METADATA_ATTRIBUTES_DIR) => {
            (metadata::attributes::ITEM.lookup)(fs, req, parent, name, reply)
        }
        Some(constants::PROJECT_METADATA_FACTS_DIR) => {
            (metadata::facts::ITEM.lookup)(fs, req, parent, name, reply)
        }
        Some(constants::PROJECT_METADATA_METRICS_DIR) => {
            (metadata::metrics::ITEM.lookup)(fs, req, parent, name, reply)
        }
        Some(constants::PROJECT_METADATA_REPORTS_DIR) => {
            (metadata::reports::ITEM.lookup)(fs, req, parent, name, reply)
        }
        Some(constants::PROJECT_METADATA_REPORT_DEFINITIONS_DIR) => {
            (metadata::report_definitions::ITEM.lookup)(fs, req, parent, name, reply)
        }
        Some(constants::USER_PERMISSIONS_JSON_FILENAME) => {
            (user_permissions::ITEM.lookup)(fs, req, parent, name, reply)
        }
        Some(constants::USER_ROLES_JSON_FILENAME) => {
            (user_roles::ITEM.lookup)(fs, req, parent, name, reply)
        }
        _ => {
            if inode.category == constants::Category::MetadataReports as u8 &&
               inode.reserved == constants::ReservedFile::KeepMe as u8 {
                let identifier = name.to_str().unwrap().replace(".json", "");
                println!("fs::project::lookup() - Looking up parent {} - {:?}, name: {:?}, \
                          identifier: {:?}",
                         parent,
                         inode,
                         name,
                         identifier);

                let project: &object::Project = &project_from_inode(fs, parent);

                let (index, report) = project.reports(&mut fs.client.connector)
                    .find_by_identifier(&identifier);
                println!("{:?}", report);

                if !report.is_some() {
                    reply.error(ENOENT);
                    return;
                }

                let inode = inode::Inode {
                    project: inode.project,
                    category: constants::Category::MetadataReports as u8,
                    item: index,
                    reserved: 0,
                };
                let json: String = report.unwrap().into();
                let attr = create_inode_file_attributes(inode::Inode::serialize(&inode),
                                                        json.len() as u64,
                                                        constants::DEFAULT_CREATE_TIME);
                reply.entry(&constants::DEFAULT_TTL, &attr, 0);
            } else {
                reply.error(ENOENT)
            }
        }
    }
}

pub fn read(fs: &mut GoodDataFS,
            _req: &Request,
            ino: u64,
            _fh: u64,
            offset: u64,
            size: u32,
            reply: ReplyData) {
    let inode = inode::Inode::deserialize(ino);
    let reserved = constants::ReservedFile::from(inode.reserved);
    match reserved {
        constants::ReservedFile::FeatureFlagsJson => {
            (feature_flags::ITEM.read)(fs, inode, reply, offset, size)
        }
        constants::ReservedFile::ProjectJson => {
            (project_json::ITEM.read)(fs, inode, reply, offset, size)
        }
        constants::ReservedFile::PermissionsJson => {
            (user_permissions::ITEM.read)(fs, inode, reply, offset, size)
        }
        constants::ReservedFile::RolesJson => {
            (user_roles::ITEM.read)(fs, inode, reply, offset, size);
        }
        _ => {
            reply.error(ENOENT);
        }

    }
}

pub fn readdir(fs: &mut GoodDataFS,
               req: &Request,
               ino: u64,
               fh: u64,
               in_offset: u64,
               mut reply: ReplyDirectory) {
    let mut offset = in_offset;

    let inode = inode::Inode::deserialize(ino);
    match inode.category {
        x if x == constants::Category::Internal as u8 => {
            let projectid = inode.project - 1;

            // Iterate over all project::ITEMS
            if offset + 1 < PROJECT_ITEMS.len() as u64 {
                for item in PROJECT_ITEMS.into_iter().skip(offset as usize) {
                    item.readdir(projectid, &offset, &mut reply);
                    offset += 1;
                }
            }

            reply.ok();
        }
        x if x == constants::Category::Ldm as u8 => {
            project::ldm::readdir(fs, req, ino, fh, in_offset, reply)
        }
        x if x == constants::Category::Metadata as u8 => {
            project::metadata::readdir(fs, req, ino, fh, in_offset, reply)
        }
        x if x == constants::Category::MetadataReports as u8 => {
            project::metadata::reports::readdir(fs, req, ino, fh, in_offset, reply)
        }
        _ => {
            println!("fs::project::readdir() - Unknow Category {}",
                     inode.category);
        }
    }
}
