use fuse::{Request, ReplyData};
use libc::ENOENT;
use rustc_serialize::json;

use fs::constants;
use fs::GoodDataFS;
use fs::inode;
use gd;
use object;

pub fn read(fs: &mut GoodDataFS,
            _req: &Request,
            ino: u64,
            fh: u64,
            offset: u64,
            size: u32,
            reply: ReplyData) {
    let inode = inode::Inode::deserialize(ino);
    println!("GoodDataFS::read() - Reading inode {:?}, fh {}, offset {}, size {}",
             inode,
             fh,
             offset,
             size);

    match ino {
        _ => {
            if inode.project > 0 {
                let reserved = constants::ReservedFile::from(inode.reserved);
                match reserved {
                    constants::ReservedFile::FeatureFlagsJson => {
                        feature_flags_json(fs, inode, reply, offset as usize)
                    }
                    constants::ReservedFile::ProjectJson => {
                        project_json(fs, inode, reply, offset as usize)
                    }
                    constants::ReservedFile::PermissionsJson => {
                        permissions_json(fs, inode, reply, offset as usize)
                    }
                    constants::ReservedFile::RolesJson => {
                        roles_json(fs, inode, reply, offset as usize);
                    }
                    _ => {
                        reply.error(ENOENT);
                    }

                }
            } else {
                reply.error(ENOENT);
            }
        }
    }
}

fn feature_flags_json(fs: &mut GoodDataFS, inode: inode::Inode, reply: ReplyData, offset: usize) {
    println!("GoodDataFS::read() - Reading {}",
             constants::FEATURE_FLAGS_JSON_FILENAME);

    let pid = (inode.project - 1) as usize;
    let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();
    let feature_flags = project.feature_flags(&mut fs.client);
    if feature_flags.is_some() {
        let json: String = feature_flags.unwrap().into();
        reply.data(&json.as_bytes()[offset as usize..]);
    }
}

fn project_json(fs: &mut GoodDataFS, inode: inode::Inode, reply: ReplyData, offset: usize) {
    println!("GoodDataFS::read() - Reading {}",
             constants::PROJECT_JSON_FILENAME);

    let client: &gd::GoodDataClient = fs.client();
    let projects = client.projects().as_ref();
    let json = json::as_pretty_json(&projects.unwrap()[(inode.project - 1) as usize]).to_string();
    reply.data(&json.as_bytes()[offset as usize..]);
}

fn permissions_json(fs: &mut GoodDataFS, inode: inode::Inode, reply: ReplyData, offset: usize) {
    println!("GoodDataFS::read() - Reading {}",
             constants::USER_PERMISSIONS_JSON_FILENAME);

    let pid = (inode.project - 1) as usize;
    let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();
    let json: String = project.user_permissions(&mut fs.client).into();
    reply.data(&json.as_bytes()[offset as usize..]);
}

fn roles_json(fs: &mut GoodDataFS, inode: inode::Inode, reply: ReplyData, offset: usize) {
    println!("GoodDataFS::read() - Reading {}",
             constants::USER_ROLES_JSON_FILENAME);

    let pid = (inode.project - 1) as usize;
    let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();
    let json: String = project.user_roles(&mut fs.client).into();
    reply.data(&json.as_bytes()[offset as usize..]);
}
