
use fuse::{FileType, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory, Request};
use libc::ENOENT;
use rustc_serialize::json;
use std::path::Path;

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::{create_inode_directory_attributes, create_inode_file_attributes};
use gd;
use object;
use helpers;

use super::item;

use super::super::inode;

pub const FEATURE_FLAGS_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::FeatureFlagsJson as u8,
    item_type: FileType::RegularFile,
    path: constants::FEATURE_FLAGS_JSON_FILENAME,
};

pub const PROJECT_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::ProjectJson as u8,
    item_type: FileType::RegularFile,
    path: constants::PROJECT_JSON_FILENAME,
};

pub const PERMISSIONS_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::PermissionsJson as u8,
    item_type: FileType::RegularFile,
    path: constants::USER_PERMISSIONS_JSON_FILENAME,
};

pub const USER_ROLES_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Ldm as u8,
    reserved: constants::ReservedFile::RolesJson as u8,
    item_type: FileType::RegularFile,
    path: constants::USER_ROLES_JSON_FILENAME,
};

pub const PROJECT_FILES: [item::ProjectItem; 4] =
    [FEATURE_FLAGS_JSON, PROJECT_JSON, PERMISSIONS_JSON, USER_ROLES_JSON];

pub const LDM_DIR: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_LDM_DIR,
};

pub const METADATA_DIR: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Metadata as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_METADATA_DIR,
};

pub const PROJECT_DIRS: [item::ProjectItem; 2] = [LDM_DIR, METADATA_DIR];

pub static PROJECT_ITEMS: [item::ProjectItem; 6] =
    [FEATURE_FLAGS_JSON, PROJECT_JSON, PERMISSIONS_JSON, USER_ROLES_JSON, LDM_DIR, METADATA_DIR];

/// Gets project from inode
pub fn project_from_inode<Type: Into<inode::Inode>>(fs: &GoodDataFS, ino: Type) -> object::Project {
    let inode = ino.into();
    let pid = (inode.project - 1) as usize;

    fs.client().projects().as_ref().unwrap()[pid].clone()
}

fn project_feature_flags_json(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let project: &object::Project = &project_from_inode(fs, ino);

    let feature_flags = project.feature_flags(&mut fs.client.connector);
    if feature_flags.is_some() {
        let json: String = feature_flags.unwrap().into();

        let attr =
            create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.attr(&constants::DEFAULT_TTL, &attr);
    }
}

fn project_project_json(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let project: &object::Project = &project_from_inode(fs, ino);
    let json = json::as_pretty_json(project).to_string();

    let attr = create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
    reply.attr(&constants::DEFAULT_TTL, &attr);
}

fn project_permissions_json(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let project: &object::Project = &project_from_inode(fs, ino);

    let user_permissions = project.user_permissions(&mut fs.client.connector);
    if user_permissions.is_some() {
        let json: String = user_permissions.unwrap().into();

        let attr =
            create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.attr(&constants::DEFAULT_TTL, &attr);
    }
}

fn project_roles_json(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let project: &object::Project = &project_from_inode(fs, ino);

    let user_roles = project.user_roles(&mut fs.client.connector);

    if user_roles.is_some() {
        let json: String = user_roles.unwrap().into();
        let attr =
            create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.attr(&constants::DEFAULT_TTL, &attr);
    }
}

pub fn getattr(fs: &mut GoodDataFS, req: &Request, ino: u64, reply: ReplyAttr) {
    let inode = inode::Inode::deserialize(ino);
    println!("fs::project::getattr() {} - {:?}", ino, inode);

    if inode.project > 0 {
        if inode.category == constants::Category::Internal as u8 {
            let reserved = constants::ReservedFile::from(inode.reserved);
            match reserved {
                constants::ReservedFile::FeatureFlagsJson => {
                    project_feature_flags_json(fs, req, ino, reply)
                }
                constants::ReservedFile::ProjectJson => project_project_json(fs, req, ino, reply),
                constants::ReservedFile::PermissionsJson => {
                    project_permissions_json(fs, req, ino, reply)
                }
                constants::ReservedFile::RolesJson => project_roles_json(fs, req, ino, reply),
                _ => reply.error(ENOENT),
            }
        } else if inode.category == constants::Category::Ldm as u8 {
            let attr = create_inode_directory_attributes(ino);
            reply.attr(&constants::DEFAULT_TTL, &attr);
        } else if inode.category == constants::Category::Metadata as u8 {
            let attr = create_inode_directory_attributes(ino);
            reply.attr(&constants::DEFAULT_TTL, &attr);
        } else if inode.category == constants::Category::MetadataAttributes as u8 {
            let attr = create_inode_directory_attributes(ino);
            reply.attr(&constants::DEFAULT_TTL, &attr);
        } else if inode.category == constants::Category::MetadataFacts as u8 {
            let attr = create_inode_directory_attributes(ino);
            reply.attr(&constants::DEFAULT_TTL, &attr);
        } else if inode.category == constants::Category::MetadataMetrics as u8 {
            let attr = create_inode_directory_attributes(ino);
            reply.attr(&constants::DEFAULT_TTL, &attr);
        } else if inode.category == constants::Category::MetadataReports as u8 {
            if inode.reserved == constants::ReservedFile::KeepMe as u8 {
                let attr = create_inode_directory_attributes(ino);
                reply.attr(&constants::DEFAULT_TTL, &attr);
            } else if inode.reserved == 0 {
                // JSON REPORT
                let project: &object::Project = &project_from_inode(fs, ino);

                let report =
                    &project.reports(&mut fs.client.connector).objects.items[inode.item as usize];

                let json: String = report.clone().into();
                let attr = create_inode_file_attributes(ino,
                                                        json.len() as u64,
                                                        constants::DEFAULT_CREATE_TIME);
                reply.attr(&constants::DEFAULT_TTL, &attr);
                println!("Getting attributes {:?}", inode);
            } else {
                println!("N A S T A L   H A P R ! ! !");
                reply.error(ENOENT);
            }
        } else if inode.category == constants::Category::MetadataReportDefinition as u8 {
            let attr = create_inode_directory_attributes(ino);
            reply.attr(&constants::DEFAULT_TTL, &attr);
        } else {
            println!("fs::project::getattr() - not found!");
        }
    } else {
        println!("GoodDataFS::getattr() - Not found inode {:?}", ino);
        reply.error(ENOENT);
    }
}

fn feature_flags_json(fs: &mut GoodDataFS, inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::create(inode_parent.project,
                                     constants::Category::Internal as u8,
                                     0,
                                     constants::ReservedFile::FeatureFlagsJson as u8);
    let project: &object::Project = &project_from_inode(fs, *inode_parent);

    let feature_flags = project.feature_flags(&mut fs.client.connector);
    if feature_flags.is_some() {
        let json: String = feature_flags.unwrap().into();
        let attr =
            create_inode_file_attributes(inode, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.entry(&constants::DEFAULT_TTL, &attr, 0);
    }
}

fn project_json(fs: &mut GoodDataFS, inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::Internal as u8,
        item: 0,
        reserved: constants::ReservedFile::ProjectJson as u8,
    });

    let client: &gd::GoodDataClient = fs.client();
    let projects = client.projects().as_ref();
    let json = json::as_pretty_json(&projects.unwrap()[(inode_parent.project - 1) as usize])
        .to_string();
    let attr =
        create_inode_file_attributes(inode, json.len() as u64, constants::DEFAULT_CREATE_TIME);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn project_ldm_dir(inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::Ldm as u8,
        item: 0,
        reserved: constants::ReservedFile::KeepMe as u8,
    });

    let attr = create_inode_directory_attributes(inode);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn project_metadata_dir(inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::Metadata as u8,
        item: 0,
        reserved: constants::ReservedFile::KeepMe as u8,
    });

    let attr = create_inode_directory_attributes(inode);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}


fn project_metadata_attributes_dir(inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::MetadataAttributes as u8,
        item: 0,
        reserved: constants::ReservedFile::KeepMe as u8,
    });

    let attr = create_inode_directory_attributes(inode);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn project_metadata_facts_dir(inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::MetadataFacts as u8,
        item: 0,
        reserved: constants::ReservedFile::KeepMe as u8,
    });

    let attr = create_inode_directory_attributes(inode);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn project_metadata_metrics_dir(inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::MetadataMetrics as u8,
        item: 0,
        reserved: constants::ReservedFile::KeepMe as u8,
    });

    let attr = create_inode_directory_attributes(inode);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn project_metadata_reports_dir(inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::MetadataReports as u8,
        item: 0,
        reserved: constants::ReservedFile::KeepMe as u8,
    });

    let attr = create_inode_directory_attributes(inode);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn project_metadata_report_definitions_dir(inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::MetadataReportDefinition as u8,
        item: 0,
        reserved: constants::ReservedFile::KeepMe as u8,
    });

    let attr = create_inode_directory_attributes(inode);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn permissions_json(fs: &mut GoodDataFS, inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::Internal as u8,
        item: 0,
        reserved: constants::ReservedFile::PermissionsJson as u8,
    });

    let project: &object::Project = &project_from_inode(fs, *inode_parent);
    let user_permissions = project.user_permissions(&mut fs.client.connector);

    if user_permissions.is_some() {
        let json: String = user_permissions.unwrap().into();
        let attr =
            create_inode_file_attributes(inode, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.entry(&constants::DEFAULT_TTL, &attr, 0);
    }
}

fn roles_json(fs: &mut GoodDataFS, inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: constants::Category::Internal as u8,
        item: 0,
        reserved: constants::ReservedFile::RolesJson as u8,
    });

    let project: &object::Project = &project_from_inode(fs, *inode_parent);
    let user_roles = project.user_roles(&mut fs.client.connector);

    if user_roles.is_some() {
        let json: String = user_roles.unwrap().into();
        let attr =
            create_inode_file_attributes(inode, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.entry(&constants::DEFAULT_TTL, &attr, 0);
    }
}

pub fn lookup(fs: &mut GoodDataFS, _req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
    let inode = inode::Inode::deserialize(parent);

    match name.to_str() {
        Some(constants::FEATURE_FLAGS_JSON_FILENAME) => feature_flags_json(fs, &inode, reply),
        Some(constants::PROJECT_JSON_FILENAME) => project_json(fs, &inode, reply),
        Some(constants::PROJECT_LDM_DIR) => project_ldm_dir(&inode, reply),
        Some(constants::PROJECT_METADATA_DIR) => project_metadata_dir(&inode, reply),
        Some(constants::PROJECT_METADATA_ATTRIBUTES_DIR) => {
            project_metadata_attributes_dir(&inode, reply)
        }
        Some(constants::PROJECT_METADATA_FACTS_DIR) => project_metadata_facts_dir(&inode, reply),
        Some(constants::PROJECT_METADATA_METRICS_DIR) => {
            project_metadata_metrics_dir(&inode, reply)
        }
        Some(constants::PROJECT_METADATA_REPORTS_DIR) => {
            project_metadata_reports_dir(&inode, reply)
        }
        Some(constants::PROJECT_METADATA_REPORT_DEFINITIONS_DIR) => {
            project_metadata_report_definitions_dir(&inode, reply)
        }
        Some(constants::USER_PERMISSIONS_JSON_FILENAME) => permissions_json(fs, &inode, reply),
        Some(constants::USER_ROLES_JSON_FILENAME) => roles_json(fs, &inode, reply),
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

fn read_feature_flags_json(fs: &mut GoodDataFS,
                           inode: inode::Inode,
                           reply: ReplyData,
                           offset: u64,
                           size: u32) {
    println!("GoodDataFS::read() - Reading {}",
             constants::FEATURE_FLAGS_JSON_FILENAME);

    let project: &object::Project = &project_from_inode(fs, inode);

    let feature_flags = project.feature_flags(&mut fs.client.connector);
    if feature_flags.is_some() {
        let json: String = feature_flags.unwrap().into();
        // reply.data(&json.as_bytes()[offset as usize..]);
        reply.data(helpers::read_bytes(&json, offset, size));
    }
}

fn read_project_json(fs: &mut GoodDataFS,
                     inode: inode::Inode,
                     reply: ReplyData,
                     offset: u64,
                     size: u32) {
    println!("GoodDataFS::read() - Reading {}",
             constants::PROJECT_JSON_FILENAME);

    let client: &gd::GoodDataClient = fs.client();
    let projects = client.projects().as_ref();
    let json = json::as_pretty_json(&projects.unwrap()[(inode.project - 1) as usize]).to_string();
    // reply.data(&json.as_bytes()[offset as usize..]);
    reply.data(helpers::read_bytes(&json, offset, size));
}

fn read_permissions_json(fs: &mut GoodDataFS,
                         inode: inode::Inode,
                         reply: ReplyData,
                         offset: u64,
                         size: u32) {
    println!("GoodDataFS::read() - Reading {}",
             constants::USER_PERMISSIONS_JSON_FILENAME);

    let project: &object::Project = &project_from_inode(fs, inode);

    let user_permissions = project.user_permissions(&mut fs.client.connector);
    if user_permissions.is_some() {
        let json: String = user_permissions.unwrap().into();
        // reply.data(&json.as_bytes()[offset as usize..]);
        reply.data(helpers::read_bytes(&json, offset, size));
    }
}

fn read_roles_json(fs: &mut GoodDataFS,
                   inode: inode::Inode,
                   reply: ReplyData,
                   offset: u64,
                   size: u32) {
    println!("GoodDataFS::read() - Reading {}",
             constants::USER_ROLES_JSON_FILENAME);

    let project: &object::Project = &project_from_inode(fs, inode);

    let user_roles = project.user_roles(&mut fs.client.connector);
    if user_roles.is_some() {
        let json: String = user_roles.unwrap().into();
        // reply.data(&json.as_bytes()[offset as usize..]);
        reply.data(helpers::read_bytes(&json, offset, size));
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
            read_feature_flags_json(fs, inode, reply, offset, size)
        }
        constants::ReservedFile::ProjectJson => read_project_json(fs, inode, reply, offset, size),
        constants::ReservedFile::PermissionsJson => {
            read_permissions_json(fs, inode, reply, offset, size)
        }
        constants::ReservedFile::RolesJson => {
            read_roles_json(fs, inode, reply, offset, size);
        }
        _ => {
            reply.error(ENOENT);
        }

    }
}

pub fn readdir(fs: &mut GoodDataFS,
               _req: &Request,
               ino: u64,
               _fh: u64,
               in_offset: u64,
               mut reply: ReplyDirectory) {
    let mut offset = in_offset;

    let inode = inode::Inode::deserialize(ino);
    match inode.category {
        x if x == constants::Category::Ldm as u8 => {
            reply.ok();
        }
        x if x == constants::Category::Metadata as u8 => {
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
        x if x == constants::Category::MetadataReports as u8 => {
            let project: &object::Project = &project_from_inode(fs, ino);
            let report_items = project.reports(&mut fs.client.connector);

            if offset == 0 {
                for item in report_items.objects.items.into_iter() {
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

                reply.ok();
            }
        }
        _ => {
            let projectid = inode.project - 1;

            // Iterate over all project::ITEMS
            if offset == 0 {
                if inode.category == constants::Category::Internal as u8 {
                    for item in PROJECT_ITEMS.into_iter().skip(offset as usize) {
                        item.readdir(projectid, &offset, &mut reply);
                        offset += 1;
                    }
                }
            }

            reply.ok();
        }
    }
}
