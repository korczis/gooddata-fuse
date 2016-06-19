use rustc_serialize;
use rustc_serialize::json;

mod associated_permissions;
mod associated_roles;
mod feature_flags;
mod project_content;
mod project_meta;
mod create;

use std::collections::HashMap;
use gd::connector::Connector;

pub use self::associated_permissions::*;
pub use self::associated_roles::*;
pub use self::feature_flags::*;
pub use self::project_content::*;
pub use self::project_meta::*;
pub use self::create::*;

pub use super::metadata::*;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ProjectBody {
    pub content: ProjectContent,
    pub links: Option<HashMap<String, String>>,
    pub meta: ProjectMeta,
}

impl ProjectBody {
    pub fn links(&self) -> &Option<HashMap<String, String>> {
        &self.links
    }

    pub fn meta(&self) -> &ProjectMeta {
        &self.meta
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Project {
    pub project: ProjectBody,
}

impl Project {
    pub fn pid(&self) -> String {
        self.get_link("self").split("/").last().unwrap().to_string()
    }

    pub fn project(&self) -> &ProjectBody {
        &self.project
    }

    pub fn get_link(&self, link_name: &str) -> String {
        self.project().links().as_ref().unwrap()[link_name].to_string()
    }

    pub fn feature_flags(&self, client: &mut Connector) -> Option<FeatureFlags> {
        client.object_by_get::<FeatureFlags>(self.get_link("projectFeatureFlags"))
    }

    pub fn user_permissions(&self, client: &mut Connector) -> Option<AssociatedPermissions> {
        client.object_by_get::<AssociatedPermissions>(self.get_link("userPermissions"))
    }

    pub fn user_roles(&self, client: &mut Connector) -> Option<AssociatedRoles> {
        client.object_by_get::<AssociatedRoles>(self.get_link("userRoles"))
    }

    pub fn fetch_metadata<T: rustc_serialize::Decodable>(&self,
                                                         client: &mut Connector,
                                                         md_type: String,
                                                         force_update: bool)
                                                         -> T {
        let uri = format!("/gdc/md/{}/objects/query?category={}&limit=50",
                          self.pid(),
                          md_type);
        let res = client.get_cached(uri, force_update);

        json::decode::<T>(&res).unwrap()
    }

    pub fn get_metadata_objects<T: rustc_serialize::Decodable>(&self,
                                                               client: &mut Connector,
                                                               name: String,
                                                               force_update: bool)
                                                               -> &MetadataObjectsBody<T> {
        self.fetch_metadata::<MetadataObjects<MetadataObjectsBody<T>>>(client, name, force_update)
            .objects()
    }

    pub fn get_metadata_items<T: rustc_serialize::Decodable>(&self,
                                                             client: &mut Connector,
                                                             name: String,
                                                             force_update: bool)
                                                             -> &Vec<T> {
        self.get_metadata_objects::<T>(client, name, force_update).items()
    }

    pub fn facts(&self, client: &mut Connector, force_update: bool) -> &Vec<Fact> {
        self.get_metadata_items::<Fact>(client, "fact".to_string(), force_update)
    }

    pub fn metrics(&self, client: &mut Connector, force_update: bool) -> &Vec<Metric> {
        self.get_metadata_items::<Metric>(client, "metric".to_string(), force_update)
    }

    pub fn reports(&self, client: &mut Connector, force_update: bool) -> &Vec<Report> {
        self.get_metadata_items::<Report>(client, "report".to_string(), force_update)
    }
}
