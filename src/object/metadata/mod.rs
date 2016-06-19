// use rustc_serialize;
// use rustc_serialize::json;

pub mod attribute;
pub mod fact;
pub mod metric;
pub mod report;
pub mod report_definition;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetadataMeta {
    pub author: Option<String>,
    pub uri: Option<String>,
    pub tags: Option<String>,
    pub created: Option<String>,
    pub identifier: Option<String>,
    pub deprecated: Option<String>,
    pub summary: Option<String>,
    pub isProduction: Option<u8>,
    pub title: Option<String>,
    pub category: Option<String>,
    pub updated: Option<String>,
    pub contributor: Option<String>,
}

impl MetadataMeta {
    pub fn identifier(&self) -> &Option<String> {
        &self.identifier
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetadataPaging {
    pub next: Option<String>,
    pub count: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetadataObjectBody<T> {
    pub content: T,
    pub meta: super::MetadataMeta,
}

impl<T> MetadataObjectBody<T> {
    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn meta(&self) -> &super::MetadataMeta {
        &self.meta
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetadataObjectsBody<T> {
    pub paging: super::MetadataPaging,
    pub items: Vec<T>,
}

impl<T> super::MetadataObjectsBody<T> {
    pub fn items(&self) -> &Vec<T> {
        &self.items
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetadataObjects<T> {
    pub objects: T,
}

impl<T> super::MetadataObjects<T> {
    pub fn objects(&self) -> &T {
        &self.objects
    }

    // pub fn find_by_identifier2(&self, identifier: &String) -> (u32, Option<T>) {
    //     let mut i: u32 = 0;
    //     for item in self.objects().items().into_iter() {
    //         if item.object().meta().identifier().as_ref().unwrap() == identifier {
    //             return (i, Some(item.clone()));
    //         }
    //
    //         i += 1;
    //     }
    //
    //     (0, None)
    // }
}

trait MetadataObject<T: Into<String>> {
    fn object(&self) -> &T;
}

pub trait MetadataObjectRootKey {
    fn root_key() -> String;
}
